//! Resizable builder struct — a horizontal split panel.

/// A resizable split panel with a draggable handle.
#[must_use]
pub struct Resizable {
    pub(crate) initial_fraction: f32,
    pub(crate) height: f32,
    pub(crate) id_salt: Option<egui::IdSalt>,
}

impl Resizable {
    /// `initial_fraction` is the initial width ratio of the left panel (0.0..1.0).
    pub fn new(initial_fraction: f32) -> Self {
        Self {
            initial_fraction: initial_fraction.clamp(0.1, 0.9),
            height: 200.0,
            id_salt: None,
        }
    }

    /// Assigns a stable ID salt to a split whose position in the UI may change.
    pub fn id_salt(mut self, salt: impl egui::AsIdSalt) -> Self {
        self.id_salt = Some(egui::IdSalt::new(salt));
        self
    }

    /// Sets the panel height in points.
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }
}
