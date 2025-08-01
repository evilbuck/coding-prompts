use iced::{Element, Result, Settings, Size, Theme, Application, Command};
use iced::widget::{button, column, container, row, scrollable, text, checkbox};
use std::path::PathBuf;
mod prompt_builder;
use prompt_builder::PromptBuilder;

#[derive(Debug, Clone)]
pub enum Message {
    AddFolderPressed,
    FolderSelected(Option<PathBuf>),
    ToggleItemSelected(PathBuf, bool),
    BuildPrompt,
    PromptBuilt(String),
    TogglePromptPanel,
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
    prompt_builder: PromptBuilder,
    show_prompt_panel: bool,
}

impl Application for FileManager {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (Self {
            folders: Vec::new(),
            prompt_builder: PromptBuilder::new(),
            show_prompt_panel: false,
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Hyprland File Manager")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::AddFolderPressed => {
                Command::perform(
                    async {
                        let folder = rfd::AsyncFileDialog::new()
                            .pick_folder()
                            .await
                            .map(|handle| handle.path().to_path_buf());
                        Message::FolderSelected(folder)
                    },
                    |message| message
                )
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
                
                // Update prompt builder state
                if selected {
                    if path.is_file() {
                        let _ = self.prompt_builder.add_file(path);
                    } else if path.is_dir() {
                        match self.prompt_builder.add_directory(path) {
                            Ok(_count) => {
                                // Could show a toast/notification: "Added {count} files"
                            },
                            Err(_e) => {
                                // Handle error - could show error message
                            }
                        }
                    }
                } else {
                    if path.is_file() {
                        self.prompt_builder.remove_file(&path);
                    } else if path.is_dir() {
                        self.prompt_builder.remove_directory(&path);
                    }
                }
                
                Command::none()
            },
            Message::BuildPrompt => {
                match self.prompt_builder.build_prompt() {
                    Ok(prompt) => Command::perform(
                        async move { Message::PromptBuilt(prompt) },
                        |message| message
                    ),
                    Err(e) => {
                        // In a real implementation, we might want to show this error in the UI
                        println!("Error building prompt: {}", e);
                        Command::none()
                    }
                }
            },
            Message::PromptBuilt(prompt) => {
                // In a real implementation, we would display this prompt in the UI
                println!("Built prompt:\n{}", prompt);
                Command::none()
            },
            Message::TogglePromptPanel => {
                self.show_prompt_panel = !self.show_prompt_panel;
                Command::none()
            },
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let add_folder_button = button("Add Folder")
            .on_press(Message::AddFolderPressed)
            .padding(10);

        // Add a button to build the prompt if we have files selected
        let build_prompt_button = if self.prompt_builder.file_count() > 0 {
            button(text(format!("Build Prompt ({})", self.prompt_builder.file_count())))
                .on_press(Message::BuildPrompt)
                .padding(10)
        } else {
            button(text("Build Prompt (0 files)"))
                .padding(10)
        };

        // Add a button to toggle the prompt panel
        let toggle_prompt_panel_button = button(text(if self.show_prompt_panel { "Hide Prompt Panel" } else { "Show Prompt Panel" }))
            .on_press(Message::TogglePromptPanel)
            .padding(10);

        let mut content = column![
            row![
                add_folder_button,
                build_prompt_button,
                toggle_prompt_panel_button
            ].spacing(10)
        ].spacing(20);

        // Display folders and their contents
        for folder in &self.folders {
            let folder_name = folder.path
                .file_name()
                .map(|name| name.to_string_lossy().to_string())
                .unwrap_or_else(|| "/".to_string());
                
            let folder_header = text(format!("Folder: {}", folder_name))
                .size(18)
                .style(iced::Color::from_rgb(0.0, 0.5, 1.0));
                
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

        // Display prompt panel if toggled on
        if self.show_prompt_panel {
            let prompt_panel_header = text("Prompt State")
                .size(18)
                .style(iced::Color::from_rgb(0.0, 0.7, 0.0));
            
            let file_info = self.prompt_builder.get_file_info();
            let readable_count = self.prompt_builder.readable_files_count();
            let unreadable_count = self.prompt_builder.unreadable_files_count();
            
            let stats_text = format!(
                "Files: {} total ({} readable, {} unreadable)",
                file_info.len(),
                readable_count,
                unreadable_count
            );
            
            let stats_row = text(stats_text).size(12).style(iced::Color::from_rgb(0.3, 0.3, 0.3));
            
            let mut prompt_files_content = column![];
            
            for (display_name, size) in file_info {
                let file_row = row![
                    text(format!("📄 {}", display_name)).size(14),
                    text(size).size(12).style(iced::Color::from_rgb(0.5, 0.5, 0.5))
                ]
                .spacing(10)
                .align_items(iced::Alignment::Center);
                
                prompt_files_content = prompt_files_content.push(file_row);
            }
            
            let prompt_panel = column![
                prompt_panel_header,
                stats_row,
                prompt_files_content
            ]
            .spacing(10)
            .padding(10);
            
            content = content.push(container(prompt_panel)
                .style(|_theme: &Theme| {
                    container::Appearance {
                        border: iced::Border {
                            color: iced::Color::from_rgb(0.0, 0.7, 0.0),
                            width: 1.0,
                            radius: 5.0.into(),
                        },
                        ..Default::default()
                    }
                })
            );
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
