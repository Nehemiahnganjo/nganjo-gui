use egui::{Color32, Pos2, Rect, Vec2};
use std::time::Instant;
use crate::theme::NeoTheme;
use crate::taskbar::Taskbar;
use crate::dock::Dock;
use crate::launcher::Launcher;
use crate::filemanager::FileManager;
use crate::settings::SettingsPanel;
use crate::notifications::{Notification, NotificationManager};
use crate::widgets::{ContextMenu, draw_desktop_icons, default_desktop_icons, DesktopIcon};
use crate::wallpaper::draw_wallpaper;

pub struct NganjoGUI {
    pub theme: NeoTheme,
    pub taskbar: Taskbar,
    pub dock: Dock,
    pub launcher: Launcher,
    pub show_launcher: bool,
    pub file_manager: FileManager,
    pub settings_panel: SettingsPanel,
    pub notifications: NotificationManager,
    pub context_menu: ContextMenu,
    pub desktop_icons: Vec<DesktopIcon>,
    pub open_windows: Vec<String>,
    pub start_time: Instant,
}

impl NganjoGUI {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let theme = NeoTheme::default();
        theme.apply(&cc.egui_ctx);

        // Enable font anti-aliasing and high quality rendering
        cc.egui_ctx.set_pixels_per_point(1.0);

        let mut notifs = NotificationManager::default();
        notifs.push(Notification::success("NganjoGUI Ready", "Welcome! Double-click icons to open apps."));

        Self {
            theme,
            taskbar: Taskbar::default(),
            dock: Dock::default(),
            launcher: Launcher::default(),
            show_launcher: false,
            file_manager: FileManager::default(),
            settings_panel: SettingsPanel::default(),
            notifications: notifs,
            context_menu: ContextMenu::desktop_menu(),
            desktop_icons: default_desktop_icons(),
            open_windows: Vec::new(),
            start_time: Instant::now(),
        }
    }

    fn handle_desktop_input(&mut self, ctx: &egui::Context, screen_rect: Rect) {
        // Right-click context menu on desktop
        let taskbar_h = 44.0;
        let desktop_rect = Rect::from_min_size(
            screen_rect.min,
            Vec2::new(screen_rect.width(), screen_rect.height() - taskbar_h),
        );

        let right_clicked = ctx.input(|i| {
            i.pointer.secondary_released()
                && i.pointer.hover_pos()
                    .map(|p| desktop_rect.contains(p))
                    .unwrap_or(false)
        });

        if right_clicked {
            if let Some(pos) = ctx.input(|i| i.pointer.hover_pos()) {
                self.context_menu.pos = pos;
                self.context_menu.visible = true;
                self.show_launcher = false;
            }
        }

        // Close launcher on desktop click (outside launcher area)
        if self.show_launcher {
            let clicked = ctx.input(|i| i.pointer.primary_released());
            if clicked {
                // Launcher handles its own close logic via the area
            }
        }
    }
}

impl eframe::App for NganjoGUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let screen_rect = ctx.screen_rect();
        let time = self.start_time.elapsed().as_secs_f32();

        // Request continuous repaint for animations
        ctx.request_repaint();

        // Apply theme
        self.theme.apply(ctx);

        // Draw wallpaper as background
        let painter = ctx.layer_painter(egui::LayerId::background());
        draw_wallpaper(&painter, screen_rect, self.theme.wallpaper_idx, time);

        // Desktop icons
        draw_desktop_icons(ctx, &self.theme, &self.desktop_icons, screen_rect, &mut self.open_windows);

        // Handle desktop input (right-click etc)
        self.handle_desktop_input(ctx, screen_rect);

        // Context menu
        let _ctx_action = self.context_menu.draw(ctx, &self.theme, &mut self.open_windows);

        // Open windows
        let mut fm_open = self.open_windows.contains(&"filemanager".to_string());
        let mut settings_open = self.open_windows.contains(&"settings".to_string());

        if fm_open {
            self.file_manager.draw_window(ctx, &self.theme, &mut fm_open);
            if !fm_open {
                self.open_windows.retain(|w| w != "filemanager");
            }
        }

        if settings_open {
            self.settings_panel.draw_window(ctx, &mut self.theme, &mut settings_open);
            if !settings_open {
                self.open_windows.retain(|w| w != "settings");
            }
        }

        // Launcher overlay — driven by either dock or taskbar
        let show_launcher = if self.theme.use_dock {
            &mut self.dock.show_launcher
        } else {
            &mut self.taskbar.show_launcher
        };
        self.launcher.draw(ctx, &self.theme, screen_rect, show_launcher, &mut self.open_windows);

        // Draw dock OR taskbar
        if self.theme.use_dock {
            self.dock.draw(ctx, &self.theme, screen_rect, &mut self.open_windows);
        } else {
            self.taskbar.draw(ctx, &self.theme, screen_rect, &mut self.open_windows);
        }

        // Notifications
        self.notifications.draw(ctx, &self.theme, screen_rect);

        // Handle taskbar settings button
        if self.taskbar.show_settings {
            if !self.open_windows.contains(&"settings".to_string()) {
                self.open_windows.push("settings".to_string());
            }
            self.taskbar.show_settings = false;
        }

        // Global keyboard shortcuts
        ctx.input(|i| {
            // Super key equivalent: Ctrl+Space to toggle launcher
            if i.key_pressed(egui::Key::Space) && i.modifiers.ctrl {
                self.taskbar.show_launcher = !self.taskbar.show_launcher;
            }
        });
    }
}
