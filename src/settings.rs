use egui::{Color32, Rounding, Vec2};
use crate::theme::{NeoTheme, ThemeMode};
use crate::wallpaper::wallpapers;

#[derive(Default)]
pub struct SettingsPanel {
    pub active_tab: SettingsTab,
}

#[derive(Default, PartialEq, Clone)]
pub enum SettingsTab {
    #[default]
    Appearance,
    Wallpaper,
    Display,
    Sound,
    Network,
    Bluetooth,
    Users,
    DateTime,
    Keyboard,
    Mouse,
    Power,
    Privacy,
    Updates,
    System,
    About,
}

impl SettingsPanel {
    pub fn draw_window(
        &mut self,
        ctx: &egui::Context,
        theme: &mut NeoTheme,
        open: &mut bool,
    ) {
        let mut window_open = *open;
        egui::Window::new("Settings")
            .id(egui::Id::new("settings_window"))
            .open(&mut window_open)
            .default_size(Vec2::new(600.0, 450.0))
            .resizable(true)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // Sidebar tabs
                    ui.vertical(|ui| {
                        ui.set_min_width(140.0);
                        ui.set_max_width(140.0);

                        let tabs = [
                            ("🎨  Appearance",  SettingsTab::Appearance),
                            ("🖼  Wallpaper",   SettingsTab::Wallpaper),
                            ("🖥  Display",     SettingsTab::Display),
                            ("🔊  Sound",       SettingsTab::Sound),
                            ("🌐  Network",     SettingsTab::Network),
                            ("🔵  Bluetooth",   SettingsTab::Bluetooth),
                            ("👤  Users",       SettingsTab::Users),
                            ("🕐  Date & Time", SettingsTab::DateTime),
                            ("⌨  Keyboard",    SettingsTab::Keyboard),
                            ("🖱  Mouse",       SettingsTab::Mouse),
                            ("🔋  Power",       SettingsTab::Power),
                            ("🔒  Privacy",     SettingsTab::Privacy),
                            ("🔄  Updates",     SettingsTab::Updates),
                            ("⚙  System",      SettingsTab::System),
                            ("ℹ  About",       SettingsTab::About),
                        ];

                        for (label, tab) in &tabs {
                            let active = self.active_tab == *tab;
                            let btn = egui::Button::new(
                                egui::RichText::new(*label)
                                    .size(13.0)
                                    .color(if active { theme.accent() } else { theme.text() })
                            )
                            .fill(if active { theme.accent_dim() } else { Color32::TRANSPARENT })
                            .rounding(Rounding::same(6.0))
                            .min_size(Vec2::new(130.0, 32.0));
                            if ui.add(btn).clicked() {
                                self.active_tab = tab.clone();
                            }
                        }
                    });

                    ui.separator();

                    // Content
                    ui.vertical(|ui| {
                        match self.active_tab {
                            SettingsTab::Appearance => self.draw_appearance(ui, theme),
                            SettingsTab::Wallpaper  => self.draw_wallpaper(ui, theme),
                            SettingsTab::Display    => self.draw_display(ui, theme),
                            SettingsTab::Sound      => self.draw_sound(ui, theme),
                            SettingsTab::Network    => self.draw_network(ui, theme),
                            SettingsTab::Bluetooth  => self.draw_bluetooth(ui, theme),
                            SettingsTab::Users      => self.draw_users(ui, theme),
                            SettingsTab::DateTime   => self.draw_datetime(ui, theme),
                            SettingsTab::Keyboard   => self.draw_keyboard(ui, theme),
                            SettingsTab::Mouse      => self.draw_mouse(ui, theme),
                            SettingsTab::Power      => self.draw_power(ui, theme),
                            SettingsTab::Privacy    => self.draw_privacy(ui, theme),
                            SettingsTab::Updates    => self.draw_updates(ui, theme),
                            SettingsTab::System     => self.draw_system(ui, theme),
                            SettingsTab::About      => self.draw_about(ui, theme),
                        }
                    });
                });
            });
        *open = window_open;
    }

    fn draw_appearance(&mut self, ui: &mut egui::Ui, theme: &mut NeoTheme) {
        ui.heading("Appearance");
        ui.add_space(12.0);

        ui.label(egui::RichText::new("Theme Mode").size(13.0));
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            let modes = [("🌙  Dark", ThemeMode::Dark), ("☀  Light", ThemeMode::Light)];
            for (label, mode) in &modes {
                let active = theme.mode == *mode;
                let btn = egui::Button::new(egui::RichText::new(*label).size(13.0))
                    .fill(if active { theme.accent() } else { theme.card_bg() })
                    .stroke(egui::Stroke::new(1.0, if active { theme.accent() } else { theme.border() }))
                    .rounding(Rounding::same(8.0))
                    .min_size(Vec2::new(120.0, 36.0));
                if ui.add(btn).clicked() {
                    theme.mode = mode.clone();
                    theme.apply(ui.ctx());
                }
            }
        });

        ui.add_space(20.0);
        ui.label(egui::RichText::new("Accent Color").size(13.0));
        ui.add_space(8.0);
        ui.horizontal_wrapped(|ui| {
            for (name, rgb) in NeoTheme::accent_presets() {
                let color = Color32::from_rgb(rgb[0], rgb[1], rgb[2]);
                let active = theme.accent == rgb;
                let (id, rect) = ui.allocate_space(Vec2::new(56.0, 36.0));
                let resp = ui.interact(rect, id, egui::Sense::click());

                ui.painter().rect_filled(rect, Rounding::same(8.0), color);
                if active {
                    ui.painter().rect_stroke(rect, Rounding::same(8.0), egui::Stroke::new(2.0, Color32::WHITE));
                    ui.painter().text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        "✓",
                        egui::FontId::proportional(14.0),
                        Color32::WHITE,
                    );
                }
                if resp.hovered() && !active {
                    ui.painter().rect_stroke(rect, Rounding::same(8.0), egui::Stroke::new(1.5, Color32::WHITE));
                }
                if resp.hovered() {
                    egui::show_tooltip_at_pointer(ui.ctx(), egui::Id::new(format!("accent_tip_{}", name)), |ui| {
                        ui.label(name);
                    });
                }
                if resp.clicked() {
                    theme.accent = rgb;
                    theme.apply(ui.ctx());
                }
            }
        });

        ui.add_space(20.0);
        ui.label(egui::RichText::new("Bottom Bar Style").size(13.0));
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            for (label, val) in &[("🚢  Dock (macOS-style)", true), ("📋  Taskbar (classic)", false)] {
                let active = theme.use_dock == *val;
                let btn = egui::Button::new(egui::RichText::new(*label).size(13.0))
                    .fill(if active { theme.accent() } else { theme.card_bg() })
                    .stroke(egui::Stroke::new(1.0, if active { theme.accent() } else { theme.border() }))
                    .rounding(egui::Rounding::same(8.0))
                    .min_size(egui::Vec2::new(160.0, 36.0));
                if ui.add(btn).clicked() { theme.use_dock = *val; }
            }
        });
        ui.add_space(4.0);
        let mut color = egui::Color32::from_rgb(theme.accent[0], theme.accent[1], theme.accent[2]);
        if ui.color_edit_button_srgba(&mut color).changed() {
            theme.accent = [color.r(), color.g(), color.b()];
            theme.apply(ui.ctx());
        }
    }

    fn draw_wallpaper(&mut self, ui: &mut egui::Ui, theme: &mut NeoTheme) {
        ui.heading("Wallpaper");
        ui.add_space(8.0);
        ui.label(egui::RichText::new("Select a procedurally animated wallpaper").size(12.0).color(theme.text_dim()));
        ui.add_space(12.0);

        let wps = wallpapers();
        let cols = 3;
        egui::Grid::new("wp_grid")
            .spacing(Vec2::new(12.0, 12.0))
            .num_columns(cols)
            .show(ui, |ui| {
                for (i, wp) in wps.iter().enumerate() {
                    if i > 0 && i % cols == 0 { ui.end_row(); }

                    let active = theme.wallpaper_idx == i;
                    let (id, rect) = ui.allocate_space(Vec2::new(140.0, 80.0));
                    let resp = ui.interact(rect, id, egui::Sense::click());

                    // Mini preview
                    let preview_colors = [
                        (Color32::from_rgb(10, 20, 40), Color32::from_rgb(20, 180, 160)),
                        (Color32::from_rgb(30, 60, 180), Color32::from_rgb(100, 30, 200)),
                        (Color32::from_rgb(5, 5, 15), Color32::from_rgb(80, 20, 140)),
                        (Color32::from_rgb(15, 25, 50), Color32::from_rgb(40, 58, 95)),
                        (Color32::from_rgb(8, 10, 18), Color32::from_rgb(100, 50, 220)),
                        (Color32::from_rgb(5, 3, 15), Color32::from_rgb(0, 200, 100)),
                    ];
                    let (bg, fg) = preview_colors[i % preview_colors.len()];
                    ui.painter().rect_filled(rect, Rounding::same(8.0), bg);
                    // Simple gradient preview
                    for x in 0..14_u32 {
                        let t = x as f32 / 14.0;
                        let r = (bg.r() as f32 * (1.0-t) + fg.r() as f32 * t) as u8;
                        let g = (bg.g() as f32 * (1.0-t) + fg.g() as f32 * t) as u8;
                        let b = (bg.b() as f32 * (1.0-t) + fg.b() as f32 * t) as u8;
                        let x_pos = rect.left() + t * rect.width();
                        let wave = (t * std::f32::consts::PI * 2.0).sin() * 10.0;
                        ui.painter().circle_filled(
                            egui::Pos2::new(x_pos, rect.center().y + wave),
                            4.0,
                            Color32::from_rgba_premultiplied(r, g, b, 160),
                        );
                    }

                    if active {
                        ui.painter().rect_stroke(rect, Rounding::same(8.0), egui::Stroke::new(2.0, theme.accent()));
                    }

                    ui.painter().text(
                        egui::Pos2::new(rect.center().x, rect.bottom() - 10.0),
                        egui::Align2::CENTER_CENTER,
                        wp.name,
                        egui::FontId::proportional(10.0),
                        Color32::WHITE,
                    );

                    if resp.clicked() {
                        theme.wallpaper_idx = i;
                    }
                }
            });
    }

    fn draw_system(&mut self, ui: &mut egui::Ui, theme: &NeoTheme) {
        ui.heading("System");
        ui.add_space(12.0);
        ui.label(egui::RichText::new("System Actions").size(13.0));
        ui.add_space(8.0);
        for (label, cmd) in &[
            ("🔒  Lock Screen", "loginctl lock-session"),
            ("🔄  Restart",     "systemctl reboot"),
            ("⏻  Shutdown",    "systemctl poweroff"),
            ("💤  Suspend",     "systemctl suspend"),
            ("🚪  Log Out",     "loginctl terminate-user $USER"),
        ] {
            if ui.add(egui::Button::new(egui::RichText::new(*label).size(13.0))
                .rounding(egui::Rounding::same(8.0))
                .min_size(egui::Vec2::new(180.0, 36.0))).clicked() {
                let _ = std::process::Command::new("sh").arg("-c").arg(cmd).spawn();
            }
            ui.add_space(4.0);
        }
    }

    fn draw_display(&mut self, ui: &mut egui::Ui, theme: &NeoTheme) {
        ui.heading("Display");
        ui.add_space(12.0);
        ui.label(egui::RichText::new("Resolution & Scaling").size(13.0).color(theme.text_dim()));
        ui.add_space(8.0);
        for (label, cmd) in &[
            ("🖥  Display Settings (xrandr)", "arandr"),
            ("🔆  Brightness Control",        "brightnessctl"),
            ("📐  Screen Layout",             "xrandr --auto"),
        ] {
            if ui.add(egui::Button::new(egui::RichText::new(*label).size(13.0))
                .rounding(egui::Rounding::same(8.0))
                .min_size(egui::Vec2::new(220.0, 34.0))).clicked() {
                let _ = std::process::Command::new("sh").arg("-c").arg(cmd).spawn();
            }
            ui.add_space(4.0);
        }
        ui.add_space(12.0);
        ui.label(egui::RichText::new("Night Light").size(13.0).color(theme.text_dim()));
        ui.add_space(4.0);
        for (label, cmd) in &[
            ("🌙  Enable Night Light",  "redshift -O 4000"),
            ("☀  Disable Night Light", "redshift -x"),
        ] {
            if ui.add(egui::Button::new(egui::RichText::new(*label).size(13.0))
                .rounding(egui::Rounding::same(8.0))
                .min_size(egui::Vec2::new(220.0, 34.0))).clicked() {
                let _ = std::process::Command::new("sh").arg("-c").arg(cmd).spawn();
            }
            ui.add_space(4.0);
        }
    }

    fn draw_sound(&mut self, ui: &mut egui::Ui, theme: &NeoTheme) {
        ui.heading("Sound");
        ui.add_space(12.0);
        for (label, cmd) in &[
            ("🔊  Volume Control (PulseAudio)", "pavucontrol"),
            ("🎵  Audio Mixer",                 "alsamixer"),
            ("🔇  Mute / Unmute",               "pactl set-sink-mute @DEFAULT_SINK@ toggle"),
            ("🔉  Volume Down",                 "pactl set-sink-volume @DEFAULT_SINK@ -5%"),
            ("🔊  Volume Up",                   "pactl set-sink-volume @DEFAULT_SINK@ +5%"),
        ] {
            if ui.add(egui::Button::new(egui::RichText::new(*label).size(13.0))
                .rounding(egui::Rounding::same(8.0))
                .min_size(egui::Vec2::new(240.0, 34.0))).clicked() {
                let _ = std::process::Command::new("sh").arg("-c").arg(cmd).spawn();
            }
            ui.add_space(4.0);
        }
    }

    fn draw_network(&mut self, ui: &mut egui::Ui, theme: &NeoTheme) {
        ui.heading("Network");
        ui.add_space(12.0);
        // Show current connections
        let output = std::process::Command::new("nmcli")
            .args(["-t", "-f", "NAME,TYPE,STATE", "connection", "show", "--active"])
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).into_owned())
            .unwrap_or_else(|_| "nmcli not available".into());
        ui.label(egui::RichText::new("Active Connections").size(13.0).color(theme.text_dim()));
        ui.add_space(4.0);
        for line in output.lines().take(6) {
            ui.label(egui::RichText::new(format!("  {}", line)).size(12.0).color(theme.text()));
        }
        ui.add_space(12.0);
        for (label, cmd) in &[
            ("📶  Network Manager TUI",  "alacritty -e nmtui"),
            ("🔌  List WiFi Networks",   "alacritty -e bash -c 'nmcli dev wifi list; read'"),
            ("🔄  Restart Networking",   "systemctl restart NetworkManager"),
            ("🌐  Show IP Addresses",    "alacritty -e bash -c 'ip addr; read'"),
        ] {
            if ui.add(egui::Button::new(egui::RichText::new(*label).size(13.0))
                .rounding(egui::Rounding::same(8.0))
                .min_size(egui::Vec2::new(240.0, 34.0))).clicked() {
                let _ = std::process::Command::new("sh").arg("-c").arg(cmd).spawn();
            }
            ui.add_space(4.0);
        }
    }

    fn draw_bluetooth(&mut self, ui: &mut egui::Ui, theme: &NeoTheme) {
        ui.heading("Bluetooth");
        ui.add_space(12.0);
        let status = std::process::Command::new("bluetoothctl")
            .arg("show")
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).lines()
                .find(|l| l.contains("Powered"))
                .map(|l| if l.contains("yes") { "🟢 Powered ON" } else { "🔴 Powered OFF" })
                .unwrap_or("Unknown")
                .to_string())
            .unwrap_or_else(|_| "bluetoothctl not available".into());
        ui.label(egui::RichText::new(format!("Status: {}", status)).size(13.0));
        ui.add_space(12.0);
        for (label, cmd) in &[
            ("🔵  Enable Bluetooth",    "bluetoothctl power on"),
            ("⚫  Disable Bluetooth",   "bluetoothctl power off"),
            ("🔍  Scan for Devices",    "alacritty -e bash -c 'bluetoothctl scan on; read'"),
            ("📋  List Paired Devices", "alacritty -e bash -c 'bluetoothctl devices; read'"),
        ] {
            if ui.add(egui::Button::new(egui::RichText::new(*label).size(13.0))
                .rounding(egui::Rounding::same(8.0))
                .min_size(egui::Vec2::new(220.0, 34.0))).clicked() {
                let _ = std::process::Command::new("sh").arg("-c").arg(cmd).spawn();
            }
            ui.add_space(4.0);
        }
    }

    fn draw_users(&mut self, ui: &mut egui::Ui, theme: &NeoTheme) {
        ui.heading("Users & Accounts");
        ui.add_space(12.0);
        let users = std::fs::read_to_string("/etc/passwd").unwrap_or_default();
        ui.label(egui::RichText::new("System Users (UID ≥ 1000)").size(13.0).color(theme.text_dim()));
        ui.add_space(4.0);
        for line in users.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 3 {
                if let Ok(uid) = parts[2].parse::<u32>() {
                    if uid >= 1000 && uid < 65534 {
                        ui.label(egui::RichText::new(format!("  👤  {} (uid {})", parts[0], uid)).size(12.0));
                    }
                }
            }
        }
        ui.add_space(12.0);
        for (label, cmd) in &[
            ("🔑  Change Password",   "alacritty -e bash -c 'passwd; read'"),
            ("➕  Add User",          "alacritty -e bash -c 'read -p \"Username: \" u && sudo useradd -m $u; read'"),
            ("🗑  Delete User",       "alacritty -e bash -c 'read -p \"Username: \" u && sudo userdel -r $u; read'"),
        ] {
            if ui.add(egui::Button::new(egui::RichText::new(*label).size(13.0))
                .rounding(egui::Rounding::same(8.0))
                .min_size(egui::Vec2::new(200.0, 34.0))).clicked() {
                let _ = std::process::Command::new("sh").arg("-c").arg(cmd).spawn();
            }
            ui.add_space(4.0);
        }
    }

    fn draw_datetime(&mut self, ui: &mut egui::Ui, theme: &NeoTheme) {
        ui.heading("Date & Time");
        ui.add_space(12.0);
        let now = chrono::Local::now();
        ui.label(egui::RichText::new(format!("🕐  {}", now.format("%A, %d %B %Y  %H:%M:%S"))).size(15.0));
        ui.add_space(12.0);
        let tz = std::fs::read_to_string("/etc/timezone")
            .or_else(|_| std::process::Command::new("timedatectl").arg("show")
                .output().map(|o| String::from_utf8_lossy(&o.stdout).into_owned()))
            .unwrap_or_else(|_| "Unknown".into());
        ui.label(egui::RichText::new(format!("Timezone: {}", tz.trim())).size(12.0).color(theme.text_dim()));
        ui.add_space(12.0);
        for (label, cmd) in &[
            ("🌍  Set Timezone",         "alacritty -e bash -c 'timedatectl list-timezones | fzf | xargs sudo timedatectl set-timezone; read'"),
            ("🔄  Sync Time (NTP)",      "sudo timedatectl set-ntp true"),
            ("📅  Set Date/Time",        "alacritty -e bash -c 'read -p \"Date (YYYY-MM-DD HH:MM:SS): \" d && sudo timedatectl set-time \"$d\"; read'"),
        ] {
            if ui.add(egui::Button::new(egui::RichText::new(*label).size(13.0))
                .rounding(egui::Rounding::same(8.0))
                .min_size(egui::Vec2::new(240.0, 34.0))).clicked() {
                let _ = std::process::Command::new("sh").arg("-c").arg(cmd).spawn();
            }
            ui.add_space(4.0);
        }
    }

    fn draw_keyboard(&mut self, ui: &mut egui::Ui, theme: &NeoTheme) {
        ui.heading("Keyboard");
        ui.add_space(12.0);
        let layout = std::process::Command::new("localectl")
            .arg("status")
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).lines()
                .find(|l| l.contains("X11 Layout"))
                .map(|l| l.split(':').nth(1).unwrap_or("unknown").trim().to_string())
                .unwrap_or_else(|| "unknown".into()))
            .unwrap_or_else(|_| "unknown".into());
        ui.label(egui::RichText::new(format!("Current Layout: {}", layout)).size(13.0));
        ui.add_space(12.0);
        for (label, cmd) in &[
            ("⌨  Set Keyboard Layout", "alacritty -e bash -c 'read -p \"Layout (e.g. us, gb, de): \" l && sudo localectl set-x11-keymap $l; read'"),
            ("🔁  Repeat Rate",         "alacritty -e bash -c 'xset r rate 300 50; echo Done; read'"),
            ("📋  Show Shortcuts",      "alacritty -e bash -c 'xmodmap -pm; read'"),
        ] {
            if ui.add(egui::Button::new(egui::RichText::new(*label).size(13.0))
                .rounding(egui::Rounding::same(8.0))
                .min_size(egui::Vec2::new(220.0, 34.0))).clicked() {
                let _ = std::process::Command::new("sh").arg("-c").arg(cmd).spawn();
            }
            ui.add_space(4.0);
        }
    }

    fn draw_mouse(&mut self, ui: &mut egui::Ui, theme: &NeoTheme) {
        ui.heading("Mouse & Touchpad");
        ui.add_space(12.0);
        for (label, cmd) in &[
            ("🖱  Increase Speed",       "xinput set-prop 'pointer:' 'libinput Accel Speed' 0.5"),
            ("🐢  Decrease Speed",       "xinput set-prop 'pointer:' 'libinput Accel Speed' -0.5"),
            ("🔄  Natural Scrolling ON", "xinput set-prop 'pointer:' 'libinput Natural Scrolling Enabled' 1"),
            ("🔄  Natural Scrolling OFF","xinput set-prop 'pointer:' 'libinput Natural Scrolling Enabled' 0"),
            ("📋  List Input Devices",   "alacritty -e bash -c 'xinput list; read'"),
        ] {
            if ui.add(egui::Button::new(egui::RichText::new(*label).size(13.0))
                .rounding(egui::Rounding::same(8.0))
                .min_size(egui::Vec2::new(240.0, 34.0))).clicked() {
                let _ = std::process::Command::new("sh").arg("-c").arg(cmd).spawn();
            }
            ui.add_space(4.0);
        }
    }

    fn draw_power(&mut self, ui: &mut egui::Ui, theme: &NeoTheme) {
        ui.heading("Power Management");
        ui.add_space(12.0);
        let bat = std::fs::read_to_string("/sys/class/power_supply/BAT0/capacity")
            .or_else(|_| std::fs::read_to_string("/sys/class/power_supply/BAT1/capacity"))
            .map(|s| format!("🔋 Battery: {}%", s.trim()))
            .unwrap_or_else(|_| "🔌 No battery detected".into());
        ui.label(egui::RichText::new(bat).size(14.0));
        ui.add_space(12.0);
        for (label, cmd) in &[
            ("⚡  Performance Mode",    "sudo cpupower frequency-set -g performance"),
            ("🌿  Power Save Mode",     "sudo cpupower frequency-set -g powersave"),
            ("⚖  Balanced Mode",       "sudo cpupower frequency-set -g schedutil"),
            ("💤  Suspend",             "systemctl suspend"),
            ("🌙  Hibernate",           "systemctl hibernate"),
            ("📊  Power Statistics",    "alacritty -e bash -c 'upower -d; read'"),
        ] {
            if ui.add(egui::Button::new(egui::RichText::new(*label).size(13.0))
                .rounding(egui::Rounding::same(8.0))
                .min_size(egui::Vec2::new(220.0, 34.0))).clicked() {
                let _ = std::process::Command::new("sh").arg("-c").arg(cmd).spawn();
            }
            ui.add_space(4.0);
        }
    }

    fn draw_privacy(&mut self, ui: &mut egui::Ui, theme: &NeoTheme) {
        ui.heading("Privacy & Security");
        ui.add_space(12.0);
        for (label, cmd) in &[
            ("🔥  Firewall Status",      "alacritty -e bash -c 'sudo ufw status verbose; read'"),
            ("🔥  Enable Firewall",      "sudo ufw enable"),
            ("🔥  Disable Firewall",     "sudo ufw disable"),
            ("🛡  AppArmor Status",      "alacritty -e bash -c 'sudo aa-status; read'"),
            ("🔑  SSH Keys",             "alacritty -e bash -c 'ls -la ~/.ssh/; read'"),
            ("🗑  Clear Bash History",   "history -c && rm -f ~/.bash_history ~/.zsh_history"),
            ("🔒  Lock Screen Now",      "loginctl lock-session"),
        ] {
            if ui.add(egui::Button::new(egui::RichText::new(*label).size(13.0))
                .rounding(egui::Rounding::same(8.0))
                .min_size(egui::Vec2::new(220.0, 34.0))).clicked() {
                let _ = std::process::Command::new("sh").arg("-c").arg(cmd).spawn();
            }
            ui.add_space(4.0);
        }
    }

    fn draw_updates(&mut self, ui: &mut egui::Ui, theme: &NeoTheme) {
        ui.heading("Software Updates");
        ui.add_space(12.0);
        for (label, cmd) in &[
            ("🔄  Check for Updates",        "alacritty -e bash -c 'checkupdates; read'"),
            ("⬆  Update All Packages",       "alacritty -e bash -c 'sudo pacman -Syu; read'"),
            ("🧹  Clean Package Cache",       "alacritty -e bash -c 'sudo pacman -Sc; read'"),
            ("📦  Install Package",           "alacritty -e bash -c 'read -p \"Package: \" p && sudo pacman -S $p; read'"),
            ("🗑  Remove Package",            "alacritty -e bash -c 'read -p \"Package: \" p && sudo pacman -R $p; read'"),
            ("🔍  Search Packages",           "alacritty -e bash -c 'read -p \"Search: \" q && pacman -Ss $q; read'"),
            ("📋  List Installed",            "alacritty -e bash -c 'pacman -Q | less'"),
            ("🏪  Flatpak Update",            "alacritty -e bash -c 'flatpak update; read'"),
        ] {
            if ui.add(egui::Button::new(egui::RichText::new(*label).size(13.0))
                .rounding(egui::Rounding::same(8.0))
                .min_size(egui::Vec2::new(240.0, 34.0))).clicked() {
                let _ = std::process::Command::new("sh").arg("-c").arg(cmd).spawn();
            }
            ui.add_space(4.0);
        }
    }

    fn draw_about(&mut self, ui: &mut egui::Ui, theme: &NeoTheme) {
        ui.add_space(12.0);
        ui.vertical_centered(|ui| {
            ui.label(egui::RichText::new("Ng'anjo").size(42.0).color(theme.accent()).strong());
            ui.label(egui::RichText::new("GUI").size(22.0).color(theme.text_dim()));
            ui.add_space(8.0);
            ui.label(egui::RichText::new("Version 1.0 Lite  ·  \"Arise\"").size(13.0).color(theme.text_dim()));
            ui.label(egui::RichText::new("Built with Rust + egui/eframe").size(12.0).color(theme.text_dim()));
            ui.label(egui::RichText::new("By Nehemiah Ng'anjo").size(12.0).color(theme.text_dim()));
            ui.add_space(12.0);
            ui.separator();
            ui.add_space(8.0);
        });
        // Live system info
        let kernel = std::fs::read_to_string("/proc/version")
            .map(|s| s.split_whitespace().nth(2).unwrap_or("?").to_string())
            .unwrap_or_else(|_| "?".into());
        let hostname = std::fs::read_to_string("/etc/hostname")
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|_| "?".into());
        let uptime = std::fs::read_to_string("/proc/uptime")
            .map(|s| {
                let secs = s.split_whitespace().next().unwrap_or("0").parse::<f64>().unwrap_or(0.0) as u64;
                format!("{}h {}m", secs / 3600, (secs % 3600) / 60)
            })
            .unwrap_or_else(|_| "?".into());
        let mem = std::fs::read_to_string("/proc/meminfo")
            .map(|s| {
                let total = s.lines().find(|l| l.starts_with("MemTotal")).and_then(|l| l.split_whitespace().nth(1)).unwrap_or("0").parse::<u64>().unwrap_or(0);
                format!("{} MB", total / 1024)
            })
            .unwrap_or_else(|_| "?".into());

        for (label, val) in &[
            ("Kernel",   kernel.as_str()),
            ("Hostname", hostname.as_str()),
            ("Uptime",   uptime.as_str()),
            ("RAM",      mem.as_str()),
            ("Arch",     "x86_64"),
            ("Base",     "Arch Linux"),
        ] {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(format!("{:<12}", label)).size(12.0).color(theme.text_dim()));
                ui.label(egui::RichText::new(*val).size(12.0).color(theme.text()));
            });
        }
    }
}
