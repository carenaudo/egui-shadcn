//! Widget trait implementation for Label.

impl egui::Widget for super::label::Label {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let theme = crate::theme::shadcn_theme_ext::ShadcnThemeExt::shadcn_theme(ui.ctx());
        let color = if self.muted { theme.muted_foreground } else { theme.foreground };

        let galley = ui.painter().layout_no_wrap(
            self.text,
            egui::FontId::proportional(14.0),
            color,
        );

        let desired = galley.size();
        let (rect, response) = ui.allocate_exact_size(desired, egui::Sense::hover());

        if ui.is_rect_visible(rect) {
            ui.painter().galley(rect.min, galley, color);
        }

        response
    }
}
