use std::fmt;

/// Config içindeki anahtar yolu: `"motion.spatial.damping"` → `["motion", "spatial", "damping"]`.
/// Nokta içeren parçalar (uygulama adları gibi) [`KeyPath::new`] ile doğrudan verilir.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyPath(Vec<String>);

impl KeyPath {
    pub fn new<S: Into<String>>(segments: impl IntoIterator<Item = S>) -> Self {
        Self(segments.into_iter().map(Into::into).collect())
    }

    /// Noktalı yazımdan (sabit anahtarlar için).
    pub fn parse(dotted: &str) -> Self {
        Self(dotted.split('.').filter(|s| !s.is_empty()).map(str::to_owned).collect())
    }

    pub fn segments(&self) -> &[String] {
        &self.0
    }

    pub fn join(&self, segment: impl Into<String>) -> Self {
        let mut v = self.0.clone();
        v.push(segment.into());
        Self(v)
    }
}

impl fmt::Display for KeyPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, s) in self.0.iter().enumerate() {
            if i > 0 {
                f.write_str(".")?;
            }
            if !s.is_empty() && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-') {
                f.write_str(s)?;
            } else {
                write!(f, "{s:?}")?;
            }
        }
        Ok(())
    }
}
