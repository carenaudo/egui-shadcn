//! Label builder struct — styled text following shadcn/ui's `text-sm font-medium`.

/// A styled text label: `text-sm font-medium`.
#[must_use]
pub struct Label {
    pub(crate) text: String,
    pub(crate) muted: bool,
}

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            muted: false,
        }
    }

    /// Use muted foreground color.
    pub fn muted(mut self) -> Self {
        self.muted = true;
        self
    }

    pub fn show(self, ui: &mut egui::Ui) -> egui::Response {
        ui.add(self)
    }
}
