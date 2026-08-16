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
///
/// `widgets.active.fg_stroke` is deliberately `theme.foreground`, not
/// `theme.primary_foreground` — this is the fix for a real bug, not a
/// stylistic choice, so it's worth spelling out. `egui::Visuals::
/// strong_text_color()` (what `RichText::strong()` / `.strong` on
/// `WidgetText` resolves its color through) is hardcoded upstream as
/// `self.widgets.active.text_color()`, with **no override field** — unlike
/// `weak_text_color`, there is nothing in `egui::Visuals` a theme can set to
/// redirect it. Every consumer that calls plain `.strong()` on a `RichText`
/// (ten widgets in this crate alone — `Accordion`, `Alert`, `AlertDialog`,
/// `Calendar`, `Command`, `Dialog`, `Drawer`, `Sheet`, `Toast`,
/// `Typography` — plus any downstream app that reaches for it, which is the
/// obvious thing to reach for) therefore silently painted with whatever
/// `widgets.active`'s foreground was set to. Setting it to
/// `theme.primary_foreground` — correct for text painted *on* a
/// `theme.primary`-colored surface (a pressed/selected button) — meant
/// "strong" text anywhere else (ordinary card/panel backgrounds) used a
/// token that was never designed to contrast against them. Most bundled
/// palettes hide this by accident (`primary_foreground` happens to still be
/// legible against `background`/`card` for values tuned close to shadcn's
/// defaults); the more saturated palettes (Nostalgia's `primary_foreground`
/// is pure white in the light variant, near-black in the dark one) make it
/// unmissable — headings render nearly invisible, backwards between the two
/// modes. `theme.foreground` is correct against `background`/`card`/
/// `popover` by construction (every text-on-surface token pair is), so
/// re-pointing `fg_stroke` there fixes every one of those ten widgets and
/// every consuming app at once. `bg_fill`/`weak_bg_fill`/`bg_stroke` stay on
/// `theme.primary`/`theme.ring` — nothing in this crate reads them (every
/// widget here paints its own primary-colored surfaces straight from
/// `ShadcnTheme`, bypassing `Visuals::widgets` entirely), so they're kept
/// only so a stock, un-wrapped egui widget still gets a plausible "pressed"
/// look instead of egui's un-themed default. One accepted cost: "strong"
/// text is no longer visually distinct in color from plain text under this
/// theme (both now resolve to `theme.foreground` — `.strong()` never
/// affected font weight in egui, only color, so there was no boldness to
/// preserve). Legible-everywhere beats a color distinction that was
/// invisible half the time anyway; an intentionally bold heading style
/// should use an explicit `FontId`, not `strong_text_color()`.
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
    visuals.widgets.noninteractive = widget_visuals(
        theme.card,
        theme.card,
        theme.border,
        theme.muted_foreground,
        radius,
    );
    visuals.widgets.inactive = widget_visuals(
        theme.secondary,
        theme.secondary,
        theme.border,
        theme.secondary_foreground,
        radius,
    );
    visuals.widgets.hovered = widget_visuals(
        theme.accent,
        theme.accent,
        theme.ring,
        theme.accent_foreground,
        radius,
    );
    // fg_stroke is `theme.foreground`, not `theme.primary_foreground` — see this
    // function's doc comment (`strong_text_color()` has no override field upstream).
    visuals.widgets.active = widget_visuals(
        theme.primary,
        theme.primary,
        theme.ring,
        theme.foreground,
        radius,
    );
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

