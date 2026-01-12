mod style;
mod views {
    pub mod card;
}

use std::collections::BTreeMap;
use std::sync::Arc; // REQUIRED for passing Traits in Messages

use iced::{
    Alignment, Color, Element, Length, Subscription, Task, Theme,
    widget::{Space, button, column, container, progress_bar, row, scrollable, text},
};
use init_core::PackageManager;
use init_core::detectors;
use init_core::manifest::{self, App};

pub fn main() -> iced::Result {
    iced::application(InitApp::new, InitApp::update, InitApp::view)
        .title(|_state: &InitApp| "The Init Project".to_string())
        .theme(|_state: &InitApp| Theme::Dark)
        .subscription(InitApp::subscription)
        .run()
}

struct InitApp {
    screen: Screen,
    apps: Vec<App>,
    grouped_apps: BTreeMap<String, Vec<usize>>,
    selected_apps: Vec<String>,
    loading: bool,
    detected_os: String,

    // FIX: Store as Arc so we can share it with async tasks
    manager: Option<Arc<Box<dyn PackageManager>>>,

    install_queue: Vec<String>,
    current_install: Option<String>,
    install_log: Vec<String>,
    progress: f32,
}

#[derive(Debug, Clone, PartialEq)]
enum Screen {
    Welcome,
    Dashboard,
    Installing,
}

#[derive(Debug, Clone)]
enum Message {
    // FIX: The Result now contains Arc<Box<...>> to satisfy Clone/Debug
    InitializationConfigured(Result<(Vec<App>, Arc<Box<dyn PackageManager>>), String>),
    Toggle(String, bool),
    ProceedToDashboard,
    StartInstall,
    InstallNext,
    InstallFinished(String, bool),
}

