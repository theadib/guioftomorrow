// Qt Widgets clock demonstrator -- a reference implementation for the "GUI
// of Tomorrow" comparison. Qt was deliberately left out of the contester
// list (see ../README.md and ../OVERVIEW.md), but is built here anyway as
// a baseline: an "old guard" mature native-widget toolkit to compare the
// contesters against.

#include <QApplication>
#include <QStyleFactory>

#include "mainwindow.h"

int main(int argc, char** argv) {
    QApplication app(argc, argv);
    QCoreApplication::setOrganizationName(QStringLiteral("GuiOfTomorrow"));
    QCoreApplication::setApplicationName(QStringLiteral("QtClock"));

    // Fusion renders entirely from QPalette rather than delegating to the
    // native platform style, so the hand-rolled dark/light/high-contrast
    // palettes in theming.cpp actually take effect consistently across
    // platforms -- some native styles (e.g. Windows' default) ignore parts
    // of an app-set QPalette.
    QApplication::setStyle(QStyleFactory::create(QStringLiteral("Fusion")));

    MainWindow window;
    window.show();
    return app.exec();
}
