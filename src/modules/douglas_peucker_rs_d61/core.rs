use crate::types::Pt;

pub fn perpendicular_distance(p: Pt, a: Pt, b: Pt) -> f64 {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    if dx == 0.0 && dy == 0.0 {
        return ((p.x - a.x).powi(2) + (p.y - a.y).powi(2)).sqrt();
    }
    let num = (dy * p.x - dx * p.y + b.x * a.y - b.y * a.x).abs();
    num / (dx * dx + dy * dy).sqrt()
}

pub fn simplify(points: &[Pt], epsilon: f64) -> Vec<Pt> {
    if points.len() <= 2 {
        return points.to_vec();
    }
    let mut keep = vec![false; points.len()];
    keep[0] = true;
    keep[points.len() - 1] = true;
    let mut stack = vec![(0usize, points.len() - 1)];
    while let Some((a, b)) = stack.pop() {
        let mut max_dist = -1.0;
        let mut idx = None;
        for i in (a + 1)..b {
            let d = perpendicular_distance(points[i], points[a], points[b]);
            if d > max_dist {
                max_dist = d;
                idx = Some(i);
            }
        }
        if let Some(i) = idx {
            if max_dist > epsilon {
                keep[i] = true;
                stack.push((a, i));
                stack.push((i, b));
            }
        }
    }
    points
        .iter()
        .enumerate()
        .filter_map(|(i, &p)| if keep[i] { Some(p) } else { None })
        .collect()
}
