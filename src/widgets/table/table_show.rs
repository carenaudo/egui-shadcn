//! Show method for Table — renders a styled data table.

impl super::table::Table {
    /// Shows the table. Returns the response for the outer frame.
    pub fn show(self, ui: &mut egui::Ui) -> egui::Response {
        let theme = crate::theme::shadcn_theme_ext::ShadcnThemeExt::shadcn_theme(ui.ctx());
        let cr = theme.radius.round() as u8;

        let frame = egui::Frame::NONE
            .fill(theme.card)
            .corner_radius(egui::CornerRadius::same(cr))
            .stroke(egui::Stroke::new(1.0, theme.border));

        frame
            .show(ui, |ui| {
                let col_count = self.headers.len();
                if col_count == 0 {
                    return;
                }

                let available = ui.available_width();
                let col_width = available / col_count as f32;

                // Header row
                Self::render_row(
                    ui,
                    &self.headers,
                    col_width,
                    theme.muted,
                    theme.muted_foreground,
                    true,
                );

                // Divider
                let rect = ui.available_rect_before_wrap();
                ui.painter().hline(
                    rect.min.x..=rect.min.x + available,
                    rect.min.y,
                    egui::Stroke::new(1.0, theme.border),
                );
                ui.add_space(1.0);

                // Data rows
                for (row_idx, row) in self.rows.iter().enumerate() {
                    let bg = if self.striped && row_idx % 2 == 1 {
                        theme.muted
                    } else {
                        egui::Color32::TRANSPARENT
                    };

                    Self::render_row(
                        ui,
                        row,
                        col_width,
                        bg,
                        theme.foreground,
                        false,
                    );

                    // Row divider (not after last)
                    if row_idx < self.rows.len() - 1 {
                        let rect = ui.available_rect_before_wrap();
                        ui.painter().hline(
                            rect.min.x..=rect.min.x + available,
                            rect.min.y,
                            egui::Stroke::new(1.0, theme.border),
                        );
                        ui.add_space(1.0);
                    }
                }
            })
            .response
    }

    fn render_row(
        ui: &mut egui::Ui,
        cells: &[String],
        col_width: f32,
        bg: egui::Color32,
        fg: egui::Color32,
        is_header: bool,
    ) {
        let height: f32 = if is_header { 40.0 } else { 44.0 };
        let font_size: f32 = 14.0;

        let (row_rect, _) = ui.allocate_exact_size(
            egui::vec2(col_width * cells.len() as f32, height),
            egui::Sense::hover(),
        );

        if bg != egui::Color32::TRANSPARENT {
            ui.painter()
                .rect_filled(row_rect, egui::CornerRadius::ZERO, bg);
        }

        for (col_idx, cell) in cells.iter().enumerate() {
            let cell_rect = egui::Rect::from_min_size(
                egui::pos2(
                    row_rect.min.x + col_width * col_idx as f32,
                    row_rect.min.y,
                ),
                egui::vec2(col_width, height),
            );

            let galley = ui.painter().layout_no_wrap(
                cell.clone(),
                egui::FontId::proportional(font_size),
                fg,
            );

            let text_pos = egui::pos2(
                cell_rect.min.x + 12.0,
                cell_rect.center().y - galley.size().y / 2.0,
            );
            ui.painter().galley(text_pos, galley, fg);
        }
    }
}
