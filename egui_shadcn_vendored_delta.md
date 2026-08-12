# egui-shadcn Fork Delta & Comparison Documentation

This document provides a detailed technical comparison of all modifications made in this local vendored fork compared to upstream **[pjankiewicz/egui-shadcn](https://github.com/pjankiewicz/egui-shadcn)**.

---

## 1. Fixes Needed for Compatibility (`egui 0.35` & Cargo Dependencies)

Upstream `egui-shadcn` was designed for `egui 0.33`. To integrate smoothly with `egui 0.35` and workspace dependencies, the following updates were made:

### `Cargo.toml` Dependencies
| Dependency | Upstream | Vendored Fork | Notes |
| :--- | :--- | :--- | :--- |
| `egui` | `0.33` | `0.35` (workspace) | Upgraded to workspace `egui 0.35` |
| `egui_flex` | `0.5` | `0.7` | First version supporting `egui 0.35` |
| `eframe` (dev) | `0.33` | `0.35` (workspace) | Matched workspace features (`default_fonts`, `wgpu`) |

### API Drift Fixes

1. **`ShadcnThemeExt` (`src/theme/shadcn_theme_ext.rs`)**:
   - `egui 0.35` removed `Context::style_mut` in favor of per-theme styles.
   - Replaced with `self.all_styles_mut(|style| ...)` to maintain single-style window fill, stroke, and shadow overrides for popups.

2. **`TextEdit::frame` Inset Migration**:
   - In `egui 0.34+`, `TextEdit::frame` changed from a boolean to a `Frame` struct.
   - Replacing `.frame(false)` with `Frame::NONE` removes padding entirely, causing 4px/2px layout shifts.
   - Updated `input_widget_impl.rs`, `textarea_widget_impl.rs`, and `input_group_show.rs` to use `Frame::NONE.inner_margin(Margin::symmetric(4, 2))` to maintain pixel-identical layout with upstream.

3. **`Painter::rect` / `rect_stroke` Signature Update**:
   - `egui 0.35` introduced explicit `StrokeKind` parameters (`StrokeKind::Inside` / `StrokeKind::Outside`) for rect strokes.
   - Updated painter invocations across widget implementations and demo applications.

---

## 2. Fixes Needed for Custom Code & Widget Behaviors

### `Toggle` Widget Text Rendering (`src/widgets/toggle/toggle_widget_impl.rs`)
- **Text Color Resolution Fix**: Previously, `text_galley` was created with `theme.foreground` before resolving interaction styles. Now `text_galley` uses resolved `style.fg` (`accent_foreground`, `muted_foreground`, etc.), ensuring readable contrast in all pressed/unpressed/hovered states.
- **RichText Formatting Preservation**: Replaced raw string extraction (`self.text.text().to_owned()`) with `self.text.into_galley(ui, None, font_size, TextStyle::Button)`. This preserves font attributes such as `.strong()`, `.italics()`, custom font sizes, and explicit font family assignments.

### Font Registration (`src/theme/setup_fonts.rs`)
- Added explicit registration for `FontFamily::Name("bold")` mapping to `Geist-Bold.ttf` in `FontDefinitions`. This allows widgets and labels to render heavy bold font weights.

---

## 3. New Themes & Examples Created

### Selector change reporting

`Select` and `SelectValue` now mark their returned `egui::Response` as changed
only when a different option is selected. The bound value is updated before
the response is returned, matching egui's standard widget contract and allowing
editor consumers to react to selection changes in the same frame.

### 10 Theme Families / 20 Variants

All themes include explicit doc-comments and contribution attribution for upstream design systems ([shadcn/ui](https://ui.shadcn.com) and [Tailwind CSS](https://tailwindcss.com) color scales):

| Theme Name | Module Path | Dark Variant | Light Variant | Color Concept |
| :--- | :--- | :--- | :--- | :--- |
| **Nova (Default)** | [`shadcn_theme_{light,dark}.rs`](file:///d:/programacion/pyrpg/src_rust/vendor/egui-shadcn/src/theme) | `dark()` | `light()` | Original shadcn/ui Nova neutral palette. |
| **Violet** | [`shadcn_theme_violet.rs`](file:///d:/programacion/pyrpg/src_rust/vendor/egui-shadcn/src/theme/shadcn_theme_violet.rs) | `violet_dark()` | `violet_light()` | Violet-600 (`#7c3aed`) accent with soft lavender grey light background (`#ebebf2`). |
| **Sky Blue** | [`shadcn_theme_sky.rs`](file:///d:/programacion/pyrpg/src_rust/vendor/egui-shadcn/src/theme/shadcn_theme_sky.rs) | `sky_dark()` | `sky_light()` | Electric Cerulean (`#0ea5e9`) accent with soft ice slate light background (`#e8eef4`). |
| **Obsidian** | [`shadcn_theme_obsidian.rs`](file:///d:/programacion/pyrpg/src_rust/vendor/egui-shadcn/src/theme/shadcn_theme_obsidian.rs) | `obsidian_dark()` | `obsidian_light()` | Volcanic jet glass (`#050505`) with smoky quartz light background (`#ebebee`). |
| **Pink** | [`shadcn_theme_pink.rs`](file:///d:/programacion/pyrpg/src_rust/vendor/egui-shadcn/src/theme/shadcn_theme_pink.rs) | `pink_dark()` | `pink_light()` | Hot Pink (`#ec4899`) accent with soft blush rose light background (`#f4e6ee`). |
| **Terracotta** | [`shadcn_theme_terracotta.rs`](file:///d:/programacion/pyrpg/src_rust/vendor/egui-shadcn/src/theme/shadcn_theme_terracotta.rs) | `terracotta_dark()` | `terracotta_light()` | Burnt sienna rust (`#c2410c`) accent with warm clay mist light background (`#f4eae4`). |
| **Rainbow** | [`shadcn_theme_rainbow.rs`](file:///d:/programacion/pyrpg/src_rust/vendor/egui-shadcn/src/theme/shadcn_theme_rainbow.rs) | `rainbow_dark()` | `rainbow_light()` | Prismatic spectrum balance with soft lavender mist light background (`#eeeaf5`). |
| **Heatmap** | [`shadcn_theme_heatmap.rs`](file:///d:/programacion/pyrpg/src_rust/vendor/egui-shadcn/src/theme/shadcn_theme_heatmap.rs) | `heatmap_dark()` | `heatmap_light()` | Thermal molten amber (`#f59e0b`) accent with warm sand light background (`#f4eede`). |
| **Cyber Aurora** | [`shadcn_theme_cyber_aurora.rs`](file:///d:/programacion/pyrpg/src_rust/vendor/egui-shadcn/src/theme/shadcn_theme_cyber_aurora.rs) | `cyber_aurora_dark()` | `cyber_aurora_light()` | Bioluminescent synthwave cyan (`#22d3ee`) with soft mint aurora light background (`#e0f2f0`). |
| **Childhood Nostalgia** | [`shadcn_theme_nostalgia.rs`](file:///d:/programacion/pyrpg/src_rust/vendor/egui-shadcn/src/theme/shadcn_theme_nostalgia.rs) | `nostalgia_dark()` | `nostalgia_light()` | Creamsicle orange (`#f97316`) accent with storybook parchment paper light background (`#f4eee2`). |

> [!NOTE]
> All custom light themes have been softened (backgrounds in the ~RGB 230–248 range) to eliminate harsh white glare and protect eye comfort during extended coding sessions.

---

### New Demo: Theme Gallery ([`examples/theme_gallery.rs`](file:///d:/programacion/pyrpg/src_rust/vendor/egui-shadcn/examples/theme_gallery.rs))
- **Live Dropdown Selector**: Uses `SelectValue` bound directly to `self.selected_palette` for instant, non-blocking theme switching.
- **Color Token Inspector**: Displays visual color swatches for active theme tokens (`background`, `card`, `primary`, `accent`, `border`, `ring`, etc.).
- **Interactive Component Sandbox**: Displays buttons, badges, alerts, inputs, checkboxes, switches, radio groups, sliders, progress bars, and tabs.
- **Cargo Target**: Registered as `[[example]] name = "theme_gallery"` in `Cargo.toml`.
- **Executable**: Compiled to `target/release/examples/theme_gallery.exe`.

---

## 4. `set_shadcn_theme` Only Partially Themed egui (bug, fixed in this fork)

### Symptom

Found while building a consuming app's theme selector: switching between a light shadcn palette
and a dark one visibly changed shadcn widgets (`Card`, `Button`, `Badge`, …) but left `DragValue`,
`TextEdit`, scrollbars, `egui::Grid` row striping, `egui_plot` charts, and any panel without an
explicit `.frame()` painted in whatever `egui::Theme` the *operating system* reported
(`ThemePreference::System` is egui's default). Two themes visibly stacked in one window — worst when
the shadcn palette and the OS theme disagreed (a light palette on a dark-mode desktop, or vice versa),
where roughly half the UI read as legible and the other half as a mismatched grey box.

### Root cause (`src/theme/shadcn_theme_ext.rs`)

`set_shadcn_theme`'s previous body:

```rust
fn set_shadcn_theme(&self, theme: ShadcnTheme) {
    self.all_styles_mut(|style| {
        style.visuals.window_fill = theme.popover;
        style.visuals.window_stroke = egui::Stroke::new(1.0, theme.border);
        style.visuals.window_shadow = egui::Shadow { /* ... */ };
    });
    self.data_mut(|d| d.insert_temp::<ShadcnTheme>(egui::Id::NULL, theme));
}
```

`egui::Visuals` has dozens of fields beyond `window_*` — `panel_fill`, `override_text_color`,
`faint_bg_color`, `extreme_bg_color`, `code_bg_color`, `warn_fg_color`/`error_fg_color`,
`selection`, `text_cursor`, and the five `widgets.{noninteractive,inactive,hovered,active,open}`
`WidgetVisuals` that drive every stock interactive widget's background/stroke/text color. None of
those were ever touched, so they stayed at whatever `egui::Visuals::dark()`/`::light()` produces for
the OS-reported theme — independent of, and frequently in conflict with, the shadcn palette actually
requested.

### Fix

Added `visuals_from(&ShadcnTheme) -> egui::Visuals` (same file), which starts from
`Visuals::dark()`/`::light()` and overrides the full set above from the theme's own tokens:

| `Visuals` field | Source `ShadcnTheme` field(s) |
|---|---|
| `panel_fill` | `background` |
| `override_text_color` | `Some(foreground)` |
| `hyperlink_color` | `primary` |
| `faint_bg_color`, `code_bg_color` | `muted` |
| `extreme_bg_color` | `input` |
| `warn_fg_color` | `destructive.gamma_multiply(0.75)` |
| `error_fg_color` | `destructive` |
| `window_fill` / `window_stroke` | `popover` / `border` (unchanged from before) |
| `selection.bg_fill` / `.stroke` | `primary.gamma_multiply(0.4)` / `ring` |
| `text_cursor.stroke.color` | `foreground` |
| `widgets.noninteractive` | `card` bg, `border` stroke, `muted_foreground` text |
| `widgets.inactive` / `open` | `secondary` bg, `border` stroke, `secondary_foreground` text |
| `widgets.hovered` | `accent` bg, `ring` stroke, `accent_foreground` text |
| `widgets.active` | `primary` bg, `ring` stroke, `primary_foreground` text |
| every `widgets.*.corner_radius` | `radius` |

`set_shadcn_theme` now calls `visuals_from` and assigns the result to `style.visuals` wholesale
(still inside the existing `all_styles_mut`, so both of egui's internal dark/light `Style`s get the
same shadcn-derived visuals — matching this function's pre-existing behavior of making the theme
uniform regardless of which `egui::Theme` the host reports).

**No signature change.** "Dark" is inferred from `background`'s relative luminance
(`0.2126*r + 0.7152*g + 0.0722*b < 0.5`) rather than threaded through as a new `dark: bool`
parameter, so every existing call site — all 20 theme constructors, every consumer already calling
`ctx.set_shadcn_theme(theme)` — keeps compiling and behaving correctly with zero changes.

### Tests added (`shadcn_theme_ext.rs`)

- `dark_and_light_backgrounds_are_classified_correctly` — sanity check on the two Nova defaults.
- `every_bundled_theme_background_is_classified_as_expected` — table-driven, all 20 constructors
  (10 families × dark/light), asserting the luminance classifier agrees with each constructor's own
  name.
- `visuals_from_dark_theme_uses_theme_colors_not_egui_defaults` — asserts `panel_fill`,
  `override_text_color`, `extreme_bg_color`, `faint_bg_color`, and `widgets.active.bg_fill` all equal
  the source theme's tokens rather than `Visuals::dark()`'s hardcoded defaults — the direct
  regression test for the bug this fixes.
