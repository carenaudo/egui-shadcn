//! Combobox builder struct — a searchable dropdown select.

/// A combobox: input with dropdown filter list.
#[must_use]
pub struct Combobox {
    pub(crate) items: Vec<String>,
    pub(crate) placeholder: String,
}

impl Combobox {
    pub fn new(items: Vec<String>) -> Self {
        Self {
            items,
            placeholder: "Select...".to_owned(),
        }
    }

    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = text.into();
        self
    }
}
