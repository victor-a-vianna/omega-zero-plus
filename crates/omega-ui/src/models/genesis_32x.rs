//! The sculpted Sega 32X cartridge shell.
//!
//! This is a visual model only. It is intentionally not associated with ROM detection or an
//! emulator platform yet.

use std::sync::OnceLock;

use crate::cart::{Board, Spec, PAD};
use crate::silhouette::{depth_map, rasterise_detail, rasterise_svg, Detail};

const SHELL_SVG: &str = include_str!("../../assets/genesis_32x_cart.svg");
const DETAIL_SVG: &str = include_str!("../../assets/genesis_32x_cart_detail.svg");

/// The 32X shell is visibly deeper than the standard Genesis cartridge.
pub const THICK: f32 = 0.20;
pub const SHELF_SCALE: f32 = 1.0;

pub const W: u32 = 360;
pub const H: u32 = 228;

const UNITS: (f32, f32) = (500.0, 316.0);
const LABEL: (u32, u32, u32, u32) = (52, 16, 256, 105);
const LABEL_ROUND: u32 = 7;
const RIM: u32 = 7;

/// Right half of the broad retail shell, clockwise from the top centre.
const HALF: &[(f32, f32)] = &[
    (250.0, 0.0),
    (457.0, 0.0),
    (481.0, 8.0),
    (496.0, 25.0),
    (500.0, 42.0),
    (500.0, 286.0),
    (492.0, 306.0),
    (466.0, 316.0),
    (250.0, 316.0),
];

pub(crate) fn outline() -> (Vec<(f32, f32)>, (f32, f32)) {
    let mut all = HALF.to_vec();
    all.extend(HALF.iter().rev().map(|&(x, y)| (UNITS.0 - x, y)));
    (all, UNITS)
}

pub fn silhouette(w: u32, h: u32) -> Vec<u8> {
    rasterise_svg(SHELL_SVG, w, h).unwrap_or_else(|| vec![255; (w * h) as usize])
}

fn mask() -> &'static [u8] {
    static MASK: OnceLock<Vec<u8>> = OnceLock::new();
    MASK.get_or_init(|| silhouette(W, H))
}

fn depth() -> &'static [u8] {
    static DEPTH: OnceLock<Vec<u8>> = OnceLock::new();
    DEPTH.get_or_init(|| depth_map(mask(), W as usize, H as usize))
}

fn detail() -> &'static Detail {
    static DETAIL: OnceLock<Detail> = OnceLock::new();
    DETAIL.get_or_init(|| rasterise_detail(DETAIL_SVG, W, H).unwrap_or_else(|| Detail::blank(W, H)))
}

pub(crate) fn spec() -> Spec {
    Spec {
        w: W,
        h: H,
        label: LABEL,
        label_round: LABEL_ROUND,
        mask: mask(),
        depth: depth(),
        detail: detail(),
        max_h: (LABEL.3 - 2 * PAD) as f32,
        rim: RIM,
        board: Board {
            x: (0.10, 0.90),
            top: 0.08,
            pins: (0.28, 0.72),
            contacts_from: 0.88,
            traces_from: 0.79,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outline_is_symmetric_and_has_a_straight_foot() {
        let (points, units) = outline();
        assert_eq!(units, UNITS);
        for &(x, y) in &points {
            assert!(
                points.contains(&(UNITS.0 - x, y)),
                "missing mirror of {x},{y}"
            );
        }
        assert!(points.iter().filter(|p| p.1 == UNITS.1).count() >= 2);
    }

    #[test]
    fn silhouette_has_clear_corners_and_an_opaque_centre() {
        let face = silhouette(W, H);
        assert_eq!(face.len(), (W * H) as usize);
        assert_eq!(face[0], 0);
        assert!(face[((H / 2) * W + W / 2) as usize] > 192);
    }
}
