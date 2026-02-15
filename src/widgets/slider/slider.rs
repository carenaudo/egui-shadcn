//! shadcn-styled Slider builder struct.

/// A slider widget styled after shadcn/ui.
#[must_use]
pub struct Slider<'a> {
    pub(crate) value: &'a mut f64,
    pub(crate) range: std::ops::RangeInclusive<f64>,
    pub(crate) step: Option<f64>,
    pub(crate) width: Option<f32>,
}

impl<'a> Slider<'a> {
    pub fn new(value: &'a mut f64, range: std::ops::RangeInclusive<f64>) -> Self {
        Self {
            value,
            range,
            step: None,
            width: None,
        }
    }

    pub fn step(mut self, step: f64) -> Self {
        self.step = Some(step);
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn show(self, ui: &mut egui::Ui) -> egui::Response {
        ui.add(self)
    }
}
