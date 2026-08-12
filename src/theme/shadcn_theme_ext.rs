//! Extension trait for accessing the ShadcnTheme from egui::Context.

use super::shadcn_theme::ShadcnTheme;

/// Extension trait that adds shadcn theme access to `egui::Context`.
pub trait ShadcnThemeExt {
    /// Returns a clone of the current shadcn theme (light by default).
    fn shadcn_theme(&self) -> ShadcnTheme;

    /// Sets the shadcn theme for all subsequent frames.
    fn set_shadcn_theme(&self, theme: ShadcnTheme);
}

impl ShadcnThemeExt for egui::Context {
    fn shadcn_theme(&self) -> ShadcnTheme {
        self.data(|d| d.get_temp::<ShadcnTheme>(egui::Id::NULL))
            .unwrap_or_default()
    }

    fn set_shadcn_theme(&self, theme: ShadcnTheme) {
        // Apply a full derived `egui::Visuals`, not just the window
        // fill/stroke/shadow this used to set — see `visuals_from`'s doc
        // comment for why the previous partial application was a bug, not a
        // deliberate scope limit. `all_styles_mut` mutates both of egui's
        // internal dark/light `Style`s identically, matching this function's
        // pre-existing behaviour: the shadcn theme, once set, paints
        // uniformly regardless of which `egui::Theme` the OS/host reports.
        let visuals = visuals_from(&theme);
        self.all_styles_mut(|style| {
            style.visuals = visuals.clone();
        });
        self.data_mut(|d| {
            d.insert_temp::<ShadcnTheme>(egui::Id::NULL, theme);
        });
    }
}

/// Derives a full `egui::Visuals` from a `ShadcnTheme`.
///
/// Before this function existed, `set_shadcn_theme` only overrode
/// `visuals.window_fill`, `window_stroke`, and `window_shadow` — three
/// fields, out of the dozens `Visuals` actually has. Every other stock egui
/// widget (`DragValue`, `TextEdit`, scrollbars, `egui::Grid` striping,
/// `egui_plot`'s background/grid/axes) and every panel without an explicit
/// `.frame()` kept painting from egui's own default `Visuals::dark()`/
/// `::light()` for whatever `egui::Theme` the OS reports — while `Card` and
/// the other shadcn widgets painted from the theme actually passed in here.
/// A light palette on a dark-mode OS (or vice versa) produced two
/// simultaneously-visible themes in the same window, which reads as "hard to
/// see" / inconsistent rather than as an obvious color mismatch. Deriving
/// and applying the full `Visuals` here means every consumer of this crate
/// gets a self-consistent egui for free, without needing to hand-roll this
/// mapping in application code.
///
/// "Dark" is inferred from `background`'s relative luminance rather than
/// threaded through as a parameter, so this stays a pure function of the
/// `ShadcnTheme` and doesn't need every one of the twenty existing
/// constructors (or a future one) to also report their own light/dark-ness.
fn visuals_from(theme: &ShadcnTheme) -> egui::Visuals {
    let dark = relative_luminance(theme.background) < 0.5;
    let mut visuals = if dark {
        egui::Visuals::dark()
    } else {
        egui::Visuals::light()
    };

    visuals.override_text_color = Some(theme.foreground);
    visuals.hyperlink_color = theme.primary;
    visuals.faint_bg_color = theme.muted;
    visuals.extreme_bg_color = theme.input;
    visuals.code_bg_color = theme.muted;
    visuals.warn_fg_color = theme.destructive.gamma_multiply(0.75);
    visuals.error_fg_color = theme.destructive;

    visuals.window_fill = theme.popover;
    visuals.window_stroke = egui::Stroke::new(1.0, theme.border);
    visuals.window_shadow = egui::Shadow {
        offset: [0, 4],
        blur: 12,
        spread: 0,
        color: egui::Color32::from_black_alpha(8),
    };
    visuals.panel_fill = theme.background;

    visuals.selection.bg_fill = theme.primary.gamma_multiply(0.4);
    visuals.selection.stroke = egui::Stroke::new(1.0, theme.ring);
    visuals.text_cursor.stroke.color = theme.foreground;

    let radius = egui::CornerRadius::same(theme.radius.round() as u8);
    visuals.widgets.noninteractive = widget_visuals(theme.card, theme.card, theme.border, theme.muted_foreground, radius);
    visuals.widgets.inactive = widget_visuals(theme.secondary, theme.secondary, theme.border, theme.secondary_foreground, radius);
    visuals.widgets.hovered = widget_visuals(theme.accent, theme.accent, theme.ring, theme.accent_foreground, radius);
    visuals.widgets.active = widget_visuals(theme.primary, theme.primary, theme.ring, theme.primary_foreground, radius);
    visuals.widgets.open = visuals.widgets.inactive;

    visuals
}

