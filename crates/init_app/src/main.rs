use iced::{Alignment, Background, Border, Color, Element, Length, Shadow, Task, Theme, Vector};
// Rename image to 'app_image' to differentiate from the module
use iced::widget::{
    button, column, container, image as app_image, row, scrollable, svg, text, tooltip,
};

use init_core::PackageManager;
use init_core::detectors;
use init_core::manifest::{self, App};

pub fn main() -> iced::Result {
    iced::application(InitApp::new, InitApp::update, InitApp::view)
        .title(|_state: &InitApp| "The Init Project".to_string())
        .theme(|_state: &InitApp| Theme::Dark)
        .run()
}

struct InitApp {
    apps: Vec<App>,
    selected_apps: Vec<String>,
    loading: bool,
    error: Option<String>,
    install_queue: Vec<String>,
    current_install: Option<String>,
    logs: Vec<String>,
    active_manager_id: String,
}

#[derive(Debug, Clone)]
enum Message {
    ManifestLoaded(Result<Vec<App>, String>),
    ToggleApp(String, bool),
    StartInstall,
    InstallNext,
    InstallFinished(String, Result<(), String>),
}

impl InitApp {
    fn new() -> (Self, Task<Message>) {
        let manager = detectors::get_system_manager();
        // If your core library trait doesn't implement .id() yet, use a fallback string here
        // e.g. let manager_id = "pacman".to_string();
        let manager_id = manager.id().to_string();

        (
            Self {
                apps: vec![],
                selected_apps: vec![],
                loading: true,
                error: None,
                install_queue: vec![],
                current_install: None,
                logs: vec![format!("Active Manager: {}", manager_id)],
                active_manager_id: manager_id,
            },
            Task::perform(load_apps(), Message::ManifestLoaded),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ManifestLoaded(result) => {
                self.loading = false;
                match result {
                    Ok(apps) => self.apps = apps,
                    Err(e) => self.error = Some(format!("Error: {}", e)),
                }
                Task::none()
            }
            Message::ToggleApp(id, is_checked) => {
                if is_checked {
                    self.selected_apps.push(id);
                } else {
                    self.selected_apps.retain(|x| x != &id);
                }
                Task::none()
            }
            Message::StartInstall => {
                if self.selected_apps.is_empty() {
                    return Task::none();
                }
                self.install_queue = self.selected_apps.clone();
                self.logs.push("Starting Queue...".to_string());
                Task::perform(async {}, |_| Message::InstallNext)
            }
            Message::InstallNext => {
                if let Some(app_id) = self.install_queue.pop() {
                    self.current_install = Some(app_id.clone());
                    let package_id = self
                        .apps
                        .iter()
                        .find(|a| a.id == app_id)
                        .and_then(|a| a.packages.get(&self.active_manager_id)) // Smart detection
                        .cloned();

                    match package_id {
                        Some(pkg) => {
                            self.logs.push(format!("Installing {}...", app_id));
                            Task::perform(install_app(app_id, pkg), |(id, res)| {
                                Message::InstallFinished(id, res)
                            })
                        }
                        None => {
                            self.logs.push(format!(
                                "Skipping {}: Not supported on {}",
                                app_id, self.active_manager_id
                            ));
                            Task::perform(async {}, |_| Message::InstallNext)
                        }
                    }
                } else {
                    self.current_install = None;
                    self.logs.push("Done.".to_string());
                    Task::none()
                }
            }
            Message::InstallFinished(id, result) => {
                match result {
                    Ok(_) => self.logs.push(format!("SUCCESS: {}", id)),
                    Err(e) => self.logs.push(format!("ERROR {}: {}", id, e)),
                }
                Task::perform(async {}, |_| Message::InstallNext)
            }
        }
    }

