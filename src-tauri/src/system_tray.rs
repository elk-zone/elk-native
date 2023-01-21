use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, WindowBuilder,
    Wry,
};

pub(crate) fn create_tray() -> SystemTray {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("show_window", "Open Elk"))
        .add_item(CustomMenuItem::new("quit", "Close Elk"));

    SystemTray::new().with_menu(tray_menu)
}

pub(crate) fn system_tray_event_handler(app: &AppHandle<Wry>, event: SystemTrayEvent) {
    if let SystemTrayEvent::MenuItemClick { id, .. } = event {
        match id.as_str() {
            "quit" => {
                std::process::exit(0);
            }
            "show_window" => {
                let window = app.get_window("main").unwrap_or_else(|| {
                    WindowBuilder::new(app, "main", tauri::WindowUrl::App("/".into()))
                        .title("Elk")
                        .build()
                        .expect("Could not create new main window")
                });

                if !window.is_visible().unwrap_or(true) {
                    window.show().expect("Could not show main window");
                }

                window.set_focus().expect("Could not put window on top");
            }
            _ => {}
        }
    }
}
