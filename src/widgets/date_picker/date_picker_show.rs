//! Show method for DatePicker — renders date input with calendar popup.

impl super::date_picker::DatePicker {
    /// Shows the date picker. `state` holds the selected date.
    pub fn show(
        self,
        ui: &mut egui::Ui,
        state: &mut super::date_picker_state::DatePickerState,
    ) -> egui::Response {
        let theme = crate::theme::shadcn_theme_ext::ShadcnThemeExt::shadcn_theme(ui.ctx());

        let display_text = if state.is_set() {
            state.format()
        } else {
            self.placeholder.clone()
        };

        let text_color = if state.is_set() {
            theme.foreground
        } else {
            theme.muted_foreground
        };

        // Trigger button: calendar icon + text
        let icon_size: f32 = 14.0;
        let gap: f32 = 6.0;
        let h_padding: f32 = 8.0;
        let height: f32 = 32.0;
        let galley =
            ui.painter()
                .layout_no_wrap(display_text, egui::FontId::proportional(14.0), text_color);
        let desired = egui::vec2(icon_size + gap + galley.size().x + h_padding * 2.0, height);
        let (trigger_rect, mut btn_response) =
            ui.allocate_exact_size(desired, egui::Sense::click());

        if ui.is_rect_visible(trigger_rect) {
            if btn_response.hovered() || btn_response.is_pointer_button_down_on() {
                let bg = if btn_response.is_pointer_button_down_on() {
                    crate::paint::interpolate_color::interpolate_color(
                        theme.background,
                        theme.accent,
                        0.85,
                    )
                } else {
                    theme.accent
                };
                ui.painter().rect_filled(
                    trigger_rect,
                    egui::CornerRadius::same(theme.radius.round() as u8),
                    bg,
                );
            }

            let icon_rect = egui::Rect::from_min_size(
                egui::pos2(
                    trigger_rect.min.x + h_padding,
                    trigger_rect.center().y - icon_size / 2.0,
                ),
                egui::vec2(icon_size, icon_size),
            );
            crate::icons::paint_icon::paint_icon(
                ui.painter(),
                icon_rect,
                &crate::icons::lucide_icon::LucideIcon::Calendar,
                theme.muted_foreground,
            );

            let text_pos = egui::pos2(
                trigger_rect.min.x + h_padding + icon_size + gap,
                trigger_rect.center().y - galley.size().y / 2.0,
            );
            ui.painter().galley(text_pos, galley, text_color);
        }

        if btn_response.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }

        let popup_id = btn_response.id.with("date_picker_popup");
        let browsing_id = popup_id.with("browsing_month");
        let was_open = egui::Popup::is_id_open(ui.ctx(), popup_id);

        let toggle_cmd = if btn_response.clicked() {
            Some(egui::SetOpenCommand::Toggle)
        } else {
            None
        };

        let cr = (theme.radius + 2.0).round() as u8;
        let themed_frame = egui::Frame::NONE
            .fill(theme.popover)
            .inner_margin(egui::Margin::same(12))
            .corner_radius(egui::CornerRadius::same(cr))
            .stroke(egui::Stroke::new(1.0, theme.border))
            .shadow(egui::Shadow {
                offset: [0, 4],
                blur: 12,
                spread: 0,
                color: egui::Color32::from_black_alpha(8),
            });

        let popup = egui::Popup::new(popup_id, ui.ctx().clone(), &btn_response, ui.layer_id())
            .open_memory(toggle_cmd)
            .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
            .frame(themed_frame);

        let selection = popup.show(|ui: &mut egui::Ui| {
            let initial_month = if state.year == 0 && !state.is_set() {
                (2026, 1)
            } else {
                (state.year, state.month.clamp(1, 12))
            };
            let (mut year, mut month) = if was_open {
                ui.data(|data| data.get_temp(browsing_id))
                    .unwrap_or(initial_month)
            } else {
                initial_month
            };
            let mut selected_day = if (year, month) == (state.year, state.month) {
                state.day
            } else {
                0
            };
            let cal = crate::widgets::calendar::calendar::Calendar::new();
            let day = cal.show(ui, &mut year, &mut month, &mut selected_day);
            ui.data_mut(|data| data.insert_temp(browsing_id, (year, month)));
            day.map(|day| (year, month, day))
        });

        if let Some((year, month, day)) = selection.and_then(|response| response.inner) {
            if (state.year, state.month, state.day) != (year, month, day) {
                *state = super::date_picker_state::DatePickerState { year, month, day };
                btn_response.mark_changed();
            }
            egui::Popup::close_id(ui.ctx(), popup_id);
            ui.ctx().request_repaint();
        }

        btn_response
    }
}
