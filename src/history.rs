//! Geri al / yinele. Kayıtlar anahtar tabanlıdır: geri almak o anahtarı önceki değerine
//! yazmaktır (aynı yazım yolundan). Bkz. `docs/ARCHITECTURE.md` §3 "Geri al".

use std::time::{Duration, Instant};

use nuxut::i18n::t;
use toml_edit::Value;

use crate::store::{FileId, KeyPath};

/// Tutulan en çok kayıt.
pub const MAX_ENTRIES: usize = 100;
/// Aynı anahtara bu süre içindeki ardışık değişiklikler tek kayıtta birleşir.
pub const COALESCE: Duration = Duration::from_secs(1);

#[derive(Debug, Clone)]
pub struct Change {
    pub file: FileId,
    pub path: KeyPath,
    pub before: Option<Value>,
    pub after: Option<Value>,
}

/// Bir kullanıcı eylemi: bir ya da birkaç anahtar (ör. "renkleri duvar kağıdından al").
#[derive(Debug, Clone)]
pub struct Entry {
    pub label: String,
    pub changes: Vec<Change>,
    pub at: Instant,
}

impl Entry {
    /// Geri alınırken yazılacaklar (ters sırada).
    pub fn undo_writes(&self) -> impl Iterator<Item = (FileId, &KeyPath, Option<Value>)> {
        self.changes.iter().rev().map(|c| (c.file, &c.path, c.before.clone()))
    }

    pub fn redo_writes(&self) -> impl Iterator<Item = (FileId, &KeyPath, Option<Value>)> {
        self.changes.iter().map(|c| (c.file, &c.path, c.after.clone()))
    }

    fn single_key(&self) -> Option<(FileId, &KeyPath)> {
        match self.changes.as_slice() {
            [c] => Some((c.file, &c.path)),
            _ => None,
        }
    }
}

#[derive(Debug, Default)]
pub struct History {
    undo: Vec<Entry>,
    redo: Vec<Entry>,
}

impl History {
    pub fn new() -> Self {
        Self::default()
    }

    /// Yeni bir eylem kaydeder. Tek anahtarlı ve son kayıtla aynı anahtarda, `COALESCE`
    /// içinde ise birleşir (yalnızca `after` güncellenir). Değişmeyen eylem kaydedilmez.
    pub fn record(&mut self, label: impl Into<String>, changes: Vec<Change>, now: Instant) {
        let changes: Vec<Change> = changes.into_iter().filter(|c| !values_equal(c.before.as_ref(), c.after.as_ref())).collect();
        if changes.is_empty() {
            return;
        }
        self.redo.clear();
        if let [c] = changes.as_slice()
            && let Some(last) = self.undo.last_mut()
            && last.single_key() == Some((c.file, &c.path))
            && now.saturating_duration_since(last.at) < COALESCE
        {
            last.changes[0].after = c.after.clone();
            last.at = now;
            if values_equal(last.changes[0].before.as_ref(), last.changes[0].after.as_ref()) {
                // Başladığı yere döndü: kayıt anlamsız.
                self.undo.pop();
            }
            return;
        }
        self.undo.push(Entry { label: label.into(), changes, at: now });
        if self.undo.len() > MAX_ENTRIES {
            self.undo.remove(0);
        }
    }

    /// Son eylemi geri alma kaydını verir (yazım çağıranındır) ve yinele yığınına taşır.
    pub fn undo(&mut self) -> Option<&Entry> {
        let e = self.undo.pop()?;
        self.redo.push(e);
        self.redo.last()
    }

    pub fn redo(&mut self) -> Option<&Entry> {
        let e = self.redo.pop()?;
        self.undo.push(e);
        self.undo.last()
    }

    /// Son değişiklikler, en yenisi önce.
    pub fn recent(&self, n: usize) -> impl Iterator<Item = (usize, &Entry)> {
        self.undo.iter().enumerate().rev().take(n)
    }

    /// Ortadaki bir kaydı tek başına geri alır ("Son değişiklikler" menüsü). Kayıt silinir;
    /// tersini yeni bir kayıt olarak ekler ki bu da geri alınabilsin.
    pub fn revert(&mut self, index: usize, now: Instant) -> Option<Entry> {
        if index >= self.undo.len() {
            return None;
        }
        let e = self.undo.remove(index);
        let inverse = Entry {
            label: t!("settings-history-reverted", label = e.label),
            changes: e
                .changes
                .iter()
                .rev()
                .map(|c| Change { file: c.file, path: c.path.clone(), before: c.after.clone(), after: c.before.clone() })
                .collect(),
            at: now,
        };
        self.redo.clear();
        self.undo.push(inverse.clone());
        Some(inverse)
    }

    pub fn can_undo(&self) -> bool {
        !self.undo.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo.is_empty()
    }