fn widget_visuals(
    bg_fill: egui::Color32,
    weak_bg_fill: egui::Color32,
    stroke_color: egui::Color32,
    fg_color: egui::Color32,
    corner_radius: egui::CornerRadius,
) -> egui::style::WidgetVisuals {
    egui::style::WidgetVisuals {
        bg_fill,
        weak_bg_fill,
        bg_stroke: egui::Stroke::new(1.0, stroke_color),
        corner_radius,
        fg_stroke: egui::Stroke::new(1.0, fg_color),
        expansion: 0.0,
    }
}

/// ITU-R BT.709 relative luminance, treating `Color32`'s channels as already
/// gamma-encoded sRGB (i.e. skipping the linearization step) — a shortcut
/// that's exact enough to classify a theme as "reads as dark" vs. "reads as
/// light" without pulling in a color-management dependency for it.
fn relative_luminance(c: egui::Color32) -> f32 {
    let r = c.r() as f32 / 255.0;
    let g = c.g() as f32 / 255.0;
    let b = c.b() as f32 / 255.0;
    0.2126 * r + 0.7152 * g + 0.0722 * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dark_and_light_backgrounds_are_classified_correctly() {
        assert!(relative_luminance(super::super::shadcn_theme_dark::dark().background) < 0.5);
        assert!(relative_luminance(super::super::shadcn_theme_light::light().background) >= 0.5);
    }

    type ThemeCtor = fn() -> ShadcnTheme;

    #[test]
    fn every_bundled_theme_background_is_classified_as_expected() {
        // (constructor, expected_dark)
        let cases: &[(ThemeCtor, bool)] = &[
            (super::super::shadcn_theme_dark::dark, true),
            (super::super::shadcn_theme_light::light, false),
            (super::super::shadcn_theme_violet::violet_dark, true),
            (super::super::shadcn_theme_violet::violet_light, false),
            (super::super::shadcn_theme_sky::sky_dark, true),
            (super::super::shadcn_theme_sky::sky_light, false),
            (super::super::shadcn_theme_obsidian::obsidian_dark, true),
            (super::super::shadcn_theme_obsidian::obsidian_light, false),
            (super::super::shadcn_theme_pink::pink_dark, true),
            (super::super::shadcn_theme_pink::pink_light, false),
            (super::super::shadcn_theme_terracotta::terracotta_dark, true),
            (super::super::shadcn_theme_terracotta::terracotta_light, false),
            (super::super::shadcn_theme_rainbow::rainbow_dark, true),
            (super::super::shadcn_theme_rainbow::rainbow_light, false),
            (super::super::shadcn_theme_heatmap::heatmap_dark, true),
            (super::super::shadcn_theme_heatmap::heatmap_light, false),
            (super::super::shadcn_theme_cyber_aurora::cyber_aurora_dark, true),
            (super::super::shadcn_theme_cyber_aurora::cyber_aurora_light, false),
            (super::super::shadcn_theme_nostalgia::nostalgia_dark, true),
            (super::super::shadcn_theme_nostalgia::nostalgia_light, false),
        ];
        for (ctor, expected_dark) in cases {
            let theme = ctor();
            let is_dark = relative_luminance(theme.background) < 0.5;
            assert_eq!(
                is_dark, *expected_dark,
                "background {:?} (luminance {:.3}) misclassified for a {} constructor",
                theme.background,
                relative_luminance(theme.background),
                if *expected_dark { "dark" } else { "light" }
            );
        }
    }

    #[test]
    fn visuals_from_dark_theme_uses_theme_colors_not_egui_defaults() {
        let theme = super::super::shadcn_theme_dark::dark();
        let visuals = visuals_from(&theme);
        assert_eq!(visuals.panel_fill, theme.background);
        assert_eq!(visuals.override_text_color, Some(theme.foreground));
        assert_eq!(visuals.extreme_bg_color, theme.input);
        assert_eq!(visuals.faint_bg_color, theme.muted);
        assert_eq!(visuals.widgets.active.bg_fill, theme.primary);
    }
}
