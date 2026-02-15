//! Widget trait implementation for Slider.

impl egui::Widget for super::slider::Slider<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let theme = crate::theme::shadcn_theme_ext::ShadcnThemeExt::shadcn_theme(ui.ctx());
        let style = super::slider_style::resolve_slider_style(&theme);

        let track_height: f32 = 4.0;   // h-1 (Nova)
        let handle_radius = 6.0;        // size-3 = 12px diameter
        let total_height = handle_radius * 2.0 + 4.0; // extra touch area
        let width = self.width.unwrap_or(ui.available_width().min(200.0));

        let desired = egui::vec2(width, total_height);
        let (rect, response) = ui.allocate_exact_size(desired, egui::Sense::click_and_drag());

        let range_start = *self.range.start();
        let range_end = *self.range.end();
        let range_span = range_end - range_start;

        // Handle drag
        if response.dragged() || response.clicked() {
            if let Some(pos) = response.interact_pointer_pos() {
                let usable_min = rect.min.x + handle_radius;
                let usable_max = rect.max.x - handle_radius;
                let t = ((pos.x - usable_min) / (usable_max - usable_min)).clamp(0.0, 1.0);
                let mut new_val = range_start + t as f64 * range_span;

                if let Some(step) = self.step {
                    new_val = (new_val / step).round() * step;
                }

                *self.value = new_val.clamp(range_start, range_end);
            }
        }

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();

            let track_y = rect.center().y;
            let usable_min = rect.min.x + handle_radius;
            let usable_max = rect.max.x - handle_radius;

            // Normalized position
            let t = if range_span > 0.0 {
                ((*self.value - range_start) / range_span) as f32
            } else {
                0.0
            };
            let handle_x = usable_min + (usable_max - usable_min) * t;

            let track_cr = (track_height / 2.0).round().min(255.0) as u8;

            // Track background
            let track_rect = egui::Rect::from_min_max(
                egui::pos2(usable_min, track_y - track_height / 2.0),
                egui::pos2(usable_max, track_y + track_height / 2.0),
            );
            painter.rect_filled(
                track_rect,
                egui::CornerRadius::same(track_cr),
                style.track_color,
            );

            // Fill (left of handle)
            let fill_rect = egui::Rect::from_min_max(
                egui::pos2(usable_min, track_y - track_height / 2.0),
                egui::pos2(handle_x, track_y + track_height / 2.0),
            );
            painter.rect_filled(
                fill_rect,
                egui::CornerRadius::same(track_cr),
                style.fill_color,
            );

            // Handle
            let handle_center = egui::pos2(handle_x, track_y);
            painter.circle_filled(handle_center, handle_radius, style.handle_fill);
            painter.circle_stroke(
                handle_center,
                handle_radius,
                egui::Stroke::new(2.0, style.handle_border),
            );

            // Focus ring
            if response.has_focus() {
                let handle_rect = egui::Rect::from_center_size(
                    handle_center,
                    egui::vec2(handle_radius * 2.0, handle_radius * 2.0),
                );
                crate::paint::paint_focus_ring::paint_focus_ring(
                    painter,
                    handle_rect,
                    handle_radius,
                    theme.ring,
                );
            }
        }

        response
    }
}
