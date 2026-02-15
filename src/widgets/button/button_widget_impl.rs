//! Widget trait implementation for Button.

impl egui::Widget for super::button::Button<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let theme = crate::theme::shadcn_theme_ext::ShadcnThemeExt::shadcn_theme(ui.ctx());

        // Check if inside a ButtonGroup and claim an index
        let group_key =
            crate::widgets::button_group::button_group::ButtonGroup::context_key();
        let group_info = ui.ctx().data_mut(|d| {
            d.get_temp::<crate::widgets::button_group::button_group_context::ButtonGroupContext>(
                group_key,
            )
            .and_then(|mut ctx| {
                if !ctx.active {
                    return None;
                }
                let index = ctx.current_index;
                let cached_count = ctx.cached_count;
                let cr = ctx.corner_radius;
                ctx.current_index += 1;
                d.insert_temp(group_key, ctx);
                Some((index, cached_count, cr))
            })
        });
        let in_group = group_info.is_some();

        let style = super::button_variant_style::resolve_button_style(
            &theme,
            self.variant,
            self.size,
            false,
            false,
            !ui.is_enabled(),
        );

        let text_string = self.text.text().to_owned();
        let is_icon_only = !text_string.is_empty() == false && self.icon.is_some();
        let has_icon = self.icon.is_some();
        let has_text = !text_string.is_empty();

        let text_galley = ui.painter().layout_no_wrap(
            text_string.clone(),
            egui::FontId::proportional(style.font_size),
            style.fg,
        );

        let icon_size = style.height * 0.5;
        let icon_gap = 6.0;

        let desired = if is_icon_only {
            egui::vec2(style.height, style.height)
        } else if has_icon && has_text {
            egui::vec2(
                style.h_padding + icon_size + icon_gap + text_galley.size().x + style.h_padding,
                style.height,
            )
        } else {
            egui::vec2(
                text_galley.size().x + style.h_padding * 2.0,
                style.height,
            )
        };

        let (rect, response) = ui.allocate_exact_size(desired, egui::Sense::click());

        // Record boundary in group context
        if in_group {
            ui.ctx().data_mut(|d| {
                if let Some(mut ctx) = d.get_temp::<crate::widgets::button_group::button_group_context::ButtonGroupContext>(group_key) {
                    if ctx.active {
                        ctx.boundaries.push(rect.max.x);
                        d.insert_temp(group_key, ctx);
                    }
                }
            });
        }

        // Re-resolve with actual interaction state
        let style = super::button_variant_style::resolve_button_style(
            &theme,
            self.variant,
            self.size,
            response.hovered(),
            response.is_pointer_button_down_on(),
            !ui.is_enabled(),
        );

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let cr = group_corner_radius(&group_info, style.corner_radius);

            // Background
            painter.rect_filled(rect, cr, style.bg);

            // Border (skip when inside a button group — the group draws borders)
            if !in_group {
                if let Some(border_color) = style.border {
                    painter.rect_stroke(
                        rect,
                        cr,
                        egui::Stroke::new(1.0, border_color),
                        egui::epaint::StrokeKind::Inside,
                    );
                }
            }

            if is_icon_only {
                // Icon centered
                if let Some(ref icon) = self.icon {
                    let icon_rect = egui::Rect::from_center_size(
                        rect.center(),
                        egui::vec2(icon_size, icon_size),
                    );
                    crate::icons::paint_icon::paint_icon(painter, icon_rect, icon, style.fg);
                }
            } else if has_icon && has_text {
                // Icon + text
                let total_w = icon_size + icon_gap + text_galley.size().x;
                let start_x = rect.center().x - total_w / 2.0;

                if let Some(ref icon) = self.icon {
                    let icon_rect = egui::Rect::from_min_size(
                        egui::pos2(start_x, rect.center().y - icon_size / 2.0),
                        egui::vec2(icon_size, icon_size),
                    );
                    crate::icons::paint_icon::paint_icon(painter, icon_rect, icon, style.fg);
                }

                let text_pos = egui::pos2(
                    start_x + icon_size + icon_gap,
                    rect.center().y - text_galley.size().y / 2.0,
                );
                painter.galley(text_pos, text_galley, style.fg);
            } else {
                // Text only
                let text_pos = egui::pos2(
                    rect.center().x - text_galley.size().x / 2.0,
                    rect.center().y - text_galley.size().y / 2.0,
                );
                painter.galley(text_pos, text_galley, style.fg);
            }

            // Underline for Link variant
            if style.underline && response.hovered() {
                let galley2 = painter.layout_no_wrap(
                    text_string,
                    egui::FontId::proportional(style.font_size),
                    style.fg,
                );
                let text_pos = egui::pos2(
                    rect.center().x - galley2.size().x / 2.0,
                    rect.center().y - galley2.size().y / 2.0,
                );
                let underline_y = text_pos.y + galley2.size().y;
                painter.hline(
                    text_pos.x..=text_pos.x + galley2.size().x,
                    underline_y,
                    egui::Stroke::new(1.0, style.fg),
                );
            }

            // Focus ring
            if response.has_focus() {
                crate::paint::paint_focus_ring::paint_focus_ring(
                    painter,
                    rect,
                    style.corner_radius,
                    theme.ring,
                );
            }
        }

        response
    }
}

/// Computes per-corner radius for a button based on its group position.
/// First button gets left rounding, last gets right rounding, middle gets none.
fn group_corner_radius(
    group_info: &Option<(usize, usize, f32)>,
    default_cr: f32,
) -> egui::CornerRadius {
    match group_info {
        Some((index, cached_count, group_cr)) => {
            let r = group_cr.round() as u8;
            let is_first = *index == 0;
            let is_last = *cached_count > 0 && *index == *cached_count - 1;
            egui::CornerRadius {
                nw: if is_first { r } else { 0 },
                sw: if is_first { r } else { 0 },
                ne: if is_last { r } else { 0 },
                se: if is_last { r } else { 0 },
            }
        }
        None => egui::CornerRadius::same(default_cr.round() as u8),
    }
}
