//! The Framesmith mark and wordmark, drawn in code (no image assets): a film frame with sprocket
//! holes, struck by a spark, on an ember tile.

use egui::{Color32, Pos2, Rect, Stroke, StrokeKind, pos2, vec2};

/// Ember, the brand colour.
pub const EMBER: Color32 = Color32::from_rgb(255, 106, 61);
/// Amber, the highlight of the tile.
pub const AMBER: Color32 = Color32::from_rgb(255, 178, 63);
/// The deep ink the mark is cut from.
pub const INK: Color32 = Color32::from_rgb(28, 22, 30);

/// Paint the square app mark into `r`.
pub fn paint_mark(p: &egui::Painter, r: Rect) {
    let s = r.width().min(r.height());
    let r = Rect::from_center_size(r.center(), vec2(s, s));
    let round = (s * 0.24).min(255.0);
    // Tile: ember with a lighter amber top half (two layers: no gradient primitive needed).
    p.rect_filled(r, round, EMBER);
    let top = Rect::from_min_max(r.min, pos2(r.max.x, r.center().y));
    p.rect_filled(top, egui::CornerRadius { nw: round as u8, ne: round as u8, sw: 0, se: 0 }, AMBER.gamma_multiply(0.55));
    // The film frame.
    let frame = Rect::from_center_size(r.center() + vec2(0.0, s * 0.02), vec2(s * 0.62, s * 0.5));
    let w = (s * 0.07).max(1.0);
    p.rect_stroke(frame, s * 0.06, Stroke::new(w, INK), StrokeKind::Middle);
    // Sprocket holes along the top and bottom edges.
    let holes = 4;
    for i in 0..holes {
        let fx = frame.min.x + frame.width() * (i as f32 + 0.5) / holes as f32;
        for y in [frame.min.y + s * 0.075, frame.max.y - s * 0.075] {
            p.rect_filled(Rect::from_center_size(pos2(fx, y), vec2(s * 0.065, s * 0.05)), s * 0.012, INK);
        }
    }
    // The spark: a four-point star where the hammer strikes.
    spark(p, pos2(frame.max.x - s * 0.02, frame.min.y + s * 0.02), s * 0.17, Color32::WHITE);
}

/// A four-point star centred at `c` with radius `r`.
pub fn spark(p: &egui::Painter, c: Pos2, r: f32, color: Color32) {
    let k = r * 0.28;
    let pts = vec![
        pos2(c.x, c.y - r),
        pos2(c.x + k, c.y - k),
        pos2(c.x + r, c.y),
        pos2(c.x + k, c.y + k),
        pos2(c.x, c.y + r),
        pos2(c.x - k, c.y + k),
        pos2(c.x - r, c.y),
        pos2(c.x - k, c.y - k),
    ];
    p.add(egui::Shape::convex_polygon(pts, color, Stroke::new(r * 0.12, INK)));
}

/// Draw the mark and the "Framesmith" wordmark `height` points tall with its left edge at
/// `left_center` (vertically centred). Returns the drawn rect.
pub fn paint_wordmark(ui: &egui::Ui, left_center: Pos2, height: f32, dark: bool) -> Option<Rect> {
    let p = ui.painter();
    let mark = Rect::from_min_size(pos2(left_center.x, left_center.y - height / 2.0), vec2(height, height));
    paint_mark(p, mark);
    let ink = if dark { Color32::from_rgb(246, 242, 238) } else { INK };
    let g = p.layout_no_wrap("Framesmith".to_string(), crate::theme::Tokens::semibold(height * 0.82), ink);
    let pos = pos2(mark.max.x + height * 0.32, left_center.y - g.size().y / 2.0);
    let text = Rect::from_min_size(pos, g.size());
    p.galley(pos, g, ink);
    Some(mark.union(text))
}
