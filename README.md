# NeoDesktop

A beautiful, lightweight desktop environment for Arch-based Linux, written entirely in Rust using [egui](https://github.com/emilk/egui).

```
┌─────────────────────────────────────────────────────────────┐
│  NeoDesktop — Rust-powered GPU-accelerated desktop shell    │
│  Wallpapers • App Launcher • File Manager • Notifications   │
└─────────────────────────────────────────────────────────────╝
```

## Features

| Feature | Description |
|---|---|
| **6 Animated Wallpapers** | Aurora, Mesh Gradient, Deep Space, Mountain, Geometric, Neon City |
| **App Launcher** | Searchable app grid with category filter (Ctrl+Space) |
| **File Manager** | Full file browser with bookmarks, grid/list view, search |
| **Settings Panel** | Theme mode, accent colors, wallpaper picker, system actions |
| **Taskbar** | Pinned apps, clock, app indicators |
| **Notifications** | Animated toast notifications with auto-dismiss |
| **Right-click Menu** | Desktop context menu |
| **Dark & Light Mode** | Fully themed |
| **Custom Accent Colors** | 6 presets + color picker |

## Requirements

- Rust 1.75+ (installed via `install.sh` if missing)
- X11 or Wayland display server
- A display manager (SDDM, LightDM, GDM, etc.)

### System dependencies (Arch)
```bash
sudo pacman -S libxkbcommon wayland mesa
```

## Installation

```bash
chmod +x install.sh
./install.sh
```

This will:
1. Check/install Rust
2. Build in release mode
3. Install binary to `/usr/local/bin/neodesktop`
4. Install `.desktop` file for your display manager

## Running

### As a desktop session (recommended)
Log out → select **NeoDesktop** at the login screen.

### Standalone (for testing)
```bash
neodesktop
# or from source:
cargo run --release
```

## Keyboard Shortcuts

| Shortcut | Action |
|---|---|
| `Ctrl+Space` | Toggle App Launcher |
| `Escape` | Close Launcher / Context Menu |
| Right-click desktop | Context Menu |
| Double-click icon | Open app |

## Adding More Apps

Edit `src/launcher.rs` and add entries to `builtin_apps()`:
```rust
AppEntry { name: "My App", desc: "Description", icon: "🚀", cmd: "myapp", category: "Tools" },
```

## Structure

```
neodesktop/
├── src/
│   ├── main.rs          # Entry point
│   ├── app.rs           # Main app state & loop
│   ├── theme.rs         # Colors, dark/light, accent
│   ├── wallpaper.rs     # 6 procedural animated wallpapers
│   ├── taskbar.rs       # Bottom taskbar / system tray
│   ├── launcher.rs      # App launcher overlay
│   ├── filemanager.rs   # File manager window
│   ├── settings.rs      # Settings panel
│   ├── notifications.rs # Toast notifications
│   └── widgets.rs       # Context menu, desktop icons
├── Cargo.toml
├── install.sh
└── neodesktop.desktop
```

## Built With

- [eframe/egui](https://github.com/emilk/egui) — immediate mode GUI
- [chrono](https://github.com/chronotope/chrono) — date/time
- [dirs](https://github.com/dirs-dev/dirs-rs) — standard directories
- [open](https://github.com/Byron/open-rs) — open files with default apps
- [walkdir](https://github.com/BurntSushi/walkdir) — directory traversal
