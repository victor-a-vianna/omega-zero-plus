use std::sync::OnceLock;

use crate::cart::{Board, Spec, PAD};
use crate::silhouette::{depth_map, rasterise_detail, rasterise_svg, Detail};

const W: u32 = 360;
const H: u32 = 360;
const LABEL: (u32, u32, u32, u32) = (50, 137, 260, 223);
pub const THICK: f32 = 0.06;
pub const SHELF_SCALE: f32 = 0.62;

fn mask() -> &'static [u8] {
    static VALUE: OnceLock<Vec<u8>> = OnceLock::new();
    VALUE.get_or_init(|| {
        rasterise_svg(include_str!("../../assets/generic_floppy.svg"), W, H)
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
        rasterise_detail(include_str!("../../assets/generic_floppy_detail.svg"), W, H)
            .unwrap_or_else(|| Detail::blank(W, H))
    })
}

pub(crate) fn spec() -> Spec {
    Spec {
        w: W,
        h: H,
        label: LABEL,
        label_round: 10,
        mask: mask(),
        depth: depth(),
        detail: detail(),
        max_h: (LABEL.3 - 2 * PAD) as f32,
        rim: 4,
        board: Board {
            x: (0.14, 0.86),
            top: 0.38,
            pins: (0.2, 0.8),
            contacts_from: 0.9,
            traces_from: 0.8,
        },
    }
}

pub(crate) fn outline() -> (Vec<(f32, f32)>, (f32, f32)) {
    (
        vec![(0., 0.), (325., 0.), (360., 35.), (360., 360.), (0., 360.)],
        (360., 360.),
    )
}
