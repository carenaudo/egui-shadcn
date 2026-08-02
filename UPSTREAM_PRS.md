# Candidate PRs back to pjankiewicz/egui-shadcn

This fork diverges from `upstream/main` at `fa5ceee` by 37 files
(+670 / −229). Not all of it belongs upstream, and what does belongs in
separate PRs — the two bugfixes are worth landing on their own even if the
egui 0.35 upgrade is not wanted yet.

Our history mixes concerns by commit (`021908f` is the 0.35 port, `b9c797e` is
the clippy pass), so **do not `git cherry-pick`**. Every PR below is
file-scoped instead, which is why the file lists matter: each listed file's
entire diff against `fa5ceee` belongs to exactly one PR, verified, so
`git checkout <our-commit> -- <file>` produces a clean branch.

Ordered by how likely a maintainer is to take them.

---

## PR 1 — Fix `NumberInput` never showing focus

**Kind:** bug · **Size:** 1 file, ~33 lines · **egui version:** independent

`number_input_widget_impl.rs` paints the identical `theme.ring` stroke in both
the focused and the hovered branch, so hovering a number input is visually
indistinguishable from focusing it. The comment three lines above already
states the intent — *"focus ring when focused, regular border otherwise"* —
and `input/input_style.rs` sets the crate convention (`ring` when focused,
`border` otherwise). The hovered branch now follows it.

The three branches collapse into one `rect_stroke` with a computed colour, so
all three states read at a glance. Clippy finds the original as
`if_same_then_else`.

Verified present verbatim at `fa5ceee`.

```
src/widgets/number_input/number_input_widget_impl.rs
```

## PR 2 — Make `Resizable::new(initial_fraction)` take effect

**Kind:** bug · **Size:** 1 file, ~13 lines · **egui version:** independent

`Resizable::new` clamps and stores `initial_fraction`, but `resizable_show.rs`
reads the split position solely from the caller's `&mut f32` and never
consults it. `Resizable::new(0.3)` and `new(0.9)` behave identically today.

`show` now seeds the caller's value on the first frame for a given `ui.id()`,
using the crate's existing `ui.data` / `get_temp` / `insert_temp` pattern.
**No signature change**, so it is not a breaking change for existing callers —
it only starts honouring an argument that was already documented as
*"the initial width ratio of the left panel"*. Clippy finds the original as
`dead_code` on the unread field.

Verified present verbatim at `fa5ceee`.

```
src/widgets/resizable/resizable_show.rs
```

## PR 3 — `paint_icon`: characterization tests, then `Pos2`/`TSTransform` params

**Kind:** quality · **Size:** 1 file, +324 / −115 · **egui version:** independent

Five functions in `icons/paint_icon.rs` carry geometry as loose `f32` pairs and
trip `too_many_arguments`:

| Function | Before | After |
|---|---|---|
| `tessellate_cubic` | 11 | 6 |
| `tessellate_quad` | 9 | 5 |
| `paint_ellipse` | 8 | 5 |
| `arc_segment_to_cubic` | 11 | 5 |
| `tessellate_arc` | 12 | 5 |

The recurring `scale: f32, offset: Pos2` pair is exactly `emath::TSTransform` —
`map_point` computed `pos2(x * scale + offset.x, …)`, which is what
`TSTransform::mul_pos` does (`scaling * pos + translation`), so the arithmetic
is bit-identical rather than merely equivalent. Coordinate pairs become
`Pos2`/`Vec2`; `ArcParams` carries the SVG arc flags and `ArcBasis` the
rotated-ellipse frame shared by the two arc functions. It also simplifies six
inline `pos2(x * scale + offset.x, …)` expressions in `paint_element` that were
not lint sites at all.

`egui::emath::TSTransform` is reachable on **egui 0.33** (`pub use
epaint::emath;` in `egui/src/lib.rs`), so this does **not** depend on PR 5.

**Offer this as two commits, and say so in the PR description.** This is SVG
path tessellation with no existing coverage — the crate's tests stop at the
`PathCommand` level in `parse_path.rs` / `parse_svg.rs` and never reach the
Bézier maths. So:

