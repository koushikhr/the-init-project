mod style;
mod views {
    pub mod card;
}

use std::collections::{BTreeMap, HashMap};
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

// A batch of packages to be installed by a specific manager
type Batch = (Arc<Box<dyn PackageManager>>, Vec<String>);

struct InitApp {
    screen: Screen,
    apps: Vec<App>,
    grouped_apps: BTreeMap<String, Vec<usize>>,
    selected_apps: Vec<String>,
    loading: bool,
    detected_os: String,

    // Store multiple detected managers
    managers: Vec<Arc<Box<dyn PackageManager>>>,

    // Queue of batches to install
    install_queue: Vec<Batch>,
    // Current batch being installed (Manager Name, Package Count)
    current_install: Option<(String, usize)>,
    install_log: Vec<String>,
    progress: f32,
    total_tasks: usize,
    completed_tasks: usize,
}

#[derive(Debug, Clone, PartialEq)]
enum Screen {
    Welcome,
    Dashboard,
    Installing,
}

#[derive(Debug, Clone)]
enum Message {
    InitializationConfigured(Result<(Option<Vec<App>>, Vec<Arc<Box<dyn PackageManager>>>), String>),
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
                managers: vec![],
                install_queue: vec![],
                current_install: None,
                install_log: vec![],
                progress: 0.0,
                total_tasks: 0,
                completed_tasks: 0,
            },
            Task::perform(initialize_app(), Message::InitializationConfigured),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::InitializationConfigured(Ok((apps_opt, managers))) => {
                println!("✅ Detected {} Managers", managers.len());
                self.managers = managers;

                if let Some(apps) = apps_opt {
                    println!("✅ Loaded {} apps", apps.len());
                    self.apps = apps;
                    let mut groups: BTreeMap<String, Vec<usize>> = BTreeMap::new();
                    for (index, app) in self.apps.iter().enumerate() {
                        let cat = app.category.clone().unwrap_or("General".to_string());
                        groups.entry(cat).or_default().push(index);
                    }
                    self.grouped_apps = groups;
                } else {
                    println!("⚠️ No apps loaded (Manifest error/missing).");
                }

                self.loading = false;
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
                self.install_log.clear();
                self.progress = 0.0;

                // Group selected apps by manager
                let mut batches: HashMap<usize, Vec<String>> = HashMap::new(); // Manager Index -> Package IDs

                for app_id in &self.selected_apps {
                    if let Some(app) = self.apps.iter().find(|a| a.id == *app_id) {
                        // Find the first manager that supports this app
                        for (idx, mgr) in self.managers.iter().enumerate() {
                            if let Some(pkg_id) = app.packages.get(mgr.id()) {
                                batches.entry(idx).or_default().push(pkg_id.clone());
                                break;
                            }
                        }
                    }
                }

                // Convert to Queue
                self.install_queue = batches
                    .into_iter()
                    .map(|(idx, pkgs)| (self.managers[idx].clone(), pkgs))
                    .collect();

                self.total_tasks = self.install_queue.len();
                self.completed_tasks = 0;

                if self.install_queue.is_empty() {
                    self.install_log
                        .push("No compatible packages found for selected apps.".to_string());
                    self.progress = 1.0;
                    Task::none()
                } else {
                    Task::perform(async {}, |_| Message::InstallNext)
                }
            }

            Message::InstallNext => {
                if let Some((mgr, pkgs)) = self.install_queue.pop() {
                    let mgr_name = mgr.name().to_string();
                    let pkg_count = pkgs.len();
                    self.current_install = Some((mgr_name.clone(), pkg_count));

                    self.install_log.push(format!(
                        "🚀 Starting batch install via {} ({} packages)...",
                        mgr_name, pkg_count
                    ));

                    return Task::perform(
                        async move {
                            let pkg_refs: Vec<&str> = pkgs.iter().map(|s| s.as_str()).collect();
                            let result = mgr.install_many(&pkg_refs).await;

                            match result {
                                Ok(_) => (format!("Batch via {}", mgr.name()), true),
                                Err(e) => {
                                    (format!("Batch via {} Failed: {}", mgr.name(), e), false)
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

            Message::InstallFinished(msg, success) => {
                if success {
                    self.install_log
                        .push(format!("✅ {} completed successfully.", msg));
                } else {
                    self.install_log.push(format!("❌ {}", msg));
                }

                self.completed_tasks += 1;

                if self.total_tasks > 0 {
                    self.progress = self.completed_tasks as f32 / self.total_tasks as f32;
                } else {
                    self.progress = 1.0;
                }

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
        let mgr_names = if self.managers.is_empty() {
            "None Detected".to_string()
        } else {
            self.managers
                .iter()
                .map(|m| m.name())
                .collect::<Vec<_>>()
                .join(", ")
        };

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
            text("Detected Managers:").color(style::TEXT_SECONDARY),
            text(mgr_names).color(style::SUCCESS)
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

                    // Check if ANY active manager can install this app
                    let is_avail = self
                        .managers
                        .iter()
                        .any(|m| app.packages.contains_key(m.id()));

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

async fn initialize_app() -> Result<(Option<Vec<App>>, Vec<Arc<Box<dyn PackageManager>>>), String> {
    // 1. Detect Managers FIRST

    let managers = detectors::detect_managers();

    let managers_arc: Vec<Arc<Box<dyn PackageManager>>> =
        managers.into_iter().map(Arc::new).collect();

    // 2. Find apps.toml (Check existence!)

    let mut config_path = None;

    // Check relative to executable (Distribution mode)

    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(parent) = exe_path.parent() {
            let path = parent.join("apps.toml");

            if path.exists() {
                config_path = Some(path);
            }
        }
    }

    // Fallback: Check current working directory (Dev mode / cargo run)

    if config_path.is_none() {
        if let Ok(cwd) = std::env::current_dir() {
            let path = cwd.join("apps.toml");

            if path.exists() {
                config_path = Some(path);
            }
        }
    }

    let path_res =
        config_path.ok_or("Could not locate apps.toml in Executable dir or CWD".to_string());

    match path_res {
        Ok(path) => {
            match manifest::load_manifest(path.to_str().unwrap()).await {
                Ok(manifest) => Ok((Some(manifest.apps), managers_arc)),

                Err(e) => {
                    println!("Manifest Load Error: {} ({})", e, path.display());

                    // Return managers even if manifest fails

                    Ok((None, managers_arc))
                }
            }
        }

        Err(e) => {
            println!("Manifest Path Error: {}", e);

            // Return managers even if manifest path not found

            Ok((None, managers_arc))
        }
    }
}