impl InitApp {
    fn new() -> (Self, Task<Message>) {
        let os = std::env::consts::OS.to_string();

        (
            Self {
                screen: Screen::Welcome,
                apps: vec![],
                grouped_apps: BTreeMap::new(),
                selected_apps: vec![],
                loading: true,
                detected_os: os,
                manager: None,
                install_queue: vec![],
                current_install: None,
                install_log: vec![],
                progress: 0.0,
            },
            Task::perform(initialize_app(), Message::InitializationConfigured),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::InitializationConfigured(Ok((apps, manager))) => {
                println!("✅ Loaded {} apps", apps.len());
                println!("✅ Detected Manager: {}", manager.name());

                self.loading = false;
                self.apps = apps;
                self.manager = Some(manager); // Already an Arc

                let mut groups: BTreeMap<String, Vec<usize>> = BTreeMap::new();
                for (index, app) in self.apps.iter().enumerate() {
                    let cat = app.category.clone().unwrap_or("General".to_string());
                    groups.entry(cat).or_default().push(index);
                }
                self.grouped_apps = groups;
                Task::none()
            }
            Message::InitializationConfigured(Err(e)) => {
                println!("❌ ERROR: {}", e);
                self.loading = false;
                Task::none()
            }

            Message::Toggle(id, true) => {
                self.selected_apps.push(id);
                Task::none()
            }
            Message::Toggle(id, false) => {
                self.selected_apps.retain(|x| x != &id);
                Task::none()
            }

            Message::ProceedToDashboard => {
                self.screen = Screen::Dashboard;
                Task::none()
            }

            Message::StartInstall => {
                self.screen = Screen::Installing;
                self.install_queue = self.selected_apps.clone();
                self.install_log.clear();
                self.progress = 0.0;
                Task::perform(async {}, |_| Message::InstallNext)
            }

            Message::InstallNext => {
                if let Some(app_id) = self.install_queue.pop() {
                    self.current_install = Some(app_id.clone());
                    self.install_log
                        .push(format!("Starting installation: {}...", app_id));

                    let app = self.apps.iter().find(|a| a.id == app_id).cloned();

                    // FIX: Clone the Arc (cheap) to pass to async task
                    let manager_arc = self.manager.as_ref().unwrap().clone();

                    return Task::perform(
                        async move {
                            let Some(target_app) = app else {
                                return (app_id, false);
                            };
                            let mgr_id = manager_arc.id();

                            let Some(pkg_name) = target_app.packages.get(mgr_id) else {
                                println!("❌ No package ID found for manager '{}'", mgr_id);
                                return (app_id, false);
                            };

                            println!("🚀 Calling Manager {} to install {}", mgr_id, pkg_name);

                            // Call the trait method directly!
                            let result = manager_arc.install(pkg_name).await;

                            match result {
                                Ok(_) => (app_id, true),
                                Err(e) => {
                                    println!("❌ Install Failed: {}", e);
                                    (app_id, false)
                                }
                            }
                        },
                        |(id, success)| Message::InstallFinished(id, success),
                    );
                }

                self.current_install = None;
                self.install_log.push("All tasks completed!".to_string());
                self.progress = 1.0;
                Task::none()
            }

            Message::InstallFinished(id, success) => {
                if success {
                    self.install_log
                        .push(format!("✅ {} installed successfully.", id));
                } else {
                    self.install_log
                        .push(format!("❌ {} failed to install.", id));
                }

                let total = self.selected_apps.len() as f32;
                let remaining = self.install_queue.len() as f32;
                self.progress = (total - remaining) / total;

                Task::perform(async {}, |_| Message::InstallNext)
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }

    fn view(&self) -> Element<'_, Message> {
        if self.loading {
            return container(
                text("Initializing Core...")
                    .color(style::TEXT_PRIMARY)
                    .size(20),
            )
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(style::main_container)
            .into();
        }
        match self.screen {
            Screen::Welcome => self.view_welcome(),
            Screen::Dashboard => self.view_dashboard(),
            Screen::Installing => self.view_installing(),
        }
    }

    fn view_welcome(&self) -> Element<'_, Message> {
        let mgr_name = self
            .manager
            .as_ref()
            .map(|m| m.name())
            .unwrap_or("Scanning...");

        let title = text("Welcome to The Init Project")
            .size(32)
            .font(iced::Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            })
            .color(style::TEXT_PRIMARY);

        let sub_title = text("Your universal bootstrap solution.")
            .size(16)
            .color(style::TEXT_SECONDARY);

        let os_info = row![
            text("Detected OS:").color(style::TEXT_SECONDARY),
            text(self.detected_os.to_uppercase())
                .color(style::ACCENT)
                .font(iced::Font::MONOSPACE)
        ]
        .spacing(10);

        let manager_info = row![
            text("Primary Manager:").color(style::TEXT_SECONDARY),
            text(mgr_name).color(style::SUCCESS)
        ]
        .spacing(10);

        let system_card = container(column![
            text("System Environment")
                .size(14)
                .color(style::TEXT_SECONDARY),
            Space::new().height(10.0),
            os_info,
            Space::new().height(10.0),
            manager_info
        ])
        .padding(20)
        .width(Length::Fill)
        .style(|_t| container::Style {
            background: Some(iced::Background::Color(Color::from_rgb(0.1, 0.1, 0.14))),
            border: iced::Border {
                radius: 8.0.into(),
                ..Default::default()
            },
            ..Default::default()
        });

        let modal_content = column![
            title,
            sub_title,
            Space::new().height(30.0),
            system_card,
            Space::new().height(30.0),
            button("Proceed to Dashboard")
                .padding([12, 24])
                .style(style::primary_button)
                .on_press(Message::ProceedToDashboard)
                .width(Length::Fill)
        ]
        .spacing(10)
        .width(400)
        .align_x(Alignment::Center);

        container(
            container(modal_content)
                .padding(40)
                .style(style::welcome_modal),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(style::main_container)
        .into()
    }

