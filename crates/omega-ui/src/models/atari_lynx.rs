use std::sync::OnceLock;

use crate::cart::{Board, Spec, PAD};
use crate::silhouette::{depth_map, rasterise_detail, rasterise_svg, Detail};

const W: u32 = 500;
const H: u32 = 500;
const LABEL: (u32, u32, u32, u32) = (16, 104, 468, 300);

fn mask() -> &'static [u8] {
    static VALUE: OnceLock<Vec<u8>> = OnceLock::new();
    VALUE.get_or_init(|| {
        rasterise_svg(include_str!("../../assets/atari_lynx_cart.svg"), W, H)
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
        rasterise_detail(
            include_str!("../../assets/atari_lynx_cart_detail.svg"),
            W,
            H,
        )
        .unwrap_or_else(|| Detail::blank(W, H))
    })
}

pub const THICK: f32 = 0.12;
pub const SHELF_SCALE: f32 = 0.5;

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
        rim: 4,
        board: Board {
            x: (0.05, 0.95),
            top: 0.08,
            pins: (0.08, 0.92),
            contacts_from: 0.82,
            traces_from: 0.72,
        },
    }
}

pub(crate) fn outline() -> (Vec<(f32, f32)>, (f32, f32)) {
    (
        vec![
            (24., 0.),
            (476., 0.),
            (493., 10.),
            (500., 34.),
            (500., 474.),
            (493., 493.),
            (474., 500.),
            (26., 500.),
            (7., 493.),
            (0., 474.),
            (0., 34.),
            (7., 10.),
        ],
        (500., 500.),
    )
}
