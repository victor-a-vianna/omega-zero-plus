use std::sync::OnceLock;

use crate::cart::{Board, Spec, PAD};
use crate::silhouette::{depth_map, rasterise_detail, rasterise_svg, Detail};

const W: u32 = 500;
const H: u32 = 430;
const LABEL: (u32, u32, u32, u32) = (47, 137, 406, 221);

fn mask() -> &'static [u8] {
    static VALUE: OnceLock<Vec<u8>> = OnceLock::new();
    VALUE.get_or_init(|| {
        rasterise_svg(include_str!("../../assets/pokemon_mini_cart.svg"), W, H)
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
            include_str!("../../assets/pokemon_mini_cart_detail.svg"),
            W,
            H,
        )
        .unwrap_or_else(|| Detail::blank(W, H))
    })
}

pub const THICK: f32 = 0.12;
pub const SHELF_SCALE: f32 = 0.58;

pub(crate) fn spec() -> Spec {
    Spec {
        w: W,
        h: H,
        label: LABEL,
        label_round: 14,
        mask: mask(),
        depth: depth(),
        detail: detail(),
        max_h: (LABEL.3 - 2 * PAD) as f32,
        rim: 4,
        board: Board {
            x: (0.10, 0.90),
            top: 0.08,
            pins: (0.14, 0.86),
            contacts_from: 0.88,
            traces_from: 0.78,
        },
    }
}

pub(crate) fn outline() -> (Vec<(f32, f32)>, (f32, f32)) {
    (
        vec![
            (35., 0.),
            (465., 0.),
            (489., 8.),
            (493., 25.),
            (500., 116.),
            (488., 138.),
            (494., 322.),
            (490., 357.),
            (475., 383.),
            (459., 399.),
            (365., 426.),
            (250., 430.),
            (135., 426.),
            (41., 399.),
            (25., 383.),
            (10., 357.),
            (6., 322.),
            (12., 138.),
            (0., 116.),
            (7., 25.),
            (11., 8.),
        ],
        (500., 430.),
    )
}
