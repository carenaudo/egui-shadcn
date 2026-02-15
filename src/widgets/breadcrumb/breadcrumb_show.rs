//! Show method for Breadcrumb — renders a navigation path.

impl super::breadcrumb::Breadcrumb {
    /// Shows the breadcrumb. Returns the index of the clicked item, if any.
    pub fn show(self, ui: &mut egui::Ui) -> Option<usize> {
        let theme = crate::theme::shadcn_theme_ext::ShadcnThemeExt::shadcn_theme(ui.ctx());
        let mut clicked_idx = None;
        let last_idx = self.items.len().saturating_sub(1);

        ui.horizontal(|ui| {
            for (idx, item) in self.items.iter().enumerate() {
                let is_last = idx == last_idx;

                let color = if is_last {
                    theme.foreground
                } else {
                    theme.muted_foreground
                };

                let response = ui.add(
                    egui::Label::new(
                        egui::RichText::new(item).color(color).size(14.0),
                    )
                    .sense(if is_last {
                        egui::Sense::hover()
                    } else {
                        egui::Sense::click()
                    }),
                );

                if !is_last {
                    if response.hovered() {
                        ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
                    }

                    if response.clicked() {
                        clicked_idx = Some(idx);
                    }

                    ui.label(
                        egui::RichText::new(&self.separator)
                            .color(theme.muted_foreground)
                            .size(14.0),
                    );
                }
            }
        });

        clicked_idx
    }
}
