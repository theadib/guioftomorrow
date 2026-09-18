#include "settings_tab.h"

#include <QComboBox>
#include <QFormLayout>
#include <QHBoxLayout>
#include <QLabel>
#include <QSlider>
#include <QVBoxLayout>

namespace clockapp::ui {

SettingsTab::SettingsTab(clockapp::AppSettings& settings, QWidget* parent)
    : QWidget(parent), m_settings(settings) {
    m_themeCombo = new QComboBox(this);
    m_themeCombo->addItems({tr("System"), tr("Light"), tr("Dark")});
    m_themeCombo->setCurrentIndex(static_cast<int>(m_settings.theme));

    m_contrastCombo = new QComboBox(this);
    m_contrastCombo->addItems({tr("Normal"), tr("High contrast")});
    m_contrastCombo->setCurrentIndex(static_cast<int>(m_settings.contrast));

    m_scaleSlider = new QSlider(Qt::Horizontal, this);
    m_scaleSlider->setRange(50, 200);
    m_scaleSlider->setSingleStep(5);
    m_scaleSlider->setValue(static_cast<int>(m_settings.scale * 100));
    m_scaleLabel = new QLabel(tr("%1%").arg(m_scaleSlider->value()), this);

    auto* scaleRow = new QWidget(this);
    auto* scaleLayout = new QHBoxLayout(scaleRow);
    scaleLayout->setContentsMargins(0, 0, 0, 0);
    scaleLayout->addWidget(m_scaleSlider, 1);
    scaleLayout->addWidget(m_scaleLabel);

    auto* form = new QFormLayout();
    form->addRow(tr("GUI theme"), m_themeCombo);
    form->addRow(tr("Contrast"), m_contrastCombo);
    form->addRow(tr("UI scale"), scaleRow);

    auto* layout = new QVBoxLayout(this);
    layout->addLayout(form);
    layout->addWidget(new QLabel(tr("Settings are saved automatically and applied live."), this));
    layout->addStretch();

    connect(m_themeCombo, &QComboBox::currentIndexChanged, this, &SettingsTab::onThemeChanged);
    connect(m_contrastCombo, &QComboBox::currentIndexChanged, this,
            &SettingsTab::onContrastChanged);
    connect(m_scaleSlider, &QSlider::valueChanged, this, &SettingsTab::onScaleChanged);
}

void SettingsTab::onThemeChanged(int index) {
    m_settings.theme = static_cast<clockapp::Theme>(index);
    emit settingsChanged();
}

void SettingsTab::onContrastChanged(int index) {
    m_settings.contrast = static_cast<clockapp::Contrast>(index);
    emit settingsChanged();
}

void SettingsTab::onScaleChanged(int value) {
    m_settings.scale = clockapp::clamp_scale(value / 100.0);
    m_scaleLabel->setText(tr("%1%").arg(value));
    emit settingsChanged();
}

}  // namespace clockapp::ui
