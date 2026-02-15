//! Tabs builder struct — a tabbed content switcher.

/// A tabbed container: `bg-muted rounded-lg p-0.5` tab bar with content area below.
#[must_use]
pub struct Tabs {
    pub(crate) labels: Vec<String>,
}

impl Tabs {
    pub fn new(labels: Vec<String>) -> Self {
        Self { labels }
    }
}
