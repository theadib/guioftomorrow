#pragma once

#include <QWidget>

QT_BEGIN_NAMESPACE
class QTimer;
class QPaintEvent;
QT_END_NAMESPACE

namespace clockapp::ui {

// The circular drawing itself, isolated in its own widget so paintEvent()
// only has to reason about its own rectangle (SynctimeTab adds the
// explanatory label below it via a normal layout).
class SynctimeCanvas : public QWidget {
    Q_OBJECT
public:
    explicit SynctimeCanvas(QWidget* parent = nullptr);
    QSize sizeHint() const override;

protected:
    void paintEvent(QPaintEvent* event) override;
};

class SynctimeTab : public QWidget {
    Q_OBJECT
public:
    explicit SynctimeTab(QWidget* parent = nullptr);
};

}  // namespace clockapp::ui
