#include <QSettings>
#include <QTemporaryFile>
#include <QtTest>

#include "settings.h"

using namespace clockapp;

class TstSettings : public QObject {
    Q_OBJECT
private slots:
    void themeStringRoundTrip();
    void contrastStringRoundTrip();
    void unknownStringsFallBackToNullopt();
    void clampScaleKeepsRange();
    void settingsRoundTripThroughQSettings();
    void readSettingsFallsBackToDefaultsWhenEmpty();
};

void TstSettings::themeStringRoundTrip() {
    QVERIFY(theme_from_string(theme_to_string(Theme::System)) == Theme::System);
    QVERIFY(theme_from_string(theme_to_string(Theme::Light)) == Theme::Light);
    QVERIFY(theme_from_string(theme_to_string(Theme::Dark)) == Theme::Dark);
}

void TstSettings::contrastStringRoundTrip() {
    QVERIFY(contrast_from_string(contrast_to_string(Contrast::Normal)) == Contrast::Normal);
    QVERIFY(contrast_from_string(contrast_to_string(Contrast::High)) == Contrast::High);
}

void TstSettings::unknownStringsFallBackToNullopt() {
    QVERIFY(!theme_from_string(QStringLiteral("bogus")).has_value());
    QVERIFY(!contrast_from_string(QStringLiteral("bogus")).has_value());
}

void TstSettings::clampScaleKeepsRange() {
    QCOMPARE(clamp_scale(0.1), 0.5);
    QCOMPARE(clamp_scale(1.0), 1.0);
    QCOMPARE(clamp_scale(5.0), 2.0);
}

void TstSettings::settingsRoundTripThroughQSettings() {
    QTemporaryFile file;
    QVERIFY(file.open());
    const QString path = file.fileName();
    file.close();

    AppSettings original;
    original.timezone = QStringLiteral("Asia/Tokyo");
    original.theme = Theme::Dark;
    original.contrast = Contrast::High;
    original.scale = 1.5;

    {
        QSettings store(path, QSettings::IniFormat);
        write_settings(store, original);
    }

    QSettings store(path, QSettings::IniFormat);
    const AppSettings parsed = read_settings(store);
    QCOMPARE(parsed.timezone, original.timezone);
    QVERIFY(parsed.theme == original.theme);
    QVERIFY(parsed.contrast == original.contrast);
    QCOMPARE(parsed.scale, original.scale);
}

void TstSettings::readSettingsFallsBackToDefaultsWhenEmpty() {
    QTemporaryFile file;
    QVERIFY(file.open());
    const QString path = file.fileName();
    file.close();

    QSettings store(path, QSettings::IniFormat);
    const AppSettings parsed = read_settings(store);
    const AppSettings defaults;
    QCOMPARE(parsed.timezone, defaults.timezone);
    QVERIFY(parsed.theme == defaults.theme);
    QVERIFY(parsed.contrast == defaults.contrast);
    QCOMPARE(parsed.scale, defaults.scale);
}

QTEST_MAIN(TstSettings)
#include "tst_settings.moc"
