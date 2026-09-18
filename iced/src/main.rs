mod settings;
mod tabs;

use std::time::Duration;

use chrono::Utc;
use iced::widget::{button, column, row, text};
use iced::{Center, Element, Fill, Size, Subscription, Task, Theme};

use settings::{AppSettings, Contrast, ThemeMode};

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("GUI of Tomorrow — Iced")
        .theme(App::theme)
        .subscription(App::subscription)
        .scale_factor(App::scale_factor)
        .window_size(Size::new(720.0, 560.0))
        .centered()
        .antialiasing(true)
        .run()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tab {
    Wallclock,
    Stopwatch,
    Synctime,
    Settings,
}

const TABS: [(Tab, &str); 4] = [
    (Tab::Wallclock, "Wallclock"),
    (Tab::Stopwatch, "Stopwatch"),
    (Tab::Synctime, "Synctime"),
    (Tab::Settings, "Settings"),
];

struct App {
    settings: AppSettings,
    active_tab: Tab,
    now_utc: chrono::DateTime<Utc>,
    os_dark: bool,
    stopwatch: tabs::stopwatch::State,
}

#[derive(Debug, Clone)]
enum Message {
    TabSelected(Tab),
    Tick,
    SlowTick,
    OsThemeDetected(bool),
    Wallclock(tabs::wallclock::Message),
    Stopwatch(tabs::stopwatch::Message),
    Synctime(tabs::synctime::Message),
    Settings(tabs::settings_tab::Message),
}

impl App {
    fn new() -> Self {
        let settings = AppSettings::load();
        let os_dark = matches!(dark_light::detect(), Ok(dark_light::Mode::Dark));
        Self {
            settings,
            active_tab: Tab::Wallclock,
            now_utc: Utc::now(),
            os_dark,
            stopwatch: tabs::stopwatch::State::default(),
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::TabSelected(tab) => {
                self.active_tab = tab;
                Task::none()
            }
            Message::Tick => {
                self.now_utc = Utc::now();
                Task::none()
            }
            Message::SlowTick => {
                // The system theme is only worth polling while "System" is
                // actually selected; the subscription below already gates
                // this, but the check is kept here too in case that ever
                // changes.
                if self.settings.theme_mode == ThemeMode::System {
                    Task::perform(detect_os_theme(), Message::OsThemeDetected)
                } else {
                    Task::none()
                }
            }
            Message::OsThemeDetected(dark) => {
                self.os_dark = dark;
                Task::none()
            }
            Message::Wallclock(tabs::wallclock::Message::TimezoneSelected(zone)) => {
                self.settings.timezone = zone.to_string();
                let _ = self.settings.save();
                Task::none()
            }
            Message::Stopwatch(message) => {
                tabs::stopwatch::update(&mut self.stopwatch, message);
                Task::none()
            }
            Message::Synctime(message) => match message {},
            Message::Settings(message) => {
                tabs::settings_tab::update(&mut self.settings, message);
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let tab_bar = row(TABS.into_iter().map(|(tab, label)| {
            let active = tab == self.active_tab;
            button(text(label))
                .on_press(Message::TabSelected(tab))
                .style(move |theme: &Theme, status| {
                    if active {
                        button::primary(theme, status)
                    } else {
                        button::secondary(theme, status)
                    }
                })
                .into()
        }))
        .spacing(8);

        let content = match self.active_tab {
            Tab::Wallclock => {
                tabs::wallclock::view(self.now_utc, &self.settings.timezone).map(Message::Wallclock)
            }
            Tab::Stopwatch => tabs::stopwatch::view(&self.stopwatch).map(Message::Stopwatch),
            Tab::Synctime => tabs::synctime::view(self.now_utc.with_timezone(&chrono::Local))
                .map(Message::Synctime),
            Tab::Settings => tabs::settings_tab::view(&self.settings).map(Message::Settings),
        };

        column![
            text("GUI of Tomorrow — Iced").size(22),
            tab_bar,
            column![content].align_x(Center).width(Fill),
        ]
        .spacing(20)
        .padding(20)
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        let tick = iced::time::every(Duration::from_millis(33)).map(|_| Message::Tick);

        if self.settings.theme_mode == ThemeMode::System {
            let slow_tick = iced::time::every(Duration::from_secs(2)).map(|_| Message::SlowTick);
            Subscription::batch([tick, slow_tick])
        } else {
            tick
        }
    }

    fn theme(&self) -> Theme {
        let dark = match self.settings.theme_mode {
            ThemeMode::Dark => true,
            ThemeMode::Light => false,
            ThemeMode::System => self.os_dark,
        };
        match (dark, self.settings.contrast) {
            (false, Contrast::Normal) => Theme::Light,
            (true, Contrast::Normal) => Theme::Dark,
            (false, Contrast::High) => Theme::custom(
                "High Contrast Light".to_string(),
                theme::HIGH_CONTRAST_LIGHT,
            ),
            (true, Contrast::High) => {
                Theme::custom("High Contrast Dark".to_string(), theme::HIGH_CONTRAST_DARK)
            }
        }
    }

    fn scale_factor(&self) -> f32 {
        self.settings.ui_scale as f32 / 100.0
    }
}

async fn detect_os_theme() -> bool {
    tokio::task::spawn_blocking(|| matches!(dark_light::detect(), Ok(dark_light::Mode::Dark)))
        .await
        .unwrap_or(false)
}

mod theme {
    use iced::theme::Palette;
    use iced::Color;

    pub const HIGH_CONTRAST_LIGHT: Palette = Palette {
        background: Color::WHITE,
        text: Color::BLACK,
        primary: Color::from_rgb(0.0, 0.0, 0.8),
        success: Color::from_rgb(0.0, 0.4, 0.0),
        warning: Color::from_rgb(0.6, 0.3, 0.0),
        danger: Color::from_rgb(0.7, 0.0, 0.0),
    };

    pub const HIGH_CONTRAST_DARK: Palette = Palette {
        background: Color::BLACK,
        text: Color::WHITE,
        primary: Color::from_rgb(0.4, 0.8, 1.0),
        success: Color::from_rgb(0.4, 1.0, 0.4),
        warning: Color::from_rgb(1.0, 0.8, 0.2),
        danger: Color::from_rgb(1.0, 0.4, 0.4),
    };
}