1. **Add characterization tests** pinning the exact points emitted for a cubic,
   a quad, a rotated large arc, and the zero-radius degenerate case, at a
   deliberately awkward scale and offset.
2. **Refactor**, with those tests passing unchanged.

That ordering is the argument for the PR: it demonstrates the refactor is
behaviour-preserving instead of asserting it. The tests are a record of
behaviour, not intent — a deliberate change to the curve maths means
recapturing them, and the module doc comment says so.

```
src/icons/paint_icon.rs
```

## PR 4 — Clippy housekeeping

**Kind:** quality · **Size:** 24 files, mostly ~6 lines each · **egui version:** independent

Clears every remaining clippy warning. Afterwards the crate passes:

```
cargo clippy --all-targets -- -D warnings -A clippy::module_inception
```

- `new_without_default` ×16 — `impl Default` delegating to `new()` for `Alert`,
  `Calendar`, `Card`, `DatePicker`, `Dialog`, `Drawer`, `HoverCard`, `Item`,
  `Menubar`, `Popover`, `PropertyGrid`, `Sheet`, `Sidebar`, `Spinner`,
  `StatusBar`, `Toolbar`
- `collapsible_if` ×3 — `button_widget_impl.rs`, `slider_widget_impl.rs`
- `let_and_return` ×2 — `tabs_show.rs`
- `nonminimal_bool` + `bool_comparison` — `button_widget_impl.rs` had
  `!text.is_empty() == false && self.icon.is_some()`. The behaviour was right
  (icon-only means no text plus an icon); only the triple negative was wrong
- `manual_div_ceil`, `needless_range_loop` — `calendar_show.rs`,
  `area_chart_show.rs`
- examples — `approx_constant` (a demo value of `3.14` reading as π),
  `derivable_impls`, `needless_borrows_for_generic_args`, `manual_div_ceil`

Plus a `[lints.clippy] module_inception = "allow"` in `Cargo.toml` with its
rationale: the crate is deliberately one public item per file, so every
`widgets/foo/` holds a `foo.rs`. That lint alone accounts for 59 of the
original 90 warnings, and silencing it is far cheaper than restructuring 249
files. **A maintainer may reasonably prefer the layout change instead** — worth
asking in the PR rather than assuming.

### Two things to flag in the PR description

- **MSRV.** The `collapsible_if` fixes use let-chains, stable since Rust 1.88
  in edition 2024. Upstream's `Cargo.toml` declares no `rust-version`, so this
  silently raises the floor. If that is unwelcome, those three sites can keep
  their nested `if`s and take a targeted `#[allow]` instead.
- **Formatting.** The crate is **not** rustfmt-clean (208 diffs at `fa5ceee`),
  so `cargo fmt` was deliberately not run — the churn would bury the change.
  The let-chain bodies were re-indented by hand.

```
src/widgets/{alert,calendar,card,date_picker,dialog,drawer,hover_card,item,
             menubar,popover,property_grid,sheet,sidebar,spinner,status_bar,
             toolbar}/*.rs          (new_without_default)
src/widgets/button/button_widget_impl.rs
src/widgets/slider/slider_widget_impl.rs
src/widgets/tabs/tabs_show.rs
src/widgets/calendar/calendar_show.rs
src/widgets/area_chart/area_chart_show.rs
Cargo.toml                          (the [lints] block only)
examples/dashboard.rs               (partly — see note below)
examples/component_dashboard.rs     (partly — see note below)
```

⚠️ The two example files are the **one exception** to the file-scoped rule:
their diffs mix PR 4 and PR 5 changes. Split by hunk, or land PR 5 first and
let PR 4 rebase on top.

## PR 6 — `Command` is not keyboard-navigable

**Kind:** bug · **Size:** 1 file, ~90 lines plus tests · **egui version:** independent

