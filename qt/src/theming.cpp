#include "theming.h"

#include <QApplication>
#include <QColor>
#include <QGuiApplication>
#include <QPalette>
#include <QStyleHints>
#include <QtGlobal>

namespace clockapp::ui {

namespace {

QPalette base_palette(bool dark) {
    QPalette pal;
    if (dark) {
        pal.setColor(QPalette::Window, QColor(45, 45, 48));
        pal.setColor(QPalette::WindowText, QColor(225, 225, 225));
        pal.setColor(QPalette::Base, QColor(30, 30, 32));
        pal.setColor(QPalette::AlternateBase, QColor(45, 45, 48));
        pal.setColor(QPalette::ToolTipBase, QColor(45, 45, 48));
        pal.setColor(QPalette::ToolTipText, QColor(225, 225, 225));
        pal.setColor(QPalette::Text, QColor(225, 225, 225));
        pal.setColor(QPalette::Button, QColor(60, 60, 64));
        pal.setColor(QPalette::ButtonText, QColor(225, 225, 225));
        pal.setColor(QPalette::Highlight, QColor(70, 130, 220));
        pal.setColor(QPalette::HighlightedText, Qt::white);
        pal.setColor(QPalette::Mid, QColor(90, 90, 95));
        pal.setColor(QPalette::Disabled, QPalette::Text, QColor(120, 120, 120));
        pal.setColor(QPalette::Disabled, QPalette::WindowText, QColor(120, 120, 120));
    } else {
        pal.setColor(QPalette::Highlight, QColor(50, 110, 200));
        pal.setColor(QPalette::HighlightedText, Qt::white);
        pal.setColor(QPalette::Mid, QColor(160, 160, 160));
    }
    return pal;
}

// Dear ImGui/Qt both lack a real "contrast mode" concept to hook into (see
// ../OVERVIEW.md); this hand-codes a handful of high-contrast palette
// entries plus thicker widget borders, the same minimal, demo-scoped
// tradeoff the imgui contester made rather than a general-purpose
// contrast transform over the whole palette.
void increase_contrast(QPalette& pal, bool dark) {
    if (dark) {
        pal.setColor(QPalette::WindowText, Qt::white);
        pal.setColor(QPalette::Text, Qt::white);
        pal.setColor(QPalette::ButtonText, Qt::white);
        pal.setColor(QPalette::Window, Qt::black);
        pal.setColor(QPalette::Base, Qt::black);
        pal.setColor(QPalette::Button, QColor(20, 20, 20));
    } else {
        pal.setColor(QPalette::WindowText, Qt::black);
        pal.setColor(QPalette::Text, Qt::black);
        pal.setColor(QPalette::ButtonText, Qt::black);
        pal.setColor(QPalette::Window, Qt::white);
        pal.setColor(QPalette::Base, Qt::white);
        pal.setColor(QPalette::Button, QColor(235, 235, 235));
    }
}

}  // namespace

void apply_style(const clockapp::AppSettings& settings, const QFont& base_font) {
    auto* app = qobject_cast<QApplication*>(QApplication::instance());
    if (!app) return;

    const bool system_is_dark =
        QGuiApplication::styleHints()->colorScheme() == Qt::ColorScheme::Dark;
    const bool use_dark = settings.theme == clockapp::Theme::Dark ||
                           (settings.theme == clockapp::Theme::System && system_is_dark);

    QPalette palette = base_palette(use_dark);
    QString stylesheet;
    if (settings.contrast == clockapp::Contrast::High) {
        increase_contrast(palette, use_dark);
        stylesheet += QStringLiteral(
                          "QPushButton, QComboBox, QLineEdit, QSlider::groove:horizontal "
                          "{ border: 2px solid %1; }")
                          .arg(use_dark ? QStringLiteral("white") : QStringLiteral("black"));
    }
    app->setPalette(palette);

    const double scale = clockapp::clamp_scale(settings.scale);
    QFont font = base_font;
    font.setPointSizeF(base_font.pointSizeF() * scale);
    app->setFont(font);

    const int padding = qRound(4 * scale);
    stylesheet += QStringLiteral("QPushButton, QComboBox { padding: %1px %2px; }")
                      .arg(padding)
                      .arg(padding * 2);
    app->setStyleSheet(stylesheet);
}

}  // namespace clockapp::ui
