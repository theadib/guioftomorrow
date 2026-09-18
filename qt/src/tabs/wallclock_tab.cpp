#include "wallclock_tab.h"

#include <QComboBox>
#include <QDateTime>
#include <QFont>
#include <QLabel>
#include <QTimer>
#include <QVBoxLayout>

#include "wallclock.h"

namespace clockapp::ui {

WallclockTab::WallclockTab(clockapp::AppSettings& settings, QWidget* parent)
    : QWidget(parent), m_settings(settings) {
    m_clockLabel = new QLabel(this);
    QFont clockFont = m_clockLabel->font();
    clockFont.setPointSizeF(clockFont.pointSizeF() * 2.5);
    m_clockLabel->setFont(clockFont);

    m_dateLabel = new QLabel(this);

    m_timezoneCombo = new QComboBox(this);
    m_timezoneCombo->addItems(clockapp::available_timezones());
    m_timezoneCombo->setCurrentText(m_settings.timezone);

    auto* layout = new QVBoxLayout(this);
    layout->addSpacing(12);
    layout->addWidget(m_clockLabel);
    layout->addWidget(m_dateLabel);
    layout->addSpacing(12);
    layout->addWidget(new QLabel(tr("Timezone"), this));
    layout->addWidget(m_timezoneCombo);
    layout->addStretch();

    connect(m_timezoneCombo, &QComboBox::currentTextChanged, this,
            &WallclockTab::onTimezoneChanged);

    auto* timer = new QTimer(this);
    connect(timer, &QTimer::timeout, this, &WallclockTab::tick);
    timer->start(500);
    tick();
}

void WallclockTab::tick() {
    const QDateTime utc_now = QDateTime::currentDateTimeUtc();
    const QDateTime local = clockapp::wallclock::time_in_zone(m_settings.timezone, utc_now);
    // Blink the ':' separators on/off once a second (500ms visible, 500ms hidden).
    const bool colon_visible = (utc_now.toSecsSinceEpoch() % 2) == 0;
    m_clockLabel->setText(clockapp::wallclock::format_clock(local, colon_visible));
    m_dateLabel->setText(clockapp::wallclock::format_date(local));
}

void WallclockTab::onTimezoneChanged(const QString& zone) {
    if (m_settings.timezone != zone) {
        m_settings.timezone = zone;
        emit settingsChanged();
    }
}

}  // namespace clockapp::ui
