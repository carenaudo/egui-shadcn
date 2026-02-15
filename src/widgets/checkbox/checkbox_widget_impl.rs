//! Widget trait implementation for Checkbox.

impl egui::Widget for super::checkbox::Checkbox<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let theme = crate::theme::shadcn_theme_ext::ShadcnThemeExt::shadcn_theme(ui.ctx());

        let box_size = 16.0;
        let corner_radius = 4.0;
        let spacing = 8.0;

        // Layout: box + optional label
        let label_galley = self.label.map(|l| {
            ui.painter().layout_no_wrap(
                l.text().to_owned(),
                egui::FontId::proportional(14.0),
                theme.foreground,
            )
        });

        let label_width = label_galley.as_ref().map_or(0.0, |g| g.size().x + spacing);
        let desired = egui::vec2(box_size + label_width, box_size.max(20.0));

        let (rect, response) = ui.allocate_exact_size(desired, egui::Sense::click());

        if response.clicked() {
            *self.checked = !*self.checked;
            response.ctx.request_repaint();
        }

        let anim_t = ui.ctx().animate_bool_responsive(response.id, *self.checked);

        if ui.is_rect_visible(rect) {
            let style = super::checkbox_style::resolve_checkbox_style(
                &theme,
                *self.checked,
                response.hovered(),
                !ui.is_enabled(),
            );

            let painter = ui.painter();
            let box_rect = egui::Rect::from_min_size(
                egui::pos2(rect.min.x, rect.center().y - box_size / 2.0),
                egui::vec2(box_size, box_size),
            );

            let cr = egui::CornerRadius::same(corner_radius as u8);
            painter.rect_filled(box_rect, cr, style.box_bg);
            painter.rect_stroke(
                box_rect,
                cr,
                egui::Stroke::new(1.0, style.box_border),
                egui::epaint::StrokeKind::Inside,
            );

            // Checkmark
            super::paint_check_icon::paint_check_icon(
                painter,
                box_rect,
                style.check_color,
                anim_t,
            );

            // Label
            if let Some(galley) = label_galley {
                let text_pos = egui::pos2(
                    box_rect.max.x + spacing,
                    rect.center().y - galley.size().y / 2.0,
                );
                painter.galley(text_pos, galley, style.text_color);
            }

            // Focus ring
            if response.has_focus() {
                crate::paint::paint_focus_ring::paint_focus_ring(
                    painter,
                    box_rect,
                    corner_radius,
                    theme.ring,
                );
            }
        }

        response
    }
}
