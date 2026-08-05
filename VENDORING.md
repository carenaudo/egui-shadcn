# Vendoring notes & Delta Comparison

This is a vendored fork of [egui-shadcn](https://github.com/pjankiewicz/egui-shadcn) by Pawel Jankiewicz (MIT License).

- **Upstream**: https://github.com/pjankiewicz/egui-shadcn
- **Forked at**: `fa5ceee` — *Add public paint_icon_svg for custom Lucide-grammar icons*

---

## 1. Compatibility Fixes (`egui 0.35` & Updated Cargo Packages)

### `Cargo.toml`
| Dependency | Upstream | Here | Notes |
|---|---|---|---|
| `egui` | `0.33` | `0.35` | Workspace dependency |
| `egui_flex` | `0.5` | `0.7` | First version on egui 0.35 |
| `eframe` (dev) | `0.33` | `0.35` | Workspace dev-dependency |

### API Migrations
- `src/theme/shadcn_theme_ext.rs` — Replaced deprecated `Context::style_mut` with `all_styles_mut` to maintain single-style window fill, stroke, and shadow overrides for popups.
- `src/widgets/{input,textarea,input_group}` — Replaced boolean `.frame(false)` with `Frame::NONE.inner_margin(Margin::symmetric(4, 2))` to prevent 4px/2px layout shifts.
- `Painter::rect_stroke` — Updated calls across widgets to match `egui 0.35` `StrokeKind` parameters.

---

## 2. Fixes Needed for Custom Code & Widgets

- **`Toggle` Widget (`src/widgets/toggle/toggle_widget_impl.rs`)**:
  - Resolved `text_galley` using `style.fg` (`accent_foreground`, `muted_foreground`, etc.) instead of hardcoded `theme.foreground`, ensuring high contrast text across all states.
  - Replaced raw string extraction with `self.text.into_galley(...)` to preserve `RichText` font formatting (`.strong()`, `.italics()`, custom font sizes).
- **Font Registration (`src/theme/setup_fonts.rs`)**:
  - Registered `Geist-Bold.ttf` under `FontFamily::Name("bold")` in `FontDefinitions`.

---

## 3. New Themes & Examples

### 10 Theme Families / 20 Variants
Located under `src/theme/`:
- `shadcn_theme_violet.rs` (`violet_dark()`, `violet_light()`)
- `shadcn_theme_sky.rs` (`sky_dark()`, `sky_light()`)
- `shadcn_theme_obsidian.rs` (`obsidian_dark()`, `obsidian_light()`)
- `shadcn_theme_pink.rs` (`pink_dark()`, `pink_light()`)
- `shadcn_theme_terracotta.rs` (`terracotta_dark()`, `terracotta_light()`)
- `shadcn_theme_rainbow.rs` (`rainbow_dark()`, `rainbow_light()`)
- `shadcn_theme_heatmap.rs` (`heatmap_dark()`, `heatmap_light()`)
- `shadcn_theme_cyber_aurora.rs` (`cyber_aurora_dark()`, `cyber_aurora_light()`)
- `shadcn_theme_nostalgia.rs` (`nostalgia_dark()`, `nostalgia_light()`)

*All custom light themes feature softened, eye-friendly backgrounds (~RGB 230–248) to eliminate eye strain.*

### Theme Gallery Demo
- `examples/theme_gallery.rs`: Interactive sandbox for inspecting and switching all 20 theme variants live.
- Registered as `[[example]] name = "theme_gallery"` in `Cargo.toml`.
