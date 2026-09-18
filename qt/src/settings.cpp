#include "settings.h"

#include <QSettings>

#include <algorithm>

namespace clockapp {

const QStringList& available_timezones() {
    static const QStringList zones = {
        QStringLiteral("UTC"),
        QStringLiteral("Europe/Berlin"),
        QStringLiteral("Europe/London"),
        QStringLiteral("Europe/Moscow"),
        QStringLiteral("America/New_York"),
        QStringLiteral("America/Los_Angeles"),
        QStringLiteral("America/Sao_Paulo"),
        QStringLiteral("Asia/Tokyo"),
        QStringLiteral("Asia/Shanghai"),
        QStringLiteral("Asia/Kolkata"),
        QStringLiteral("Asia/Dubai"),
        QStringLiteral("Australia/Sydney"),
        QStringLiteral("Pacific/Auckland"),
    };
    return zones;
}

QString theme_to_string(Theme theme) {
    switch (theme) {
        case Theme::Light: return QStringLiteral("light");
        case Theme::Dark: return QStringLiteral("dark");
        case Theme::System: default: return QStringLiteral("system");
    }
}

std::optional<Theme> theme_from_string(const QString& text) {
    if (text == QStringLiteral("light")) return Theme::Light;
    if (text == QStringLiteral("dark")) return Theme::Dark;
    if (text == QStringLiteral("system")) return Theme::System;
    return std::nullopt;
}

QString contrast_to_string(Contrast contrast) {
    return contrast == Contrast::High ? QStringLiteral("high") : QStringLiteral("normal");
}

std::optional<Contrast> contrast_from_string(const QString& text) {
    if (text == QStringLiteral("high")) return Contrast::High;
    if (text == QStringLiteral("normal")) return Contrast::Normal;
    return std::nullopt;
}

double clamp_scale(double scale) {
    return std::clamp(scale, 0.5, 2.0);
}

void write_settings(QSettings& store, const AppSettings& settings) {
    store.setValue(QStringLiteral("timezone"), settings.timezone);
    store.setValue(QStringLiteral("theme"), theme_to_string(settings.theme));
    store.setValue(QStringLiteral("contrast"), contrast_to_string(settings.contrast));
    store.setValue(QStringLiteral("scale"), clamp_scale(settings.scale));
}

AppSettings read_settings(QSettings& store) {
    AppSettings settings;  // start from defaults; unrecognized/missing keys keep them

    const QString tz = store.value(QStringLiteral("timezone")).toString();
    if (!tz.isEmpty()) settings.timezone = tz;

    if (auto t = theme_from_string(store.value(QStringLiteral("theme")).toString())) {
        settings.theme = *t;
    }
    if (auto c = contrast_from_string(store.value(QStringLiteral("contrast")).toString())) {
        settings.contrast = *c;
    }

    bool scale_ok = false;
    const double scale = store.value(QStringLiteral("scale")).toDouble(&scale_ok);
    if (scale_ok) settings.scale = clamp_scale(scale);

    return settings;
}

namespace {
QSettings default_store() {
    return QSettings(QSettings::IniFormat, QSettings::UserScope, QStringLiteral("GuiOfTomorrow"),
                      QStringLiteral("QtClock"));
}
}  // namespace

QString settings_file_path() {
    return default_store().fileName();
}

AppSettings load_settings() {
    QSettings store = default_store();
    return read_settings(store);
}

void save_settings(const AppSettings& settings) {
    QSettings store = default_store();
    write_settings(store, settings);
    store.sync();
}

}  // namespace clockapp
