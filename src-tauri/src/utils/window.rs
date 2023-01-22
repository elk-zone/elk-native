use tauri::utils::config::WindowConfig;
use tauri::{AppHandle, WindowBuilder, Wry};

pub trait WindowWithConfig<'a> {
    fn with_config(app: &'a AppHandle<Wry>, config: &WindowConfig) -> Self;
}

impl<'a> WindowWithConfig<'a> for WindowBuilder<'a, Wry> {
    fn with_config(app: &'a AppHandle<Wry>, config: &WindowConfig) -> Self {
        let mut builder = WindowBuilder::new(app, &config.label, config.url.clone())
            .title(&config.title)
            .fullscreen(config.fullscreen)
            .resizable(config.resizable)
            .inner_size(config.width, config.height);

        #[cfg(target_os = "macos")]
        {
            builder = builder
                .title_bar_style(config.title_bar_style.clone())
                .hidden_title(config.hidden_title);
        }

        if config.min_width.is_some() && config.min_height.is_some() {
            builder = builder.min_inner_size(config.min_width.unwrap(), config.min_height.unwrap());
        }

        if config.max_width.is_some() && config.max_height.is_some() {
            builder = builder.max_inner_size(config.max_width.unwrap(), config.max_height.unwrap());
        }

        builder
    }
}
