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
