//! Dashed rectangle painting utility.

/// Paints a dashed border rectangle.
///
/// Invalid lengths or rectangles are ignored. A zero gap or a pattern too
/// dense to draw within the segment budget is rendered as solid edges.
pub fn paint_dashed_rect(
    painter: &egui::Painter,
    rect: egui::Rect,
    corner_radius: f32,
    stroke: egui::Stroke,
    dash_length: f32,
    gap_length: f32,
) {
    const MAX_DASHES_PER_EDGE: usize = 4096;

    if !rect.is_finite()
        || rect.is_negative()
        || !dash_length.is_finite()
        || dash_length <= 0.0
        || !gap_length.is_finite()
        || gap_length < 0.0
    {
        return;
    }

    // Approximate with small line segments along each edge.
    let edges: [(egui::Pos2, egui::Pos2); 4] = [
        (rect.left_top(), rect.right_top()),       // top
        (rect.right_top(), rect.right_bottom()),   // right
        (rect.right_bottom(), rect.left_bottom()), // bottom
        (rect.left_bottom(), rect.left_top()),     // left
    ];

    let _ = corner_radius; // corner rounding not applied to dash segments

    for (start, end) in edges {
        let dir = end - start;
        let length = dir.x.abs().max(dir.y.abs());
        if !length.is_finite() || length < 0.1 {
            continue;
        }
        let length = f64::from(length);
        let dash = f64::from(dash_length);
        let period = dash + f64::from(gap_length);
        let count = (length / period).ceil();
        if gap_length == 0.0 || count > MAX_DASHES_PER_EDGE as f64 {
            painter.line_segment([start, end], stroke);
            continue;
        }

        for index in 0..count as usize {
            let offset = index as f64 * period;
            let p0 = start + dir * (offset / length) as f32;
            let p1 = start + dir * ((offset + dash).min(length) / length) as f32;
            painter.line_segment([p0, p1], stroke);
        }
    }
}
