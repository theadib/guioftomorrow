use dioxus::prelude::*;

use crate::settings::{AppSettings, Contrast, ThemeMode};

#[component]
pub fn SettingsTab() -> Element {
    let mut settings = use_context::<Signal<AppSettings>>();
    let current = settings.read().clone();

    rsx! {
        div { class: "tab settings-tab",
            fieldset {
                legend { "GUI theme" }
                for (mode , label) in [
                    (ThemeMode::System, "System"),
                    (ThemeMode::Light, "Light"),
                    (ThemeMode::Dark, "Dark"),
                ]
                {
                    label { class: "radio-option",
                        input {
                            r#type: "radio",
                            name: "theme-mode",
                            checked: current.theme_mode == mode,
                            onchange: move |_| {
                                settings.with_mut(|s| {
                                    s.theme_mode = mode;
                                    let _ = s.save();
                                });
                            },
                        }
                        "{label}"
                    }
                }
            }
            fieldset {
                legend { "Contrast" }
                for (contrast , label) in [(Contrast::Normal, "Normal"), (Contrast::High, "High contrast")] {
                    label { class: "radio-option",
                        input {
                            r#type: "radio",
                            name: "contrast",
                            checked: current.contrast == contrast,
                            onchange: move |_| {
                                settings.with_mut(|s| {
                                    s.contrast = contrast;
                                    let _ = s.save();
                                });
                            },
                        }
                        "{label}"
                    }
                }
            }
            fieldset {
                legend { "UI scale — {current.ui_scale}%" }
                input {
                    r#type: "range",
                    min: "50",
                    max: "200",
                    step: "10",
                    value: "{current.ui_scale}",
                    oninput: move |evt| {
                        if let Ok(scale) = evt.value().parse::<u32>() {
                            settings.with_mut(|s| {
                                s.ui_scale = scale;
                                let _ = s.save();
                            });
                        }
                    },
                }
            }
        }
    }
}
