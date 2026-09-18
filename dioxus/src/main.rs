mod settings;
mod tabs;

use std::time::Duration;

use dioxus::desktop::{Config, WindowBuilder};
use dioxus::prelude::*;

use settings::{AppSettings, Contrast, ThemeMode};
use tabs::{SettingsTab, StopwatchTab, SynctimeTab, WallclockTab};

const CSS: &str = include_str!("../assets/main.css");

fn main() {
    let window = WindowBuilder::new()
        .with_title("GUI of Tomorrow — Dioxus")
        .with_inner_size(dioxus::desktop::LogicalSize::new(720.0, 560.0));

    LaunchBuilder::desktop()
        .with_cfg(Config::new().with_window(window))
        .launch(App);
}

#[derive(Clone, Copy, PartialEq)]
enum Tab {
    Wallclock,
    Stopwatch,
    Synctime,
    Settings,
}

#[component]
fn App() -> Element {
    let settings = use_signal(AppSettings::load);
    use_context_provider(|| settings);

    let mut os_dark = use_signal(|| matches!(dark_light::detect(), Ok(dark_light::Mode::Dark)));

    // The system theme is only worth polling while "System" is actually
    // selected; otherwise this just idles.
    use_future(move || async move {
        loop {
            tokio::time::sleep(Duration::from_secs(2)).await;
            if settings.read().theme_mode == ThemeMode::System {
                if let Ok(mode) = tokio::task::spawn_blocking(dark_light::detect).await {
                    os_dark.set(matches!(mode, Ok(dark_light::Mode::Dark)));
                }
            }
        }
    });

    let current = settings.read().clone();
    let dark = match current.theme_mode {
        ThemeMode::Dark => true,
        ThemeMode::Light => false,
        ThemeMode::System => *os_dark.read(),
    };
    let theme_class = if dark { "theme-dark" } else { "theme-light" };
    let contrast_class = if current.contrast == Contrast::High {
        "contrast-high"
    } else {
        "contrast-normal"
    };
    let root_style = format!("font-size: {}%;", current.ui_scale);

    let mut active_tab = use_signal(|| Tab::Wallclock);
    let tab = *active_tab.read();

    rsx! {
        style { "{CSS}" }
        div { class: "app-root {theme_class} {contrast_class}", style: "{root_style}",
            header { class: "app-header", h1 { "GUI of Tomorrow — Dioxus" } }
            nav { class: "tab-bar",
                for (target , label) in [
                    (Tab::Wallclock, "Wallclock"),
                    (Tab::Stopwatch, "Stopwatch"),
                    (Tab::Synctime, "Synctime"),
                    (Tab::Settings, "Settings"),
                ]
                {
                    button {
                        class: if tab == target { "tab-button active" } else { "tab-button" },
                        onclick: move |_| active_tab.set(target),
                        "{label}"
                    }
                }
            }
            main { class: "tab-content",
                match tab {
                    Tab::Wallclock => rsx! {
                        WallclockTab {}
                    },
                    Tab::Stopwatch => rsx! {
                        StopwatchTab {}
                    },
                    Tab::Synctime => rsx! {
                        SynctimeTab {}
                    },
                    Tab::Settings => rsx! {
                        SettingsTab {}
                    },
                }
            }
        }
    }
}
