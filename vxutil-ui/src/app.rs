//! Main application state and message handling

use iced::{Element, Task, Theme};
use vxutil_core::{Project, ProjectSettings};

pub struct VxUtil {
    project: Option<Project>,
}

#[derive(Debug, Clone)]
pub enum Message {
    // Project messages
    NewProject,
    OpenProject,
    SaveProject,

    // Will add more messages as we build features
}

impl VxUtil {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                project: None,
            },
            Task::none(),
        )
    }

    pub fn title(&self) -> String {
        match &self.project {
            Some(project) => format!("VxUtil - {}", project.name),
            None => "VxUtil".to_string(),
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::NewProject => {
                tracing::info!("Creating new project");
                self.project = Some(Project::new(
                    "Untitled Project".to_string(),
                    std::env::current_dir().unwrap_or_default(),
                    ProjectSettings::default(),
                ));
            }
            Message::OpenProject => {
                tracing::info!("Opening project");
                // TODO: File dialog
            }
            Message::SaveProject => {
                tracing::info!("Saving project");
                // TODO: Save project
            }
        }

        Task::none()
    }

    pub fn view(&self) -> Element<Message> {
        use iced::widget::{button, column, container, text};

        let content = if self.project.is_some() {
            column![
                text("VxUtil Video Editor").size(32),
                text("Project loaded - UI coming soon!"),
            ]
            .spacing(20)
        } else {
            column![
                text("VxUtil Video Editor").size(32),
                text("Welcome to VxUtil!").size(16),
                button("New Project").on_press(Message::NewProject),
                button("Open Project").on_press(Message::OpenProject),
            ]
            .spacing(20)
        };

        container(content)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .center(iced::Length::Fill)
            .into()
    }

    pub fn theme(&self) -> Theme {
        Theme::Dark
    }
}