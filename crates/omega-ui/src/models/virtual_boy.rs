use std::sync::OnceLock;

use crate::cart::{Board, Spec, PAD};
use crate::silhouette::{depth_map, rasterise_detail, rasterise_svg, Detail};

const W: u32 = 560;
const H: u32 = 350;
const LABEL: (u32, u32, u32, u32) = (74, 36, 412, 214);

fn mask() -> &'static [u8] {
    static VALUE: OnceLock<Vec<u8>> = OnceLock::new();
    VALUE.get_or_init(|| {
        rasterise_svg(include_str!("../../assets/virtual_boy_cart.svg"), W, H)
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
            include_str!("../../assets/virtual_boy_cart_detail.svg"),
            W,
            H,
        )
        .unwrap_or_else(|| Detail::blank(W, H))
    })
}

pub const THICK: f32 = 0.12;
pub const SHELF_SCALE: f32 = 0.6;

pub(crate) fn spec() -> Spec {
    Spec {
        w: W,
        h: H,
        label: LABEL,
        label_round: 8,
        mask: mask(),
        depth: depth(),
        detail: detail(),
        max_h: (LABEL.3 - 2 * PAD) as f32,
        rim: 4,
        board: Board {
            x: (0.10, 0.90),
            top: 0.08,
            pins: (0.16, 0.84),
            contacts_from: 0.90,
            traces_from: 0.80,
        },
    }
}

pub(crate) fn outline() -> (Vec<(f32, f32)>, (f32, f32)) {
    (
        vec![
            (28., 0.),
            (532., 0.),
            (560., 28.),
            (560., 298.),
            (540., 318.),
            (510., 318.),
            (510., 342.),
            (502., 350.),
            (58., 350.),
            (50., 342.),
            (50., 318.),
            (20., 318.),
            (0., 298.),
            (0., 28.),
        ],
        (560., 350.),
    )
}