    fn view_dashboard(&self) -> Element<'_, Message> {
        let header = row![
            column![
                text("Init Dashboard").size(24).color(style::TEXT_PRIMARY),
                text("Select apps to install")
                    .size(14)
                    .color(style::TEXT_SECONDARY),
            ],
            Space::new().width(Length::Fill),
        ]
        .width(Length::Fill)
        .align_y(Alignment::Center);

        let mut main_content = column![header].spacing(30);

        for (category, app_indices) in &self.grouped_apps {
            let cat_title = text(category)
                .size(18)
                .color(style::ACCENT)
                .font(iced::Font {
                    weight: iced::font::Weight::Bold,
                    ..Default::default()
                });

            let mut grid = column![].spacing(20);

            for chunk in app_indices.chunks(4) {
                let mut row_widget = row![].spacing(20).height(Length::Fixed(180.0));

                for &app_idx in chunk {
                    let app = &self.apps[app_idx];
                    let is_selected = self.selected_apps.contains(&app.id);
                    let is_avail = !app.packages.is_empty();

                    row_widget = row_widget.push(views::card::view(
                        app,
                        is_selected,
                        is_avail,
                        Message::Toggle,
                    ));
                }
                grid = grid.push(row_widget);
            }
            main_content = main_content.push(column![cat_title, grid].spacing(15));
        }

        let bottom_bar = if !self.selected_apps.is_empty() {
            container(
                row![
                    text(format!("{} apps selected", self.selected_apps.len()))
                        .color(style::TEXT_PRIMARY),
                    Space::new().width(Length::Fill),
                    button("Install Selected")
                        .padding([10, 20])
                        .style(style::primary_button)
                        .on_press(Message::StartInstall)
                ]
                .align_y(Alignment::Center),
            )
            .padding(20)
            .style(|_t| container::Style {
                background: Some(iced::Background::Color(Color::from_rgb(0.08, 0.08, 0.1))),
                border: iced::Border {
                    color: style::ACCENT,
                    width: 1.0,
                    ..Default::default()
                },
                ..Default::default()
            })
        } else {
            container(Space::new())
        };

        column![
            container(scrollable(main_content).height(Length::Fill))
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(40)
                .style(style::main_container),
            bottom_bar
        ]
        .into()
    }

    fn view_installing(&self) -> Element<'_, Message> {
        let title = text("Installing Applications...")
            .size(24)
            .color(style::TEXT_PRIMARY);

        let bar = container(progress_bar(0.0..=1.0, self.progress).style(|_theme| {
            iced::widget::progress_bar::Style {
                background: iced::Background::Color(style::BG_CARD),
                bar: iced::Background::Color(style::ACCENT),
                border: iced::Border::default(),
            }
        }))
        .height(10)
        .width(Length::Fill);

        let log_view = scrollable(
            column(
                self.install_log
                    .iter()
                    .map(|l| text(l).color(style::TEXT_SECONDARY).into()),
            )
            .spacing(5),
        )
        .height(Length::Fill);

        let content = column![
            title,
            Space::new().height(20.0),
            bar,
            Space::new().height(20.0),
            container(log_view)
                .padding(20)
                .style(|_t| container::Style {
                    background: Some(iced::Background::Color(style::BG_CARD)),
                    border: iced::Border {
                        radius: 8.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .height(Length::Fill)
                .width(Length::Fill)
        ]
        .spacing(10)
        .padding(40);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(style::main_container)
            .into()
    }
}

// FIX: Helper returns Arc to satisfy Message requirements
async fn initialize_app() -> Result<(Vec<App>, Arc<Box<dyn PackageManager>>), String> {
    let current = std::env::current_dir().map_err(|e| e.to_string())?;
    let path = current.join("apps.toml");

    let manifest = manifest::load_manifest(path.to_str().unwrap())
        .await
        .map_err(|e| format!("Manifest Error: {}", e))?;

    let manager = detectors::get_system_manager();

    // Wrap the manager in Arc
    Ok((manifest.apps, Arc::new(manager)))
}