/// WCAG 2.x contrast ratio between two colors (1.0 = no contrast, 21.0 =
/// black-on-white). Unlike `relative_luminance` above — a "close enough to
/// classify dark-vs-light" shortcut — this linearizes sRGB first, since the
/// WCAG formula is defined on linear-light luminance and understates
/// contrast for mid-tones otherwise. Exists to catch exactly the class of
/// bug fixed by this file's `widgets.active.fg_stroke` change: a
/// theme-derived text color that's individually plausible but illegible
/// against the specific surface it actually gets painted on.
#[cfg(test)]
fn contrast_ratio(a: egui::Color32, b: egui::Color32) -> f32 {
    fn linear_luminance(c: egui::Color32) -> f32 {
        let to_linear = |channel: u8| {
            let c = channel as f32 / 255.0;
            if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
        };
        0.2126 * to_linear(c.r()) + 0.7152 * to_linear(c.g()) + 0.0722 * to_linear(c.b())
    }
    let (l1, l2) = (linear_luminance(a), linear_luminance(b));
    let (lighter, darker) = if l1 >= l2 { (l1, l2) } else { (l2, l1) };
    (lighter + 0.05) / (darker + 0.05)
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

    /// Every bundled palette constructor, paired with whether it's the dark
    /// or light variant. Shared by every test in this module that needs to
    /// loop over "all 20 bundled themes" so the list can't silently drift
    /// out of sync between tests as new palettes are added.
    const ALL_THEME_CTORS: &[(ThemeCtor, bool)] = &[
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
        (
            super::super::shadcn_theme_terracotta::terracotta_light,
            false,
        ),
        (super::super::shadcn_theme_rainbow::rainbow_dark, true),
        (super::super::shadcn_theme_rainbow::rainbow_light, false),
        (super::super::shadcn_theme_heatmap::heatmap_dark, true),
        (super::super::shadcn_theme_heatmap::heatmap_light, false),
        (
            super::super::shadcn_theme_cyber_aurora::cyber_aurora_dark,
            true,
        ),
        (
            super::super::shadcn_theme_cyber_aurora::cyber_aurora_light,
            false,
        ),
        (super::super::shadcn_theme_nostalgia::nostalgia_dark, true),
        (super::super::shadcn_theme_nostalgia::nostalgia_light, false),
    ];

    #[test]
    fn every_bundled_theme_background_is_classified_as_expected() {
        for (ctor, expected_dark) in ALL_THEME_CTORS {
            let theme = ctor();
            let is_dark = relative_luminance(theme.background) < 0.5;
            assert_eq!(
                is_dark,
                *expected_dark,
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

    /// WCAG AA's minimum contrast ratio for normal text. Regular labels
    /// could reasonably hold to the stricter large-text threshold (3.0), but
    /// this crate doesn't distinguish text sizes when deriving colors, so
    /// every text/surface pairing is held to the stricter bar.
    const MIN_TEXT_CONTRAST: f32 = 4.5;

    /// The regression test for the bug this file's `widgets.active.fg_stroke`
    /// change fixes: `RichText::strong()` resolves its color through
    /// `Visuals::strong_text_color()`, which egui hardcodes to
    /// `widgets.active.text_color()` with no override hook (see
    /// `visuals_from`'s doc comment). Static token-pair contrast alone
    /// wouldn't have caught this — `theme.primary`/`theme.primary_foreground`
    /// individually contrast fine against *each other*; the bug was pairing
    /// `primary_foreground` against `background`/`card`/`popover`, surfaces
    /// it was never designed for. So this asserts contrast against the
    /// *actual* resolved `strong_text_color()`, on every surface a
    /// non-interactive "strong" label can plausibly sit on in this crate
    /// (`Card`/`Toolbar`/`PropertyGrid` all fill with `theme.card`;
    /// `Dialog`/`Popover`/`Toast` sit on `theme.popover`; anything else
    /// un-wrapped sits directly on `theme.background`) — not just the
    /// isolated token pairs `visuals_from` happens to define.
    #[test]
    fn strong_text_is_legible_on_every_surface_it_can_appear_on() {
        for (ctor, _) in ALL_THEME_CTORS {
            let theme = ctor();
            let visuals = visuals_from(&theme);
            let strong = visuals.strong_text_color();
            for (surface_name, surface) in
                [("background", theme.background), ("card", theme.card), ("popover", theme.popover)]
            {
                let ratio = contrast_ratio(strong, surface);
                assert!(
                    ratio >= MIN_TEXT_CONTRAST,
                    "strong_text_color() {strong:?} on {surface_name} {surface:?} has contrast \
                     {ratio:.2}, below the {MIN_TEXT_CONTRAST} minimum (theme background {:?})",
                    theme.background,
                );
            }
        }
    }

    /// Same coverage as `strong_text_is_legible_on_every_surface_it_can_appear_on`
    /// but for plain (non-`.strong()`) text, i.e. `override_text_color`. This
    /// was never broken by the bug fixed in this file, but without this test
    /// nothing would catch a future palette shipping a `foreground` that's
    /// illegible against its own `card`/`popover` — the case the initial
    /// manual read-through of every bundled palette (see this phase's plan)
    /// only spot-checked rather than exhaustively verified.
    #[test]
    fn plain_text_is_legible_on_every_surface_it_can_appear_on() {
        for (ctor, _) in ALL_THEME_CTORS {
            let theme = ctor();
            for (surface_name, surface) in
                [("background", theme.background), ("card", theme.card), ("popover", theme.popover)]
            {
                let ratio = contrast_ratio(theme.foreground, surface);
                assert!(
                    ratio >= MIN_TEXT_CONTRAST,
                    "foreground {:?} on {surface_name} {surface:?} has contrast {ratio:.2}, below \
                     the {MIN_TEXT_CONTRAST} minimum (theme background {:?})",
                    theme.foreground,
                    theme.background,
                );
            }
        }
    }
}
