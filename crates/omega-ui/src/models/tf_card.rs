use std::sync::OnceLock;

use crate::cart::{Board, Spec, PAD};
use crate::silhouette::{depth_map, rasterise_detail, rasterise_svg, Detail};

const W: u32 = 260;
const H: u32 = 338;
const LABEL: (u32, u32, u32, u32) = (31, 64, 190, 221);
pub const THICK: f32 = 0.055;
pub const SHELF_SCALE: f32 = 0.58;

fn mask() -> &'static [u8] {
    static VALUE: OnceLock<Vec<u8>> = OnceLock::new();
    VALUE.get_or_init(|| {
        rasterise_svg(include_str!("../../assets/tf_card.svg"), W, H)
            .unwrap_or_else(|| vec![255; (W * H) as usize])
    })
}

fn depth() -> &'static [u8] {
    static VALUE: OnceLock<Vec<u8>> = OnceLock::new();
    VALUE.get_or_init(|| depth_map(mask(), W as usize, H as usize))
}

fn detail() -> &'static Detail {
    static VALUE: OnceLock<Detail> = OnceLock::new();
    VALUE.get_or_init(|| {
        rasterise_detail(include_str!("../../assets/tf_card_detail.svg"), W, H)
            .unwrap_or_else(|| Detail::blank(W, H))
    })
}

pub(crate) fn spec() -> Spec {
    Spec {
        w: W,
        h: H,
        label: LABEL,
        label_round: 9,
        mask: mask(),
        depth: depth(),
        detail: detail(),
        max_h: (LABEL.3 - 2 * PAD) as f32,
        rim: 4,
        board: Board {
            x: (0.13, 0.87),
            top: 0.20,
            pins: (0.18, 0.82),
            contacts_from: 0.91,
            traces_from: 0.84,
        },
    }
}

pub(crate) fn outline() -> (Vec<(f32, f32)>, (f32, f32)) {
    (
        vec![
            (25., 0.),
            (222., 0.),
            (260., 38.),
            (260., 313.),
            (25., 338.),
            (0., 313.),
            (0., 73.),
            (10., 73.),
            (10., 47.),
            (0., 47.),
            (0., 25.),
        ],
        (260., 338.),
    )
}
