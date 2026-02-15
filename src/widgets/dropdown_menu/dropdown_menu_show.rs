//! Show method for DropdownMenu — renders a popup menu.

impl super::dropdown_menu::DropdownMenu {
    /// Shows a dropdown menu anchored to a response.
    pub fn show(
        ui: &mut egui::Ui,
        trigger_response: &egui::Response,
        items: &[&str],
        on_select: impl FnOnce(usize),
    ) {
        let popup_id = trigger_response.id.with("dropdown");
        let theme = crate::theme::shadcn_theme_ext::ShadcnThemeExt::shadcn_theme(ui.ctx());

        let toggle_cmd = if trigger_response.clicked() {
            Some(egui::SetOpenCommand::Toggle)
        } else {
            None
        };

        let cr = (theme.radius + 2.0).round() as u8;
        let themed_frame = egui::Frame::NONE
            .fill(theme.popover)
            .inner_margin(egui::Margin::same(4))
            .corner_radius(egui::CornerRadius::same(cr))
            .stroke(egui::Stroke::new(1.0, theme.border))
            .shadow(egui::Shadow {
                offset: [0, 4],
                blur: 12,
                spread: 0,
                color: egui::Color32::from_black_alpha(8),
            });

        let popup = egui::Popup::new(
            popup_id,
            ui.ctx().clone(),
            trigger_response,
            ui.layer_id(),
        )
        .open_memory(toggle_cmd)
        .frame(themed_frame);

        let mut selected_idx = None;

        popup.show(|ui: &mut egui::Ui| {
            // Compute max text width to set a tight popup width
            let max_text_width: f32 = items
                .iter()
                .map(|label| {
                    ui.painter()
                        .layout_no_wrap(
                            label.to_string(),
                            egui::FontId::proportional(14.0),
                            theme.popover_foreground,
                        )
                        .size()
                        .x
                })
                .fold(0.0_f32, f32::max);

            let menu_width = (max_text_width + 24.0).max(120.0);
            ui.set_min_width(menu_width);
            ui.set_max_width(menu_width);

            for (idx, &label) in items.iter().enumerate() {
                let galley = ui.painter().layout_no_wrap(
                    label.to_owned(),
                    egui::FontId::proportional(14.0),
                    theme.popover_foreground,
                );
                let desired = egui::vec2(menu_width, galley.size().y + 8.0);
                let (rect, r) = ui.allocate_exact_size(desired, egui::Sense::click());

                if r.hovered() {
                    ui.painter().rect_filled(
                        rect,
                        egui::CornerRadius::same(4),
                        theme.accent,
                    );
                }

                if ui.is_rect_visible(rect) {
                    ui.painter().galley(
                        egui::pos2(
                            rect.min.x + 8.0,
                            rect.center().y - galley.size().y / 2.0,
                        ),
                        galley,
                        theme.popover_foreground,
                    );
                }

                if r.clicked() {
                    selected_idx = Some(idx);
                    egui::Popup::close_id(ui.ctx(), popup_id);
                    ui.ctx().request_repaint();
                }
            }
        });

        if let Some(idx) = selected_idx {
            on_select(idx);
        }
    }
}
