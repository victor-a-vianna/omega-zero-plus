use std::sync::OnceLock;

use crate::cart::{Board, Spec, PAD};
use crate::silhouette::{depth_map, rasterise_detail, rasterise_svg, Detail};

const W: u32 = 620;
const H: u32 = 500;
const LABEL: (u32, u32, u32, u32) = (39, 124, 542, 244);

fn mask() -> &'static [u8] {
    static VALUE: OnceLock<Vec<u8>> = OnceLock::new();
    VALUE.get_or_init(|| {
        rasterise_svg(include_str!("../../assets/atari_jaguar_cart.svg"), W, H)
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
            include_str!("../../assets/atari_jaguar_cart_detail.svg"),
            W,
            H,
        )
        .unwrap_or_else(|| Detail::blank(W, H))
    })
}

pub const THICK: f32 = 0.11;
pub const SHELF_SCALE: f32 = 0.52;

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
            x: (0.06, 0.94),
            top: 0.08,
            pins: (0.10, 0.90),
            contacts_from: 0.86,
            traces_from: 0.76,
        },
    }
}

pub(crate) fn outline() -> (Vec<(f32, f32)>, (f32, f32)) {
    (
        vec![
            (22., 0.),
            (598., 0.),
            (614., 14.),
            (620., 72.),
            (615., 94.),
            (603., 104.),
            (603., 486.),
            (589., 500.),
            (31., 500.),
            (17., 486.),
            (17., 104.),
            (5., 94.),
            (0., 72.),
            (6., 14.),
        ],
        (620., 500.),
    )
}
