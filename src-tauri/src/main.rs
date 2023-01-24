#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use env_logger::filter::Builder as FilterBuilder;
use log::LevelFilter;
use tauri::Menu;
#[cfg(debug_assertions)]
use tauri_plugin_log::fern::colors::ColoredLevelConfig;
use tauri_plugin_log::{Builder as LogPluginBuilder, LogTarget};
use tauri_plugin_store::Builder as StorePluginBuilder;
use tauri_plugin_window_state::{Builder as WindowStateBuilder, StateFlags};

fn main() {
    let log_plugin = {
        let targets = [
            LogTarget::LogDir,
            #[cfg(debug_assertions)]
            LogTarget::Stdout,
        ];

        let filter = std::env::var("RUST_LOG")
            .map(|ref filter| FilterBuilder::new().parse(filter).build().filter())
            .unwrap_or(LevelFilter::Debug);

        let builder = LogPluginBuilder::new().targets(targets).level(filter);

        #[cfg(debug_assertions)]
        let builder = builder.with_colors(ColoredLevelConfig::default());

        builder.build()
    };

    tauri::Builder::default()
        .plugin(log_plugin)
        .plugin(StorePluginBuilder::default().build())
        .plugin(WindowStateBuilder::default().with_state_flags(StateFlags::SIZE | StateFlags::POSITION | StateFlags::MAXIMIZED).build())
        .menu(Menu::os_default("Elk"))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
