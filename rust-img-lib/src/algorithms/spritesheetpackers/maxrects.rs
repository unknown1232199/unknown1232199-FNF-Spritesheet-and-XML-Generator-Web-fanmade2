use crate::algorithms::{FitRect, PackingRectangle};
use crate::utils::PackError;

#[derive(Clone, Copy)]
struct FreeRect { x: u32, y: u32, w: u32, h: u32 }

fn intersects(a: FreeRect, b: FreeRect) -> bool {
    a.x < b.x + b.w && a.x + a.w > b.x && a.y < b.y + b.h && a.y + a.h > b.y
}

fn contains(a: FreeRect, b: FreeRect) -> bool {
    b.x >= a.x && b.y >= a.y && b.x + b.w <= a.x + a.w && b.y + b.h <= a.y + a.h
}

fn split_free(free: &mut Vec<FreeRect>, used: FreeRect) {
    let mut next = Vec::new();
    for f in free.drain(..) {
        if !intersects(f, used) {
            next.push(f);
            continue;
        }
        if used.x > f.x && used.x < f.x + f.w {
            next.push(FreeRect { x: f.x, y: f.y, w: used.x - f.x, h: f.h });
        }
        if used.x + used.w < f.x + f.w {
            next.push(FreeRect { x: used.x + used.w, y: f.y, w: f.x + f.w - (used.x + used.w), h: f.h });
        }
        if used.y > f.y && used.y < f.y + f.h {
            next.push(FreeRect { x: f.x, y: f.y, w: f.w, h: used.y - f.y });
        }
        if used.y + used.h < f.y + f.h {
            next.push(FreeRect { x: f.x, y: used.y + used.h, w: f.w, h: f.y + f.h - (used.y + used.h) });
        }
    }
    next.retain(|r| r.w > 0 && r.h > 0);
    let mut keep = vec![true; next.len()];
    for i in 0..next.len() {
        if !keep[i] { continue; }
        for j in 0..next.len() {
            if i != j && keep[j] && contains(next[i], next[j]) {
                keep[j] = false;
            }
        }
    }
    *free = next.into_iter().enumerate().filter_map(|(i, r)| if keep[i] { Some(r) } else { None }).collect();
}

fn attempt(items: &[PackingRectangle<u64>], side: u32, heuristic: usize) -> Option<Vec<FitRect>> {
    let mut ordered = items.to_vec();
    ordered.sort_by(|a, b| {
        let area_a = a.width as u64 * a.height as u64;
        let area_b = b.width as u64 * b.height as u64;
        area_b.cmp(&area_a)
            .then_with(|| b.width.max(b.height).cmp(&a.width.max(a.height)))
            .then_with(|| b.width.cmp(&a.width))
    });

    let mut free = vec![FreeRect { x: 0, y: 0, w: side, h: side }];
    let mut placed = Vec::with_capacity(ordered.len());

    for item in ordered {
        let mut best: Option<(usize, (u64, u64, u64, u64), FreeRect)> = None;
        for (idx, f) in free.iter().enumerate() {
            if item.width > f.w || item.height > f.h { continue; }
            let dw = f.w - item.width;
            let dh = f.h - item.height;
            let short = dw.min(dh) as u64;
            let long = dw.max(dh) as u64;
            let area = (f.w as u64 * f.h as u64) - (item.width as u64 * item.height as u64);
            let score = match heuristic {
                1 => (area, short, f.y as u64, f.x as u64),
                2 => (long, short, f.y as u64, f.x as u64),
                3 => (f.y as u64 + item.height as u64, f.x as u64, short, long),
                _ => (short, long, f.y as u64, f.x as u64),
            };
            if best.as_ref().map_or(true, |(_, old, _)| score < *old) {
                best = Some((idx, score, FreeRect { x: f.x, y: f.y, w: item.width, h: item.height }));
            }
        }
        let (_, _, used) = best?;
        split_free(&mut free, used);
        placed.push(FitRect::new(used.x, used.y, item.width, item.height, item.id));
    }
    Some(placed)
}

/// MaxRects: test several placement heuristics and retain the tightest result.
/// Auto mode grows its candidate bin until every frame fits; a fixed size is a hard limit.
pub fn pack_maxrects(items: Vec<PackingRectangle<u64>>, max_size: u32) -> Result<(u32, u32, Vec<FitRect>), PackError> {
    if items.is_empty() { return Ok((0, 0, Vec::new())); }
    let max_dim = items.iter().map(|r| r.width.max(r.height)).max().unwrap_or(1);
    if max_size > 0 && max_dim > max_size { return Err(PackError); }

    let mut side = if max_size > 0 {
        max_size
    } else {
        let area: u64 = items.iter().map(|r| r.width as u64 * r.height as u64).sum();
        let estimate = (area as f64).sqrt().ceil() as u32;
        max_dim.max(estimate).max(1).next_power_of_two()
    };

    loop {
        let mut best: Option<(u64, u32, u32, Vec<FitRect>)> = None;
        for heuristic in 0..4 {
            if let Some(fits) = attempt(&items, side, heuristic) {
                let width = fits.iter().map(|r| r.x + r.width).max().unwrap_or(0);
                let height = fits.iter().map(|r| r.y + r.height).max().unwrap_or(0);
                let area = width as u64 * height as u64;
                let candidate = (area, width.max(height), width + height, fits);
                if best.as_ref().map_or(true, |b| (candidate.0, candidate.1, candidate.2) < (b.0, b.1, b.2)) {
                    best = Some(candidate);
                }
            }
        }
        if let Some((_, _, _, fits)) = best {
            let width = fits.iter().map(|r| r.x + r.width).max().unwrap_or(0);
            let height = fits.iter().map(|r| r.y + r.height).max().unwrap_or(0);
            return Ok((width, height, fits));
        }
        if max_size > 0 || side >= 32768 { return Err(PackError); }
        side = side.saturating_mul(2);
        if side == 0 { return Err(PackError); }
    }
}
