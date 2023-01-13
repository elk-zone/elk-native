#[cfg(mobile)]
mod mobile;
#[cfg(mobile)]
pub use mobile::*;

use tauri_plugin_store::Builder as StorePluginBuilder;

pub fn main() {
    #[cfg(not(mobile))]
    let log_plugin = {
        use env_logger::filter::Builder as FilterBuilder;
        use log::LevelFilter;
        #[cfg(debug_assertions)]
        use tauri_plugin_log::fern::colors::ColoredLevelConfig;
        use tauri_plugin_log::{Builder as LogPluginBuilder, LogTarget};

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

    let builder = tauri::Builder::default().plugin(StorePluginBuilder::default().build());

    #[cfg(not(mobile))]
    let builder = builder.plugin(log_plugin);

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
