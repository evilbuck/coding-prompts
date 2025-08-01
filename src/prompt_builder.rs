use std::path::PathBuf;
use std::fs;
use std::io;

#[derive(Debug, Clone)]
pub struct FileReference {
    pub path: PathBuf,
    pub display_name: String,
    pub order: usize,
}

impl FileReference {
    pub fn new(path: PathBuf, display_name: String, order: usize) -> Self {
        Self {
            path,
            display_name,
            order,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PromptState {
    // File references (paths only, not content)
    file_contexts: Vec<FileReference>,
    // Future: User instructions
    user_instructions: Option<String>,
    // Future: System prompts
    system_prompts: Vec<String>,
    next_order: usize,
}

#[derive(Debug, Clone)]
pub struct PromptBuilder {
    state: PromptState,
}

impl PromptBuilder {
    pub fn new() -> Self {
        Self {
            state: PromptState::default(),
        }
    }
    
    pub fn add_file(&mut self, path: PathBuf) -> Result<(), String> {
        // Check if file already exists in prompt
        if self.state.file_contexts.iter().any(|f| f.path == path) {
            return Err("File already in prompt".to_string());
        }
        
        // Verify file exists
        if !path.exists() {
            return Err("File does not exist".to_string());
        }
        
        let display_name = path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| path.display().to_string());
        
        self.state.file_contexts.push(FileReference::new(
            path,
            display_name,
            self.state.next_order,
        ));
        self.state.next_order += 1;
        
        Ok(())
    }
    
    pub fn add_directory(&mut self, dir_path: PathBuf) -> Result<usize, String> {
        if !dir_path.is_dir() {
            return Err("Path is not a directory".to_string());
        }
        
        let mut added_count = 0;
        self.add_directory_recursive(&dir_path, &mut added_count)?;
        self.state.next_order += added_count;
        
        Ok(added_count)
    }
    
    fn add_directory_recursive(&mut self, dir: &std::path::Path, count: &mut usize) -> Result<(), String> {
        let entries = fs::read_dir(dir)
            .map_err(|e| format!("Failed to read directory: {}", e))?;
        
        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
            let path = entry.path();
            
            if path.is_file() {
                // Skip if already added
                if !self.state.file_contexts.iter().any(|f| f.path == path) {
                    let display_name = path.strip_prefix(dir)
                        .unwrap_or(&path)
                        .to_string_lossy()
                        .to_string();
                    
                    self.state.file_contexts.push(FileReference::new(
                        path.clone(),
                        display_name,
                        self.state.next_order + *count,
                    ));
                    *count += 1;
                }
            } else if path.is_dir() {
                // Recursively process subdirectories
                self.add_directory_recursive(&path, count)?;
            }
        }
        
        Ok(())
    }
    
    pub fn remove_file(&mut self, path: &PathBuf) {
        self.state.file_contexts.retain(|f| &f.path != path);
    }
    
    pub fn remove_directory(&mut self, dir_path: &std::path::Path) {
        self.state.file_contexts.retain(|f| !f.path.starts_with(dir_path));
    }
    
    pub fn clear(&mut self) {
        self.state.file_contexts.clear();
        self.state.next_order = 0;
    }
    
    pub fn get_files(&self) -> &Vec<FileReference> {
        &self.state.file_contexts
    }
    
    pub fn file_count(&self) -> usize {
        self.state.file_contexts.len()
    }
    
    // Build the actual prompt by reading file contents
    pub fn build_prompt(&self) -> Result<String, io::Error> {
        let mut prompt = String::new();
        
        for file_ref in &self.state.file_contexts {
            prompt.push_str(&format!("=== File: {} ===\n", file_ref.display_name));
            
            match fs::read_to_string(&file_ref.path) {
                Ok(content) => {
                    prompt.push_str(&content);
                    prompt.push_str("\n\n");
                },
                Err(e) => {
                    prompt.push_str(&format!("Error reading file: {}\n\n", e));
                }
            }
        }
        
        Ok(prompt)
    }
    
    // Get file information without reading content (lazy loading)
    pub fn get_file_info(&self) -> Vec<(String, String)> {
        self.state.file_contexts.iter().map(|file_ref| {
            let metadata = fs::metadata(&file_ref.path);
            let size = match metadata {
                Ok(meta) => format!("{} bytes", meta.len()),
                Err(_) => "Unknown size".to_string(),
            };
            (file_ref.display_name.clone(), size)
        }).collect()
    }
    
    // Check if a file exists without reading its content
    pub fn file_exists(&self, path: &PathBuf) -> bool {
        path.exists()
    }
    
    // Get count of files that can be read
    pub fn readable_files_count(&self) -> usize {
        self.state.file_contexts.iter().filter(|file_ref| {
            file_ref.path.exists() && file_ref.path.is_file()
        }).count()
    }
    
    // Get count of files that cannot be read
    pub fn unreadable_files_count(&self) -> usize {
        self.state.file_contexts.iter().filter(|file_ref| {
            !file_ref.path.exists() || !file_ref.path.is_file()
        }).count()
    }
}