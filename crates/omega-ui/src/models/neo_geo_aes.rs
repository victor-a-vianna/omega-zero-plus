//! Neo Geo AES cartridge shell.

use std::sync::OnceLock;

use crate::cart::{Board, Spec, PAD};
use crate::silhouette::{depth_map, rasterise_detail, rasterise_svg, Detail};

const SHELL_SVG: &str = include_str!("../../assets/neo_geo_aes_cart.svg");
const DETAIL_SVG: &str = include_str!("../../assets/neo_geo_aes_cart_detail.svg");

pub const THICK: f32 = 0.14;
pub const SHELF_SCALE: f32 = 0.58;
pub const W: u32 = 600;
pub const H: u32 = 410;
const LABEL: (u32, u32, u32, u32) = (137, 10, 326, 194);
const OUTLINE: &[(f32, f32)] = &[
    (137.0, 0.0),
    (463.0, 0.0),
    (495.0, 20.0),
    (592.0, 20.0),
    (600.0, 28.0),
    (600.0, 386.0),
    (576.0, 410.0),
    (24.0, 410.0),
    (0.0, 386.0),
    (0.0, 28.0),
    (8.0, 20.0),
    (62.0, 20.0),
    (105.0, 16.0),
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
        label_round: 7,
        mask: mask(),
        depth: depth(),
        detail: detail(),
        max_h: (LABEL.3 - 2 * PAD) as f32,
        rim: 7,
        board: Board {
            x: (0.08, 0.92),
            top: 0.06,
            pins: (0.12, 0.88),
            contacts_from: 0.88,
            traces_from: 0.78,
        },
    }
}
