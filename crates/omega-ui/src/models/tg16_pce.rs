//! TurboGrafx-16, PC Engine and SuperGrafx HuCard shell.

use std::sync::OnceLock;

use crate::cart::{Board, Spec, PAD};
use crate::silhouette::{depth_map, rasterise_detail, rasterise_svg, Detail};

const SHELL_SVG: &str = include_str!("../../assets/tg16_pce_cart.svg");
const DETAIL_SVG: &str = include_str!("../../assets/tg16_pce_cart_detail.svg");

pub const THICK: f32 = 0.035;
pub const SHELF_SCALE: f32 = 0.5;
pub const W: u32 = 340;
pub const H: u32 = 500;
const LABEL: (u32, u32, u32, u32) = (20, 196, 300, 244);
const OUTLINE: &[(f32, f32)] = &[
    (22.0, 0.0),
    (318.0, 0.0),
    (340.0, 22.0),
    (340.0, 470.0),
    (310.0, 500.0),
    (30.0, 500.0),
    (0.0, 470.0),
    (0.0, 22.0),
];

pub(crate) fn outline() -> (Vec<(f32, f32)>, (f32, f32)) {
    (OUTLINE.to_vec(), (W as f32, H as f32))
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
        label_round: 15,
        mask: mask(),
        depth: depth(),
        detail: detail(),
        max_h: (LABEL.3 - 2 * PAD) as f32,
        rim: 4,
        board: Board {
            x: (0.08, 0.92),
            top: 0.04,
            pins: (0.12, 0.88),
            contacts_from: 0.82,
            traces_from: 0.72,
        },
    }
}
