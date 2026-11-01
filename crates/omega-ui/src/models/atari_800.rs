//! Atari 400/800 family cartridge shell.

use std::sync::OnceLock;

use crate::cart::{Board, Spec, PAD};
use crate::silhouette::{depth_map, rasterise_detail, rasterise_svg, Detail};

const SHELL_SVG: &str = include_str!("../../assets/atari_800_cart.svg");
const DETAIL_SVG: &str = include_str!("../../assets/atari_800_cart_detail.svg");

pub const THICK: f32 = 0.16;
pub const SHELF_SCALE: f32 = 0.72;
pub const W: u32 = 280;
pub const H: u32 = 340;

const LABEL: (u32, u32, u32, u32) = (18, 15, 244, 230);
const UNITS: (f32, f32) = (280.0, 340.0);
const OUTLINE: &[(f32, f32)] = &[
    (15.0, 0.0),
    (265.0, 0.0),
    (280.0, 15.0),
    (280.0, 325.0),
    (265.0, 340.0),
    (15.0, 340.0),
    (0.0, 325.0),
    (0.0, 15.0),
];

pub(crate) fn outline() -> (Vec<(f32, f32)>, (f32, f32)) {
    (OUTLINE.to_vec(), UNITS)
}

fn mask() -> &'static [u8] {
    static MASK: OnceLock<Vec<u8>> = OnceLock::new();
    MASK.get_or_init(|| {
        rasterise_svg(SHELL_SVG, W, H).unwrap_or_else(|| vec![255; (W * H) as usize])
    })
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
        label_round: 4,
        mask: mask(),
        depth: depth(),
        detail: detail(),
        max_h: (LABEL.3 - 2 * PAD) as f32,
        rim: 5,
        board: Board {
            x: (0.12, 0.88),
            top: 0.10,
            pins: (0.22, 0.78),
            contacts_from: 0.90,
            traces_from: 0.80,
        },
    }
}
