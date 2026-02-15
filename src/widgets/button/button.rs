//! shadcn-styled Button builder struct.

/// A button widget styled after shadcn/ui's Button component.
///
/// ```no_run
/// # egui::__run_test_ui(|ui| {
/// if egui_shadcn::Button::new("Click me").show(ui).clicked() {
///     // handle click
/// }
/// # });
/// ```
#[must_use]
pub struct Button<'a> {
    pub(crate) text: egui::WidgetText,
    pub(crate) variant: crate::tokens::button_variant::ButtonVariant,
    pub(crate) size: crate::tokens::component_size::ComponentSize,
    pub(crate) enabled: bool,
    pub(crate) icon: Option<crate::icons::lucide_icon::LucideIcon>,
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> Button<'a> {
    pub fn new(text: impl Into<egui::WidgetText>) -> Self {
        Self {
            text: text.into(),
            variant: crate::tokens::button_variant::ButtonVariant::Default,
            size: crate::tokens::component_size::ComponentSize::Default,
            enabled: true,
            icon: None,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Creates an icon-only button (no text).
    pub fn icon_only(icon: crate::icons::lucide_icon::LucideIcon) -> Self {
        Self {
            text: "".into(),
            variant: crate::tokens::button_variant::ButtonVariant::Default,
            size: crate::tokens::component_size::ComponentSize::Default,
            enabled: true,
            icon: Some(icon),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn variant(mut self, variant: crate::tokens::button_variant::ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: crate::tokens::component_size::ComponentSize) -> Self {
        self.size = size;
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn icon(mut self, icon: crate::icons::lucide_icon::LucideIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Convenience method: adds this widget to the Ui and returns the Response.
    pub fn show(self, ui: &mut egui::Ui) -> egui::Response {
        ui.add_enabled(self.enabled, self)
    }
}
