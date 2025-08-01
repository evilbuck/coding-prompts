# Product Requirements Document: Hyprland File Manager

## Overview
A lightweight file management application for Hyprland window manager on Arch Linux, built using Rust.

## Project Goals
Build a simple, efficient file manager that integrates well with the Hyprland compositor, focusing on:
- Native Wayland support
- Minimal resource usage
- Clean, functional interface
- Progressive feature development

## Development Stages

### Stage 1: Basic Window Application
**Objective**: Establish the foundation with a working window
- Display a window with "Hello World" message
- Ensure proper Wayland/Hyprland compatibility
- Set up basic Rust project structure
- Implement window lifecycle (open, close, resize)

**Success Criteria**:
- Application launches without errors
- Window displays correctly in Hyprland
- Basic window controls work (close, move, resize)

### Stage 2: File Picker Interface
**Objective**: Implement core file browsing functionality
- Add ability to browse and display folder contents
- Implement "Add Folder" functionality to include directories in view
- Display files and folders with checkboxes for selection
- Support multiple folder views

**Success Criteria**:
- Can add folders to the application view
- Files and folders display with functional checkboxes
- Basic file/folder information visible (name, type indicator)

## Technical Requirements

### Platform
- **OS**: Arch Linux
- **Window Manager**: Hyprland (Wayland compositor)
- **Language**: Rust

### Technical Stack (Proposed)
- **GUI Framework**: 
  - Option 1: `iced` - Pure Rust, supports Wayland
  - Option 2: `egui` - Immediate mode GUI, good Wayland support
  - Option 3: `gtk4-rs` - GTK4 bindings, native Wayland support
- **File System Operations**: Standard Rust `std::fs`
- **Async Runtime** (if needed): `tokio`

### Architecture Considerations
- Event-driven architecture for UI responsiveness
- Separation of UI and file system logic
- Prepare for future feature additions (file operations, preview, etc.)

## User Interface Requirements

### Stage 1 UI
- Single window with centered "Hello World" text
- Window title: "Hyprland File Manager"
- Minimum window size: 400x300 pixels

### Stage 2 UI
- **Layout**:
  - Top bar with "Add Folder" button
  - Main area showing file/folder list
  - Each item has checkbox, icon, and name
- **Interactions**:
  - Click "Add Folder" to open folder selection dialog
  - Click checkboxes to select/deselect items
  - Basic scrolling for long lists

## Future Considerations (Not in Current Scope)
- File operations (copy, move, delete)
- File preview functionality
- Search and filter capabilities
- Keyboard shortcuts
- Configuration/preferences
- Theme support

## Development Approach
1. Start with minimal viable implementation
2. Iterate based on functionality and performance
3. Maintain clean, documented code
4. Consider Wayland-specific requirements throughout

## Success Metrics
- Application stability (no crashes during normal use)
- Responsive UI (no noticeable lag)
- Proper Hyprland integration
- Clean, maintainable codebase