`command/command_show.rs` handles only Escape. The row highlight follows
`r.hovered()` and selection fires only on `r.clicked()`, so a command palette —
the one widget whose entire purpose is keyboard-first invocation — cannot be
driven from the keyboard. You open it with a chord, type to filter, and then
have to reach for the mouse.

Added:

- An active-row index in `Context` temp data, keyed per palette
- Up/Down to move it with wrapping, Home/End to jump, Enter to select
- Reset to the first row whenever the filter text changes, so a narrowed list
  cannot run whatever coincidentally sits at the old offset
- The active row painted with `theme.accent`, the same as hover, so keyboard
  and mouse agree on what is selected
- `scroll_to_rect` on the active row, inside a new `ScrollArea` capped at
  320 px — without one, a list longer than the viewport grew the palette off
  screen and arrowing past the fold went nowhere

The navigation keys are **consumed** (`input_mut` + `consume_key`) before the
search `Input` requests focus and renders. A focused `TextEdit` claims
Up/Down/Home/End/Enter for caret movement, so reading them afterwards moves the
cursor instead of the selection — worth calling out in the PR, as it is the
non-obvious part.

Seven headless tests cover default selection, arrowing, wrapping, Home/End,
selecting within a filtered list, the reset-on-filter-change case, and Escape.

```
src/widgets/command/command_show.rs
```

## PR 5 — Upgrade to egui 0.35

**Kind:** breaking · **Size:** 11 files

The largest PR and the one most likely to need maintainer buy-in, since it
drops support for 0.33 users and warrants a version bump. The surprise is how
small it is: across ~17.6k lines, egui 0.33 → 0.35 breaks exactly two things.

- `Context::style_mut` was removed when 0.35 moved to per-theme styles.
  `all_styles_mut` preserves the old single-style semantics.
- `TextEdit::frame` changed from `bool` to `Frame` in **0.34** (3 sites).

  The second one has a trap worth spelling out in the PR: `.frame(false)` only
  suppressed *painting*, while the widget's default
  `margin: Margin::symmetric(4, 2)` still applied. Since 0.34 a custom `Frame`
  replaces `margin` outright, so a bare `Frame::NONE` shifts text 4px left and
  2px up. The new `paint::text_edit_frame` helper restores the margin and
  documents why, so all three sites stay pixel-identical.

Dependencies: `egui` 0.33 → 0.35, `egui_flex` 0.5 → 0.7 (first release on
0.35), `eframe` dev-dep 0.33 → 0.35.

The examples need the eframe/egui 0.35 migration — `App::update` → `App::ui`,
`SidePanel`/`TopBottomPanel` → the unified `Panel`, `.show(ctx, …)` →
`.show(ui, …)`, and `exact_width`/`default_width` → `exact_size`/`default_size`.
`component_dashboard` keeps a `let ctx = ui.ctx().clone()` because the overlay
widgets (`Dialog`, `AlertDialog`, `Sheet`, `Drawer`, `Command`, `Toast`) are
still `Context`-driven. One doctest in `setup_fonts.rs` drifts too.

```
Cargo.toml                          (dependency versions only)
src/theme/shadcn_theme_ext.rs
src/theme/setup_fonts.rs
src/paint/text_edit_frame.rs        (new)
src/paint/mod.rs
src/widgets/input/input_widget_impl.rs
src/widgets/textarea/textarea_widget_impl.rs
src/widgets/input_group/input_group_show.rs
examples/{demo,shadcn_demo,dashboard,component_dashboard}.rs
```

---

## Not for upstream

- `VENDORING.md` — records this fork's divergence; meaningless upstream.
- `UPSTREAM_PRS.md` — this file.

## Preparing a branch

Each PR starts from upstream, never from our `main`:

```bash
git fetch upstream
```

```bash
git checkout -b fix-number-input-focus upstream/main
```

```bash
git checkout b9c797e -- src/widgets/number_input/number_input_widget_impl.rs
```

Then build and test against **upstream's** egui 0.33 before opening the PR —
PRs 1–4 are all claimed to be version-independent, and that claim is worth
checking rather than trusting. PR 5 is the only one that requires the bump.
