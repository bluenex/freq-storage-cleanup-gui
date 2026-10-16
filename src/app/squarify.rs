use eframe::egui::{Rect, pos2, vec2};

/// Worst aspect ratio in a row laid along a side of length `side`.
fn worst(row: &[f64], side: f64) -> f64 {
    let sum: f64 = row.iter().sum();
    let max = row.iter().cloned().fold(f64::MIN, f64::max);
    let min = row.iter().cloned().fold(f64::MAX, f64::min);
    let side2 = side * side;
    (side2 * max / (sum * sum)).max(sum * sum / (side2 * min))
}

/// Squarified treemap (Bruls et al.). `sizes` should be sorted largest first.
/// Returns one rect per size, in the same order.
pub fn squarify(sizes: &[u64], rect: Rect) -> Vec<Rect> {
    let mut out = vec![Rect::NOTHING; sizes.len()];
    let total: f64 = sizes.iter().map(|&s| s.max(1) as f64).sum();
    let area = f64::from(rect.area());
    if sizes.is_empty() || area <= 0.0 {
        return out;
    }
    let areas: Vec<f64> = sizes.iter().map(|&s| s.max(1) as f64 / total * area).collect();

    let mut free = rect;
    let mut start = 0;
    while start < areas.len() {
        let side = f64::from(free.width().min(free.height())).max(1.0);
        let mut end = start + 1;
        while end < areas.len() && worst(&areas[start..=end], side) <= worst(&areas[start..end], side) {
            end += 1;
        }

        let thickness = (areas[start..end].iter().sum::<f64>() / side) as f32;
        let mut offset = 0.0;
        for i in start..end {
            let length = (areas[i] / f64::from(thickness)) as f32;
            out[i] = if free.width() >= free.height() {
                Rect::from_min_size(pos2(free.left(), free.top() + offset), vec2(thickness, length))
            } else {
                Rect::from_min_size(pos2(free.left() + offset, free.top()), vec2(length, thickness))
            };
            offset += length;
        }
        if free.width() >= free.height() {
            free.min.x += thickness;
        } else {
            free.min.y += thickness;
        }
        start = end;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fills_the_rect_proportionally() {
        let rect = Rect::from_min_size(pos2(0.0, 0.0), vec2(600.0, 400.0));
        let rects = squarify(&[6, 6, 4, 3, 2, 2, 1], rect);
        let areas: Vec<f32> = rects.iter().map(|r| r.area()).collect();
        let total: f32 = areas.iter().sum();
        assert!((total - rect.area()).abs() < 1.0);
        assert!((areas[0] - rect.area() * 6.0 / 24.0).abs() < 1.0);
        assert!(rects.iter().all(|r| rect.expand(0.01).contains_rect(*r)));
    }
}
