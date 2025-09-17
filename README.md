# Niri Config GUI

A modern graphical user interface for configuring the [Niri](https://github.com/YaLTeR/niri) Wayland compositor.

## 🎯 Project Goal

Simplify Niri configuration through an intuitive GUI, focusing on the most common pain points like monitor setup, keybindings, and window rules.

## 📦 Project Structure

This project uses Rust workspaces to maintain modularity and reusability:

### 🖥️ `niri-monitor`
**Monitor Detection & Configuration Library**
- Detects connected monitors via `niri msg outputs`
- Parses monitor capabilities (resolution, refresh rates, scale)
- Generates monitor configuration for `config.kdl`
- Handles complex multi-monitor setups

### ⚙️ `niri-config`
**Configuration Management Library**
- Reads and parses existing `~/.config/niri/config.kdl`
- Validates configuration syntax and structure
- Writes new configuration with proper formatting
- Backup and restore functionality

### 🔌 `niri-ipc`
**Niri Communication Library**
- Wrapper for `niri msg` commands
- Real-time communication with running Niri instance
- Configuration reload and validation
- Status monitoring and error handling

### 🖼️ `niri-gui`
**Main GUI Application**
- Visual interface built with Tauri + TypeScript
- Drag-and-drop monitor layout editor
- Keybinding configuration interface
- Real-time preview of changes

## 🚧 Development Status

### Phase 1: Backend Foundation
- [x] Setup workspace structure
- [x] Create basic module skeleton
- [x] Implement `niri-ipc` for command execution
- [x] Test communication with Niri

### Phase 2: Monitor Configuration
- [x] Parse `niri msg outputs` in `niri-monitor`
- [x] Create monitor data structures
- [x] Implement monitor positioning logic
- [ ] Generate KDL output configuration
- [ ] Handle edge cases (rotation, scaling, etc.)

### Phase 3: Configuration Management
- [ ] Parse existing `config.kdl` in `niri-config`
- [ ] Implement configuration validation
- [ ] Create backup/restore functionality
- [ ] Handle configuration merging

### Phase 4: GUI Development
- [ ] Setup Tauri application structure
- [ ] Create monitor layout visual editor
- [ ] Implement drag-and-drop positioning
- [ ] Add real-time preview
- [ ] Create configuration forms

### Phase 5: Advanced Features
- [ ] Keybinding editor
- [ ] Window rules configuration
- [ ] Animation settings
- [ ] Theme management
- [ ] Import/export configurations

### Phase 6: Polish & Distribution
- [ ] Comprehensive testing
- [ ] Documentation
- [ ] Package for AUR
- [ ] Create installation guides
- [ ] Community feedback integration

## 🛠️ Development

### Prerequisites
- Rust 1.70+
- Niri compositor running
- Linux environment (Wayland)

### Building
```bash
# Clone the repository
git clone https://github.com/enricolo4/niri-config-gui.git
cd niri-config-gui

# Build all crates
cargo build

# Run tests
cargo test

# Run the GUI (when available)
cargo run -p niri-gui
```

### Testing Individual Libraries
```bash
# Test monitor detection
cargo run -p niri-monitor --example detect

# Test config parsing  
cargo run -p niri-config --example parse

# Test Niri communication
cargo run -p niri-ipc --example status
```

## 🤝 Contributing

Contributions are welcome! This project is perfect for:
- Rust beginners wanting to learn with real-world code
- Niri users frustrated with manual configuration
- GUI developers interested in Tauri

### Areas for Contribution
- [ ] Monitor detection edge cases
- [ ] KDL parsing improvements
- [ ] GUI/UX design
- [ ] Testing on different hardware setups
- [ ] Documentation and examples

## 📋 Known Issues
- [ ] Ultra-wide monitor positioning
- [ ] Mixed DPI setups
- [ ] Monitor rotation handling
- [ ] HiDPI scaling conflicts

## 🎯 Roadmap

**v0.1.0** - Monitor Configuration MVP
- Basic monitor detection and positioning
- Simple GUI for layout editing
- Save/load basic configurations

**v0.2.0** - Full Configuration Support
- Complete `config.kdl` parsing
- Keybinding editor
- Window rules configuration

**v1.0.0** - Production Ready
- Stable API for all libraries
- Comprehensive GUI
- Package distribution

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [YaLTeR](https://github.com/YaLTeR) for creating Niri
- The Niri community for feedback and testing
- Tauri team for the excellent framework
