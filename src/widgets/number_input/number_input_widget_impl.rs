//! Widget trait implementation for NumberInput.

impl egui::Widget for super::number_input::NumberInput<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let theme = crate::theme::shadcn_theme_ext::ShadcnThemeExt::shadcn_theme(ui.ctx());

        let height: f32 = ui.spacing().interact_size.y.max(20.0);
        let h_padding: f32 = 6.0;
        let cr_val = theme.radius.round().min(255.0) as u8;
        let cr = egui::CornerRadius::same(cr_val);

        // Convert to f64 proxy for DragValue
        let mut proxy: f64 = match &self.value {
            super::number_input::ValueRef::F64(v) => **v,
            super::number_input::ValueRef::F32(v) => **v as f64,
            super::number_input::ValueRef::I32(v) => **v as f64,
        };

        let width = self.width.unwrap_or(60.0);
        let desired = egui::vec2(width, height);
        let (outer_rect, _) = ui.allocate_exact_size(desired, egui::Sense::hover());

        // Paint background only — border is drawn after DragValue so we
        // can choose between the regular border and the focus ring.
        ui.painter().rect_filled(outer_rect, cr, theme.muted);

        // Inner region with padding
        let inner_rect = egui::Rect::from_min_max(
            egui::pos2(outer_rect.min.x + h_padding, outer_rect.min.y),
            egui::pos2(outer_rect.max.x - h_padding, outer_rect.max.y),
        );

        let mut child_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(inner_rect)
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
        );

        // Paint prefix text
        if let Some(ref prefix) = self.prefix {
            let galley = child_ui.painter().layout_no_wrap(
                prefix.clone(),
                egui::FontId::proportional(11.0),
                theme.muted_foreground,
            );
            let prefix_rect = child_ui
                .allocate_exact_size(galley.size(), egui::Sense::hover())
                .0;
            child_ui
                .painter()
                .galley(prefix_rect.min, galley, theme.muted_foreground);
            child_ui.add_space(2.0);
        }

        // Build DragValue
        let mut dv = egui::DragValue::new(&mut proxy)
            .speed(self.speed)
            .update_while_editing(false);

        if let Some(range) = &self.range {
            dv = dv.range(range.clone());
        }
        if let Some(decimals) = self.decimals {
            dv = dv.max_decimals(decimals).min_decimals(decimals);
        }
        if let Some(ref suffix) = self.suffix {
            dv = dv.suffix(suffix.clone());
        }

        let remaining_width = child_ui.available_width();
        child_ui.set_max_width(remaining_width);

        // Style overrides — suppress ALL inner backgrounds/borders so only our
        // outer rect draws the visual chrome (prevents double-border on focus).
        {
            let wv = &mut child_ui.style_mut().visuals.widgets;
            wv.inactive.bg_fill = egui::Color32::TRANSPARENT;
            wv.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;
            wv.inactive.bg_stroke = egui::Stroke::NONE;
            wv.hovered.bg_fill = egui::Color32::TRANSPARENT;
            wv.hovered.weak_bg_fill = egui::Color32::TRANSPARENT;
            wv.hovered.bg_stroke = egui::Stroke::NONE;
            wv.active.bg_fill = egui::Color32::TRANSPARENT;
            wv.active.weak_bg_fill = egui::Color32::TRANSPARENT;
            wv.active.bg_stroke = egui::Stroke::NONE;
            wv.noninteractive.bg_fill = egui::Color32::TRANSPARENT;
            wv.noninteractive.bg_stroke = egui::Stroke::NONE;
            wv.open.bg_fill = egui::Color32::TRANSPARENT;
            wv.open.bg_stroke = egui::Stroke::NONE;
        }
        // Prevent the TextEdit (when DragValue enters editing mode) from
        // drawing its own dark background — it uses extreme_bg_color.
        child_ui.style_mut().visuals.extreme_bg_color = egui::Color32::TRANSPARENT;
        child_ui.style_mut().visuals.override_text_color = Some(theme.foreground);
        // Keep text selection highlight visible.
        child_ui.style_mut().visuals.selection.bg_fill = theme.primary;

        let response = child_ui.add(dv);

        // Write back
        match self.value {
            super::number_input::ValueRef::F64(v) => *v = proxy,
            super::number_input::ValueRef::F32(v) => *v = proxy as f32,
            super::number_input::ValueRef::I32(v) => *v = proxy.round() as i32,
        }

        // Single border: focus ring when focused, regular border otherwise.
        if response.has_focus() || response.dragged() {
            ui.painter().rect_stroke(
                outer_rect,
                cr,
                egui::Stroke::new(1.0, theme.ring),
                egui::epaint::StrokeKind::Inside,
            );
        } else {
            ui.painter().rect_stroke(
                outer_rect,
                cr,
                egui::Stroke::new(1.0, theme.input),
                egui::epaint::StrokeKind::Inside,
            );
        }

        response
    }
}
