#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use env_logger::filter::Builder as FilterBuilder;
use log::LevelFilter;
#[cfg(debug_assertions)]
use tauri_plugin_log::fern::colors::ColoredLevelConfig;
use tauri_plugin_log::{LogTarget, LoggerBuilder};
use tauri_plugin_store::PluginBuilder;

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

        let builder = LoggerBuilder::new().targets(targets).level(filter);

        #[cfg(debug_assertions)]
        let builder = builder.with_colors(ColoredLevelConfig::default());

        builder.build()
    };

    tauri::Builder::default()
        .plugin(log_plugin)
        .plugin(PluginBuilder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
