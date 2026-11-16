use std::time::Duration;

use egui::{Color32, Context, RawInput, Shape, ViewportId, pos2, vec2};
use egui_shadcn::{AreaChart, AreaSeries, DatePicker, DatePickerState, Pagination, Resizable};

#[test]
fn dashed_border_ignores_invalid_lengths_and_bounds_dense_patterns() {
    let ctx = Context::default();
    let rect = egui::Rect::from_min_size(pos2(0.0, 0.0), vec2(8.0, 4.0));
    let stroke = egui::Stroke::new(1.0, Color32::RED);
    for (dash, gap, expected) in [
        (0.0, 0.0, 0),
        (f32::NAN, 2.0, 0),
        (2.0, -2.0, 0),
        (2.0, 0.0, 4),
        (f32::MIN_POSITIVE, f32::MIN_POSITIVE, 4),
        (2.0, 2.0, 6),
    ] {
        let mut output = ctx.run_ui(RawInput::default(), |ui| {
            egui_shadcn::paint::paint_dashed_rect::paint_dashed_rect(
                ui.painter(),
                rect,
                0.0,
                stroke,
                dash,
                gap,
            );
        });
        output.textures_delta.clear();
        let lines = output
            .shapes
            .iter()
            .filter(
                |shape| matches!(shape.shape, Shape::LineSegment { stroke: s, .. } if s == stroke),
            )
            .count();
        assert_eq!(lines, expected, "dash={dash}, gap={gap}");
    }
}

#[test]
fn pagination_handles_zero_pages_and_a_zero_size_window() {
    let ctx = Context::default();
    let mut current = usize::MAX;
    ctx.run_ui(RawInput::default(), |ui| {
        Pagination::new(0).max_visible(0).show(ui, &mut current);
        assert_eq!(current, usize::MAX);
        Pagination::new(10).max_visible(0).show(ui, &mut current);
        assert_eq!(current, 9);
        current = 0;
        Pagination::new(10).max_visible(0).show(ui, &mut current);
        assert_eq!(current, 0);
    })
    .textures_delta
    .clear();
}

#[test]
fn two_resizables_seed_independently_without_resetting_on_the_next_frame() {
    let ctx = Context::default();
    for (before, after) in [((0.0, 0.0), (0.25, 0.75)), ((0.4, 0.6), (0.4, 0.6))] {
        let (mut first, mut second) = before;
        ctx.run_ui(RawInput::default(), |ui| {
            Resizable::new(0.25).show(ui, &mut first, |_| {}, |_| {});
            Resizable::new(0.75).show(ui, &mut second, |_| {}, |_| {});
        })
        .textures_delta
        .clear();
        assert_eq!((first, second), after);
    }
}

#[test]
fn chart_fills_to_the_axis_unless_stacked() {
    for stacked in [false, true] {
        let ctx = Context::default();
        let mut axis_y = 0.0;
        let mut output = ctx.run_ui(RawInput::default(), |ui| {
            let chart = AreaChart::new(vec!["A".into(), "B".into()])
                .series(AreaSeries {
                    values: vec![1.0, 3.0],
                    color: Color32::RED,
                })
                .series(AreaSeries {
                    values: vec![3.0, 1.0],
                    color: Color32::GREEN,
                });
            let chart = if stacked { chart.stacked() } else { chart };
            axis_y = chart.show(ui).rect.bottom() - 24.0;
        });
        output.textures_delta.clear();
        let fills: Vec<_> = output
            .shapes
            .iter()
            .filter_map(|shape| {
                if let Shape::Mesh(mesh) = &shape.shape {
                    Some(mesh)
                } else {
                    None
                }
            })
            .collect();
        assert_eq!(fills.len(), 2);
        for i in 0..2 {
            let baseline_y = if stacked {
                fills[0].vertices[i].pos.y
            } else {
                axis_y
            };
            assert_eq!(fills[1].vertices[i + 2].pos.y, baseline_y);
        }
    }
}

#[test]
fn toasts_schedule_the_earliest_expiration_and_then_the_next() {
    let ctx = Context::default();
    ctx.global_style_mut(|style| style.animation_time = 0.0);
    let mut toasts = egui_shadcn::ToastState::new();
    toasts.add("First", egui_shadcn::ToastVariant::Info, 0.0);
    toasts.add("Second", egui_shadcn::ToastVariant::Info, 2.0);

    for (now, expected_delay) in [(3.0, 1.0), (4.0, 2.0)] {
        // Allow egui to finish initial sizing/repaint passes before reading the timer.
        let mut delay = Duration::ZERO;
        for _ in 0..4 {
            let input = RawInput {
                time: Some(now),
                ..Default::default()
            };
            let mut output = ctx.run_ui(input, |ui| toasts.show(ui.ctx()));
            output.textures_delta.clear();
            delay = output.viewport_output[&ViewportId::ROOT].repaint_delay;
        }
        assert_eq!(delay, Duration::from_secs_f64(expected_delay));
    }
}

#[test]
fn date_picker_browsing_and_dismissal_preserve_the_selected_date() {
    let ctx = Context::default();
    let mut state = DatePickerState {
        year: 2026,
        month: 1,
        day: 31,
    };
    let mut frame = |input| {
        let mut popup_id = egui::Id::NULL;
        let mut output = ctx.run_ui(input, |ui| {
            let response = DatePicker::new().show(ui, &mut state);
            popup_id = response.id.with("date_picker_popup");
        });
        output.textures_delta.clear();
        (popup_id, output.shapes)
    };
    let (popup_id, _) = frame(RawInput::default());
    egui::Popup::open_id(&ctx, popup_id);
    frame(RawInput::default());
    let (_, shapes) = frame(RawInput::default());
    let heading = shapes
        .iter()
        .find_map(|shape| match &shape.shape {
            Shape::Text(text) if text.galley.text() == "January 2026" => Some(text),
            _ => None,
        })
        .expect("calendar month heading");
    let arrow = heading.pos
        + vec2(
            heading.galley.size().x + 15.0,
            heading.galley.size().y / 2.0,
        );
    for pressed in [true, false] {
        frame(RawInput {
            events: vec![
                egui::Event::PointerMoved(arrow),
                egui::Event::PointerButton {
                    pos: arrow,
                    button: egui::PointerButton::Primary,
                    pressed,
                    modifiers: egui::Modifiers::NONE,
                },
            ],
            ..Default::default()
        });
    }
    assert_eq!(
        ctx.data(|data| data.get_temp(popup_id.with("browsing_month"))),
        Some((2026_i32, 2_u32))
    );
    egui::Popup::close_id(&ctx, popup_id);
    frame(RawInput::default());
    assert_eq!(state.format(), "2026-01-31");
}
