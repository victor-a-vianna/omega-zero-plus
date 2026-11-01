use std::sync::OnceLock;

use crate::cart::{Board, Spec, PAD};
use crate::silhouette::{depth_map, rasterise_detail, rasterise_svg, Detail};

const W: u32 = 640;
const H: u32 = 400;
const LABEL: (u32, u32, u32, u32) = (34, 70, 572, 221);

fn mask() -> &'static [u8] {
    static VALUE: OnceLock<Vec<u8>> = OnceLock::new();
    VALUE.get_or_init(|| {
        rasterise_svg(include_str!("../../assets/wonderswan_cart.svg"), W, H)
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
            include_str!("../../assets/wonderswan_cart_detail.svg"),
            W,
            H,
        )
        .unwrap_or_else(|| Detail::blank(W, H))
    })
}

pub const THICK: f32 = 0.075;
pub const SHELF_SCALE: f32 = 0.52;

pub(crate) fn spec() -> Spec {
    Spec {
        w: W,
        h: H,
        label: LABEL,
        label_round: 13,
        mask: mask(),
        depth: depth(),
        detail: detail(),
        max_h: (LABEL.3 - 2 * PAD) as f32,
        rim: 4,
        board: Board {
            x: (0.07, 0.93),
            top: 0.08,
            pins: (0.09, 0.91),
            contacts_from: 0.84,
            traces_from: 0.76,
        },
    }
}

pub(crate) fn outline() -> (Vec<(f32, f32)>, (f32, f32)) {
    (
        vec![
            (18., 14.),
            (622., 14.),
            (634., 26.),
            (634., 334.),
            (622., 346.),
            (604., 346.),
            (604., 377.),
            (589., 386.),
            (51., 386.),
            (36., 377.),
            (36., 346.),
            (18., 346.),
            (6., 334.),
            (6., 26.),
        ],
        (640., 400.),
    )
}