    // --- THE COMPLETED CARD VIEW ---
    fn view_app_card<'a>(&self, app: &'a App) -> Element<'a, Message> {
        let is_available = app.packages.contains_key(&self.active_manager_id);
        let is_selected = self.selected_apps.contains(&app.id);

        // 1. Determine Icon Path (Dynamic)
        let current_dir = std::env::current_dir().unwrap_or_default();
        let default_icon = current_dir.join("icons").join("default.svg");

        let icon_path = if let Some(path_str) = &app.icon {
            // If the path in toml is relative ("icons/firefox.svg"), join it with current dir
            current_dir.join(path_str).to_string_lossy().to_string()
        } else {
            default_icon.to_string_lossy().to_string()
        };

        // 2. Build the Icon Widget (SVG vs PNG)
        let icon_widget: Element<'a, Message> = if icon_path.ends_with(".svg") {
            // Use the svg() function directly as shown in your example
            svg(icon_path).width(64).height(64).into()
        } else {
            // Fallback for PNG/JPG
            app_image(icon_path).width(64).height(64).into()
        };

        // 3. Status Text
        let status_text = if is_selected {
            text("✅ Selected")
                .size(12)
                .color(Color::from_rgb(0.0, 0.8, 0.0))
        } else if !is_available {
            text("Unavailable")
                .size(12)
                .color(Color::from_rgb(0.5, 0.5, 0.5))
        } else {
            text("Click to Select")
                .size(12)
                .color(Color::from_rgb(0.7, 0.7, 0.7))
        };

        let card_content = column![
            container(icon_widget).height(80).center_y(Length::Fill),
            text(&app.name).size(14).align_y(Alignment::Center),
            status_text
        ]
        .spacing(5)
        .align_x(Alignment::Center);

        // 4. Style Logic
        let card_style = move |_theme: &Theme| {
            if is_selected {
                container::Style {
                    background: Some(Background::Color(Color::from_rgb(0.2, 0.25, 0.3))),
                    border: Border {
                        color: Color::from_rgb(0.4, 1.0, 0.4),
                        width: 2.0,
                        radius: 8.0.into(),
                    },
                    ..Default::default()
                }
            } else if !is_available {
                container::Style {
                    background: Some(Background::Color(Color::from_rgb(0.1, 0.1, 0.1))),
                    text_color: Some(Color::from_rgb(0.5, 0.5, 0.5)),
                    border: Border {
                        color: Color::from_rgb(0.3, 0.3, 0.3),
                        width: 1.0,
                        radius: 8.0.into(),
                    },
                    ..Default::default()
                }
            } else {
                container::Style {
                    background: Some(Background::Color(Color::from_rgb(0.2, 0.2, 0.2))),
                    border: Border {
                        color: Color::from_rgb(0.4, 0.4, 0.4),
                        width: 1.0,
                        radius: 8.0.into(),
                    },
                    shadow: Shadow {
                        color: Color::BLACK,
                        offset: Vector::new(0.0, 2.0),
                        blur_radius: 5.0,
                    },
                    ..Default::default()
                }
            }
        };

        // 5. Button Wrapper
        let mut btn = button(
            container(card_content)
                .width(Length::Fixed(150.0))
                .height(Length::Fixed(160.0))
                .padding(10)
                .style(card_style),
        )
        .padding(0);

        // 6. Interaction
        if is_available {
            btn = btn.on_press(Message::ToggleApp(app.id.clone(), !is_selected));
        }

        // 7. Tooltip
        tooltip(
            btn,
            container(text(
                app.description
                    .clone()
                    .unwrap_or("No description".to_string()),
            ))
            .padding(5),
            tooltip::Position::Bottom,
        )
        .style(|_| container::Style {
            background: Some(Background::Color(Color::from_rgb(0.1, 0.1, 0.1))),
            border: Border {
                color: Color::WHITE,
                width: 1.0,
                radius: 4.0.into(),
            },
            text_color: Some(Color::WHITE),
            ..Default::default()
        })
        .into()
    }

    fn view(&self) -> Element<Message> {
        let content: Element<Message> = if self.loading {
            container(text("Loading...").size(30))
                .center_x(Length::Fill)
                .into()
        } else {
            // IMPERATIVE GRID LOOP (Reliable)
            let mut grid_column = column![].spacing(20);
            for chunk in self.apps.chunks(3) {
                let mut chunk_row = row![].spacing(20);
                for app in chunk {
                    chunk_row = chunk_row.push(self.view_app_card(app));
                }
                grid_column = grid_column.push(chunk_row);
            }

            scrollable(grid_column)
                .height(Length::Fill)
                .width(Length::Fill)
                .into()
        };

        // Sidebar / Footer
        let is_busy = self.current_install.is_some();
        let can_install = !self.selected_apps.is_empty() && !is_busy;

        let install_btn = button("Install Selected")
            .padding(15)
            .width(Length::Fill)
            .on_press_maybe(if can_install {
                Some(Message::StartInstall)
            } else {
                None
            });

        let log_view =
            scrollable(column(self.logs.iter().map(|log| text(log).size(14).into())).spacing(5))
                .height(Length::Fixed(150.0));

        let main_layout = column![
            text("The Init Project")
                .size(30)
                .font(iced::Font::MONOSPACE),
            container(content).height(Length::Fill).width(Length::Fill),
            install_btn,
            text("Status Log:").size(16).font(iced::Font::MONOSPACE),
            container(log_view)
                .style(|_| container::Style {
                    border: Border {
                        color: Color::from_rgb(0.3, 0.3, 0.3),
                        width: 1.0,
                        radius: 4.0.into()
                    },
                    ..Default::default()
                })
                .padding(10)
        ]
        .spacing(20)
        .padding(20);

        container(main_layout).height(Length::Fill).into()
    }
}

async fn load_apps() -> Result<Vec<App>, String> {
    let current_dir = std::env::current_dir().map_err(|e| e.to_string())?;
    let path = current_dir.join("apps.toml");

    // Convert path to string for logging/errors
    let path_str = path.to_str().unwrap_or("apps.toml");

    match manifest::load_manifest(path_str).await {
        Ok(manifest) => Ok(manifest.apps),
        Err(e) => Err(format!("Could not load {}: {}", path_str, e)),
    }
}

async fn install_app(app_id: String, package_id: String) -> (String, Result<(), String>) {
    let manager = detectors::get_system_manager();
    let result = manager
        .install(&package_id)
        .await
        .map_err(|e| e.to_string());
    (app_id, result)
}
