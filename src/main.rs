use iced::{Element, Result, Settings, Size, Theme, Application, Command};
use iced::widget::{button, column, container, row, scrollable, text, checkbox};
use std::path::PathBuf;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Message {
    AddFolderPressed,
    FolderSelected(Option<PathBuf>),
    ToggleItemSelected(PathBuf, bool),
}

#[derive(Debug, Clone)]
pub struct Folder {
    path: PathBuf,
    items: Vec<FileSystemItem>,
}

#[derive(Debug, Clone)]
pub struct FileSystemItem {
    path: PathBuf,
    name: String,
    is_file: bool,
    selected: bool,
}

impl FileSystemItem {
    pub fn new(path: PathBuf) -> Self {
        let name = path.file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_else(|| "/".to_string());
        let is_file = path.is_file();
        
        Self {
            path,
            name,
            is_file,
            selected: false,
        }
    }
}

pub struct FileManager {
    folders: Vec<Folder>,
}

impl Application for FileManager {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (Self {
            folders: Vec::new(),
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Hyprland File Manager")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::AddFolderPressed => {
                Command::perform(async {}, |_| {
                    let folder = rfd::AsyncFileDialog::new()
                        .pick_folder()
                        .await
                        .map(|handle| handle.path().to_path_buf());
                    Message::FolderSelected(folder)
                })
            },
            Message::FolderSelected(Some(path)) => {
                // Read folder contents
                if let Ok(entries) = std::fs::read_dir(&path) {
                    let items: Vec<FileSystemItem> = entries
                        .filter_map(|entry| {
                            if let Ok(entry) = entry {
                                Some(FileSystemItem::new(entry.path()))
                            } else {
                                None
                            }
                        })
                        .collect();
                    
                    self.folders.push(Folder {
                        path: path.clone(),
                        items,
                    });
                }
                Command::none()
            },
            Message::FolderSelected(None) => {
                Command::none()
            },
            Message::ToggleItemSelected(path, selected) => {
                // Find and update the item in folders
                for folder in &mut self.folders {
                    for item in &mut folder.items {
                        if item.path == path {
                            item.selected = selected;
                            break;
                        }
                    }
                }
                Command::none()
            },
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let add_folder_button = button("Add Folder")
            .on_press(Message::AddFolderPressed)
            .padding(10);

        let mut content = column![add_folder_button].spacing(20);

        // Display folders and their contents
        for folder in &self.folders {
            let folder_name = folder.path
                .file_name()
                .map(|name| name.to_string_lossy().to_string())
                .unwrap_or_else(|| "/".to_string());
                
            let folder_header = text(format!("Folder: {}", folder_name))
                .size(18)
                .style(iced::widget::text::Style::default().color(iced::Color::from_rgb(0.0, 0.5, 1.0)));
                
            let mut folder_content = column![];
            
            for item in &folder.items {
                let icon = if item.is_file { "📄" } else { "📁" };
                let item_text = format!("{} {}", icon, item.name);
                
                let item_row = row![
                    checkbox("", item.selected)
                        .on_toggle(move |checked| Message::ToggleItemSelected(item.path.clone(), checked)),
                    text(item_text).size(14)
                ]
                .spacing(10)
                .align_items(iced::Alignment::Center);
                
                folder_content = folder_content.push(item_row);
            }
            
            let folder_section = column![
                folder_header,
                folder_content
            ]
            .spacing(10);
            
            content = content.push(folder_section);
        }

        container(scrollable(content))
            .padding(20)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}

#[tokio::main]
async fn main() -> Result {
    FileManager::run(Settings {
        window: iced::window::Settings {
            size: Size::new(600.0, 500.0),
            min_size: Some(Size::new(400.0, 300.0)),
            ..Default::default()
        },
        ..Default::default()
    })
}
