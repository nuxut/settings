//! Eksik paket düzeltmesinin düğmesi (Görünüm durum satırı ve ikon/imleç ızgarası ortak): resmî
//! depo paketi için "Kur" (kök yardımcının `packages` alanı, yönetici onayı;
//! [`crate::services::packages`]), AUR paketi için kopyalanacak yardımcı komutu. Kullanıcıya kök
//! komutu yazdırılmaz (kullanıcı kararı 2026-10-17).

use std::time::Instant;

use nuxut::i18n::t;
use nuxut::ui::input::{CursorShape, KeyEvent, PointerEvent};
use nuxut::ui::{Button, ButtonVariant, Icon, IconName, Response, Ui};
use nuxut::{Point, Rect, Size, TextSystem, Theme};

use crate::block::{Action, Env};
use crate::services::look::Fix;
use crate::services::packages;

#[derive(Debug)]
pub struct FixButton {
    copy: Button,
    install: Button,
    fix: Option<Fix>,
}

impl FixButton {
    /// `copy_label`: kopyalama düğmesinin yerdeki adı ("Komutu kopyala", "Kurulum komutunu kopyala").
    pub fn new(theme: &Theme, copy_label: String) -> Self {
        Self {
            copy: Button::new(theme, ButtonVariant::Text).with_icon(theme, Icon::named(IconName::Copy)).with_label(theme, copy_label),
            install: Button::new(theme, ButtonVariant::Text).with_icon(theme, Icon::named(IconName::Download)).with_label(theme, t!("settings-packages-install")),
            fix: None,
        }
    }

    #[cfg(test)]
    pub fn fix(&self) -> Option<&Fix> {
        self.fix.as_ref()
    }

    pub fn is_some(&self) -> bool {
        self.fix.is_some()
    }

    /// Düzeltmeyi ve kurulum düğmesinin durumunu (sürüyorsa "Kuruluyor…", basılamaz) günceller.
    pub fn set_fix(&mut self, fix: Option<Fix>, now: Instant) {
        self.fix = fix;
        let running = packages::running();
        self.install.set_label(&if running { t!("settings-packages-installing") } else { t!("settings-packages-install") });
        self.install.set_enabled(!running, now);
    }

    fn active(&self) -> &Button {
        match self.fix {
            Some(Fix::Install(_)) => &self.install,
            _ => &self.copy,
        }
    }

    fn active_mut(&mut self) -> &mut Button {
        match self.fix {
            Some(Fix::Install(_)) => &mut self.install,
            _ => &mut self.copy,
        }
    }

    pub fn preferred_size(&mut self, text: &mut TextSystem, theme: &Theme) -> Size {
        self.active_mut().preferred_size(text, theme)
    }

    pub fn set_rect(&mut self, rect: Rect) {
        self.copy.set_rect(rect);
        self.install.set_rect(rect);
    }

    pub fn rect(&self) -> Rect {
        self.active().rect()
    }

    pub fn pointer(&mut self, e: &PointerEvent, now: Instant) -> Response {
        if self.fix.is_none() {
            return Response::default();
        }
        self.active_mut().pointer(e, now)
    }

    pub fn key(&mut self, e: &KeyEvent, now: Instant) -> Response {
        if self.fix.is_none() {
            return Response::default();
        }
        self.active_mut().key(e, now)
    }

    /// Düğmeye basıldı: komutu panoya kopyalar ya da kurulumu başlatır.
    pub fn activate(&self, env: &mut Env) {
        match &self.fix {
            Some(Fix::Copy(cmd)) => env.push(Action::Copy(cmd.clone())),
            Some(Fix::Install(names)) => packages::start(names),
            None => {}
        }
    }

    pub fn cursor(&self, p: Point) -> Option<CursorShape> {
        self.fix.as_ref().and_then(|_| self.active().cursor(p))
    }

    pub fn set_focus_visible(&mut self, on: bool) {
        self.copy.set_focus_visible(on);
        self.install.set_focus_visible(on);
    }

    pub fn set_theme(&mut self, theme: &Theme, now: Instant) {
        self.copy.set_theme(theme, now);
        self.install.set_theme(theme, now);
    }

    pub fn draw(&mut self, ui: &mut Ui) {
        if self.fix.is_some() {
            self.active_mut().draw(ui);
        }
    }
}
