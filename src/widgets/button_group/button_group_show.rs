//! Show method for ButtonGroup — renders buttons in a connected strip.

impl super::button_group::ButtonGroup {
    /// The egui temp data key for the active button group context.
    pub fn context_key() -> egui::Id {
        egui::Id::new("egui_shadcn_btn_group")
    }

    /// Renders a connected button group. Pass buttons inside the closure.
    /// Buttons detect the active context and render with per-corner radii:
    /// first button gets left rounding, last button gets right rounding.
    pub fn show(
        ui: &mut egui::Ui,
        content: impl FnOnce(&mut egui::Ui),
    ) -> egui::InnerResponse<()> {
        let theme = crate::theme::shadcn_theme_ext::ShadcnThemeExt::shadcn_theme(ui.ctx());
        let cr = theme.radius;
        let key = Self::context_key();

        // Per-group count cache (unique per UI position)
        let count_key = ui.auto_id_with("btn_group_count");
        let cached_count = ui
            .ctx()
            .data(|d| d.get_temp::<usize>(count_key))
            .unwrap_or(0);

        // Activate group context
        ui.ctx().data_mut(|d| {
            d.insert_temp(
                key,
                super::button_group_context::ButtonGroupContext {
                    active: true,
                    boundaries: Vec::new(),
                    cached_count,
                    current_index: 0,
                    corner_radius: cr,
                },
            );
        });

        let result = ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            content(ui);
        });

        // Read boundaries, final count, and deactivate
        let (boundaries, final_count) = ui.ctx().data_mut(|d| {
            let ctx = d
                .get_temp::<super::button_group_context::ButtonGroupContext>(key);
            d.insert_temp(
                key,
                super::button_group_context::ButtonGroupContext {
                    active: false,
                    ..Default::default()
                },
            );
            ctx.map(|c| (c.boundaries, c.current_index))
                .unwrap_or_default()
        });

        // Cache this group's count for next frame
        ui.ctx()
            .data_mut(|d| d.insert_temp(count_key, final_count));

        // Draw outer ring and dividers
        let rect = result.response.rect;
        if ui.is_rect_visible(rect) {
            let ring_color = egui::Color32::from_rgba_unmultiplied(
                theme.foreground.r(),
                theme.foreground.g(),
                theme.foreground.b(),
                26,
            );

            // Outer rounded border
            ui.painter().rect_stroke(
                rect,
                egui::CornerRadius::same(cr.round() as u8),
                egui::Stroke::new(1.0, ring_color),
                egui::epaint::StrokeKind::Inside,
            );

            // Vertical dividers between buttons (skip after the last one)
            if boundaries.len() > 1 {
                for &x in boundaries.iter().take(boundaries.len() - 1) {
                    ui.painter().vline(
                        x,
                        rect.min.y..=rect.max.y,
                        egui::Stroke::new(1.0, ring_color),
                    );
                }
            }
        }

        result
    }
}