    /// Son kaydı birleştirmeye kapatır (ör. sürükleme bitti): sonraki değişiklik yeni kayıt olur.
    pub fn seal(&mut self) {
        if let Some(last) = self.undo.last_mut() {
            last.at = last.at.checked_sub(COALESCE).unwrap_or(last.at);
        }
    }
}

fn values_equal(a: Option<&Value>, b: Option<&Value>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(a), Some(b)) => a.to_string().trim() == b.to_string().trim(),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ch(path: &str, before: Option<f64>, after: Option<f64>) -> Change {
        Change { file: FileId::Framework, path: KeyPath::parse(path), before: before.map(Value::from), after: after.map(Value::from) }
    }

    #[test]
    fn coalesces_drag_on_same_key() {
        let t = Instant::now();
        let mut h = History::new();
        h.record("Hız", vec![ch("motion.speed", Some(1.0), Some(1.1))], t);
        h.record("Hız", vec![ch("motion.speed", Some(1.1), Some(1.4))], t + Duration::from_millis(100));
        h.record("Hız", vec![ch("motion.speed", Some(1.4), Some(1.8))], t + Duration::from_millis(900));
        let e = h.undo().unwrap();
        assert_eq!(e.changes.len(), 1);
        assert_eq!(e.changes[0].before.as_ref().and_then(Value::as_float), Some(1.0));
        assert_eq!(e.changes[0].after.as_ref().and_then(Value::as_float), Some(1.8));
        assert!(!h.can_undo());
    }

    #[test]
    fn separate_keys_or_late_changes_are_separate_entries() {
        let t = Instant::now();
        let mut h = History::new();
        h.record("Hız", vec![ch("motion.speed", Some(1.0), Some(2.0))], t);
        h.record("Bulanıklık", vec![ch("shadow.blur", None, Some(20.0))], t);
        h.record("Hız", vec![ch("motion.speed", Some(2.0), Some(3.0))], t + Duration::from_secs(2));
        assert_eq!(h.recent(10).count(), 3);
    }

    #[test]
    fn returning_to_start_drops_entry() {
        let t = Instant::now();
        let mut h = History::new();
        h.record("Hız", vec![ch("motion.speed", Some(1.0), Some(2.0))], t);
        h.record("Hız", vec![ch("motion.speed", Some(2.0), Some(1.0))], t);
        assert!(!h.can_undo());
    }

    #[test]
    fn undo_redo_cycle() {
        let t = Instant::now();
        let mut h = History::new();
        h.record("Hız", vec![ch("motion.speed", None, Some(2.0))], t);
        let w: Vec<_> = h.undo().unwrap().undo_writes().map(|(_, p, v)| (p.to_string(), v.is_none())).collect();
        assert_eq!(w, vec![("motion.speed".to_owned(), true)]);
        assert!(h.can_redo());
        let w: Vec<_> = h.redo().unwrap().redo_writes().map(|(_, _, v)| v.and_then(|v| v.as_float())).collect();
        assert_eq!(w, vec![Some(2.0)]);
        // Yeni kayıt yinele yığınını temizler.
        h.undo();
        h.record("Bulanıklık", vec![ch("shadow.blur", None, Some(3.0))], t);
        assert!(!h.can_redo());
    }

    #[test]
    fn group_undo_is_reverse_order() {
        let t = Instant::now();
        let mut h = History::new();
        h.record("Duvar kağıdı renkleri", vec![ch("a", None, Some(1.0)), ch("b", None, Some(2.0))], t);
        let paths: Vec<_> = h.undo().unwrap().undo_writes().map(|(_, p, _)| p.to_string()).collect();
        assert_eq!(paths, ["b", "a"]);
    }

    #[test]
    fn revert_middle_entry_adds_inverse() {
        crate::i18n::use_turkish_for_tests();
        let t = Instant::now();
        let mut h = History::new();
        h.record("A", vec![ch("a", Some(0.0), Some(1.0))], t);
        h.record("B", vec![ch("b", Some(0.0), Some(1.0))], t);
        let inv = h.revert(0, t).unwrap();
        assert_eq!(inv.changes[0].after.as_ref().and_then(Value::as_float), Some(0.0));
        let labels: Vec<_> = h.recent(10).map(|(_, e)| e.label.clone()).collect();
        assert_eq!(labels, ["A (geri alındı)", "B"]);
    }

    #[test]
    fn capped() {
        let t = Instant::now();
        let mut h = History::new();
        for i in 0..(MAX_ENTRIES + 10) {
            h.record("x", vec![ch(&format!("k{i}"), None, Some(1.0))], t);
        }
        assert_eq!(h.recent(usize::MAX).count(), MAX_ENTRIES);
    }
}
