use std::sync::OnceLock;

use crate::cart::{Board, Spec, PAD};
use crate::silhouette::{depth_map, rasterise_detail, rasterise_svg, Detail};

const W: u32 = 400;
const H: u32 = 500;
const LABEL: (u32, u32, u32, u32) = (27, 60, 346, 230);

pub const THICK: f32 = 0.14;
pub const SHELF_SCALE: f32 = 0.5;

fn mask() -> &'static [u8] {
    static MASK: OnceLock<Vec<u8>> = OnceLock::new();
    MASK.get_or_init(|| {
        rasterise_svg(include_str!("../../assets/sufami_turbo_cart.svg"), W, H)
            .unwrap_or_else(|| vec![255; (W * H) as usize])
    })
}

fn depth() -> &'static [u8] {
    static DEPTH: OnceLock<Vec<u8>> = OnceLock::new();
    DEPTH.get_or_init(|| depth_map(mask(), W as usize, H as usize))
}

fn detail() -> &'static Detail {
    static DETAIL: OnceLock<Detail> = OnceLock::new();
    DETAIL.get_or_init(|| {
        rasterise_detail(
            include_str!("../../assets/sufami_turbo_cart_detail.svg"),
            W,
            H,
        )
        .unwrap_or_else(|| Detail::blank(W, H))
    })
}

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
        rim: 5,
        board: Board {
            x: (0.1, 0.9),
            top: 0.08,
            pins: (0.2, 0.8),
            contacts_from: 0.9,
            traces_from: 0.8,
        },
    }
}

pub(crate) fn outline() -> (Vec<(f32, f32)>, (f32, f32)) {
    (
        vec![
            (30.0, 0.0),
            (370.0, 0.0),
            (400.0, 32.0),
            (400.0, 470.0),
            (370.0, 500.0),
            (30.0, 500.0),
            (0.0, 470.0),
            (0.0, 32.0),
        ],
        (W as f32, H as f32),
    )
}
