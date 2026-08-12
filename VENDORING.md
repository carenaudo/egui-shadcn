# Vendoring notes & Delta Comparison

This is a vendored fork of [egui-shadcn](https://github.com/pjankiewicz/egui-shadcn) by Pawel Jankiewicz (MIT License).

- **Upstream**: https://github.com/pjankiewicz/egui-shadcn
- **Forked at**: `fa5ceee` — *Add public paint_icon_svg for custom Lucide-grammar icons*

---

## 1. Compatibility Fixes (`egui 0.36` & Updated Cargo Packages)

### `Cargo.toml`
| Dependency | Upstream | Here | Notes |
|---|---|---|---|
| `egui` | `0.33` | `0.36` | Workspace dependency |
| `egui_flex` | `0.5` | `0.8` | First version on egui 0.36 |
| `eframe` (dev) | `0.33` | `0.36` | Workspace dev-dependency |

The `web/` wasm demo crate (pinned `egui`/`eframe` `0.33`, workspace-`exclude`d)
was removed rather than ported — it was already stale and unused.

### API Migrations
- `src/theme/shadcn_theme_ext.rs` — Replaced deprecated `Context::style_mut` with `all_styles_mut` to maintain single-style window fill, stroke, and shadow overrides for popups.
- `src/widgets/{input,textarea,input_group}` — Replaced boolean `.frame(false)` with `Frame::NONE.inner_margin(Margin::symmetric(4, 2))` to prevent 4px/2px layout shifts.
- `Painter::rect_stroke` — Updated calls across widgets to match `egui 0.35` `StrokeKind` parameters.
- `RawInput::modifiers` was removed in `egui 0.36` (modifiers now arrive as
  part of each `Event`); the one construction site
  (`crates/rpg_editor_ui/src/shortcuts.rs`, workspace-side) dropped the
  now-nonexistent field assignment.
- `egui 0.36`'s `epaint::TexturesDelta` gained a `Drop` impl that
  `debug_assert`s if it still holds unapplied deltas. Every headless test
  that calls `ctx.run_ui(...)` and discards the `FullOutput` without
  painting now explicitly calls `.textures_delta.clear()` on the result —
  see `src/widgets/command/command_show.rs` in this fork, and the
  equivalent sites across `rpg_editor_ui`'s test harnesses workspace-side.
- `egui_dock 0.20 → 0.21` added a required `TabViewer::id` method (workspace
  side, `crates/rpg_editor_ui/src/dock.rs`): implemented as
  `Id::new(*tab)`, which required adding `Hash` to `EditorTab`'s derive.

---

## 2. Fixes Needed for Custom Code & Widgets

- **`Toggle` Widget (`src/widgets/toggle/toggle_widget_impl.rs`)**:
  - Resolved `text_galley` using `style.fg` (`accent_foreground`, `muted_foreground`, etc.) instead of hardcoded `theme.foreground`, ensuring high contrast text across all states.
  - Replaced raw string extraction with `self.text.into_galley(...)` to preserve `RichText` font formatting (`.strong()`, `.italics()`, custom font sizes).
- **Font Registration (`src/theme/setup_fonts.rs`)**:
  - Registered `Geist-Bold.ttf` under `FontFamily::Name("bold")` in `FontDefinitions`.
- **`ShadcnThemeExt::set_shadcn_theme` only themed shadcn widgets (`src/theme/shadcn_theme_ext.rs`)**:
  - It previously overrode exactly three `Visuals` fields — `window_fill`, `window_stroke`,
    `window_shadow` — leaving `panel_fill`, `override_text_color`, `faint_bg_color`,
    `extreme_bg_color`, `code_bg_color`, `warn_fg_color`/`error_fg_color`, `selection`, and every
    `widgets.*` (`noninteractive`/`inactive`/`hovered`/`active`/`open`) at egui's own
    `Visuals::dark()`/`::light()` defaults for whatever `egui::Theme` the OS/host reports. Result:
    `Card`, `Button`, etc. painted the shadcn palette while `DragValue`, `TextEdit`, scrollbars,
    `egui::Grid` striping, `egui_plot`, and any un-`.frame()`d panel painted egui's own defaults —
    two themes stacked in the same window, worst on a light shadcn palette against a dark-mode OS
    (or vice versa).
  - Added a `visuals_from(&ShadcnTheme) -> egui::Visuals` that derives and applies a full `Visuals`
    from the theme's own tokens, called from `set_shadcn_theme`. "Dark" is inferred from
    `background`'s relative luminance rather than threaded through as a new parameter, so every
    existing call site (including all 20 theme constructors and every consumer) keeps working
    unmodified. Covered by three new tests in `shadcn_theme_ext.rs` (luminance classification for
    all 20 bundled themes, and that the derived `Visuals` actually reads from the theme rather than
    from egui's defaults).

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
