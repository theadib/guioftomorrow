#include "synctime_tab.h"

#include <QDateTime>
#include <QFont>
#include <QLabel>
#include <QPainter>
#include <QTimer>
#include <QVBoxLayout>
#include <QtGlobal>

#include "synctime.h"

namespace clockapp::ui {

SynctimeCanvas::SynctimeCanvas(QWidget* parent) : QWidget(parent) {
    auto* timer = new QTimer(this);
    // Redrawn on a fixed 60fps timer -- there's no separate "is anything
    // dirty" tracking to manage, the arcs just always reflect the current
    // instant whenever paintEvent() next runs, mirroring the imgui
    // contester's every-frame immediate-mode redraw.
    connect(timer, &QTimer::timeout, this, QOverload<>::of(&QWidget::update));
    timer->start(16);
}

QSize SynctimeCanvas::sizeHint() const {
    return QSize(280, 280);
}

void SynctimeCanvas::paintEvent(QPaintEvent*) {
    using clockapp::synctime::angle_for_fraction;
    using clockapp::synctime::fraction_of_period;
    using clockapp::synctime::point_on_circle;

    QPainter painter(this);
    painter.setRenderHint(QPainter::Antialiasing);

    const QDateTime local_now = QDateTime::currentDateTime();
    const double seconds_since_epoch =
        QDateTime::currentDateTimeUtc().toMSecsSinceEpoch() / 1000.0;

    const double side = qMin(width(), height());
    const double outer_radius = side * 0.42;
    const double inner_radius = side * 0.29;
    const QPointF center(width() / 2.0, height() / 2.0);

    QPen ring_pen(palette().color(QPalette::Mid));
    ring_pen.setWidthF(1.5);
    painter.setPen(ring_pen);
    painter.drawEllipse(center, outer_radius, outer_radius);
    painter.drawEllipse(center, inner_radius, inner_radius);

    // Quarter tick marks on the outer ring, placed with the pure
    // point_on_circle() geometry helper (unit-tested separately, shared
    // with the imgui contester's synctime.h math).
    QPen tick_pen(palette().color(QPalette::WindowText));
    tick_pen.setWidthF(2.0);
    painter.setPen(tick_pen);
    for (int i = 0; i < 4; ++i) {
        const double angle = angle_for_fraction(i / 4.0);
        const auto a = point_on_circle({center.x(), center.y()}, outer_radius + 6.0, angle);
        const auto b = point_on_circle({center.x(), center.y()}, outer_radius - 6.0, angle);
        painter.drawLine(QPointF(a.x, a.y), QPointF(b.x, b.y));
    }

    const double sec_fraction = fraction_of_period(seconds_since_epoch, 1.0);
    const double min_fraction = fraction_of_period(seconds_since_epoch, 60.0);

    const QRectF outer_rect(center.x() - outer_radius, center.y() - outer_radius,
                             outer_radius * 2, outer_radius * 2);
    const QRectF inner_rect(center.x() - inner_radius, center.y() - inner_radius,
                             inner_radius * 2, inner_radius * 2);

    // QPainter::drawArc() angles are in 1/16-degree units, start at 3
    // o'clock and grow counter-clockwise -- so a clockwise sweep starting
    // at 12 o'clock (90*16) needs a negative span.
    QPen sec_pen(QColor(70, 160, 255));
    sec_pen.setWidthF(4.0);
    painter.setPen(sec_pen);
    painter.drawArc(outer_rect, 90 * 16, -qRound(sec_fraction * 360.0 * 16));

    QPen min_pen(QColor(255, 170, 60));
    min_pen.setWidthF(4.0);
    painter.setPen(min_pen);
    painter.drawArc(inner_rect, 90 * 16, -qRound(min_fraction * 360.0 * 16));

    QFont center_font = painter.font();
    center_font.setPointSizeF(center_font.pointSizeF() * 1.3);
    painter.setFont(center_font);
    painter.setPen(palette().color(QPalette::WindowText));
    painter.drawText(QRectF(center.x() - inner_radius, center.y() - 12, inner_radius * 2, 24),
                      Qt::AlignCenter, local_now.toString(QStringLiteral("HH:mm:ss")));
}

SynctimeTab::SynctimeTab(QWidget* parent) : QWidget(parent) {
    auto* canvas = new SynctimeCanvas(this);

    auto* description = new QLabel(
        tr("Outer arc: one full rotation per second. Inner arc: one full rotation per minute. "
           "Run this on two machines to visually compare their clocks."),
        this);
    description->setWordWrap(true);

    auto* layout = new QVBoxLayout(this);
    layout->addWidget(canvas, 1, Qt::AlignHCenter);
    layout->addWidget(description);
}

}  // namespace clockapp::ui
