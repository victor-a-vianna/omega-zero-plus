use std::sync::OnceLock;

use crate::cart::{Board, Spec, PAD};
use crate::silhouette::{depth_map, rasterise_detail, rasterise_svg, Detail};

const W: u32 = 560;
const H: u32 = 500;
const LABEL: (u32, u32, u32, u32) = (54, 116, 452, 288);

fn mask() -> &'static [u8] {
    static VALUE: OnceLock<Vec<u8>> = OnceLock::new();
    VALUE.get_or_init(|| {
        rasterise_svg(include_str!("../../assets/supervision_cart.svg"), W, H)
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
            include_str!("../../assets/supervision_cart_detail.svg"),
            W,
            H,
        )
        .unwrap_or_else(|| Detail::blank(W, H))
    })
}

pub const THICK: f32 = 0.09;
pub const SHELF_SCALE: f32 = 0.52;

pub(crate) fn spec() -> Spec {
    Spec {
        w: W,
        h: H,
        label: LABEL,
        label_round: 18,
        mask: mask(),
        depth: depth(),
        detail: detail(),
        max_h: (LABEL.3 - 2 * PAD) as f32,
        rim: 5,
        board: Board {
            x: (0.09, 0.91),
            top: 0.08,
            pins: (0.12, 0.88),
            contacts_from: 0.89,
            traces_from: 0.82,
        },
    }
}

pub(crate) fn outline() -> (Vec<(f32, f32)>, (f32, f32)) {
    (
        vec![
            (24., 8.),
            (536., 8.),
            (548., 20.),
            (548., 434.),
            (536., 446.),
            (508., 446.),
            (508., 488.),
            (498., 496.),
            (62., 496.),
            (52., 488.),
            (52., 446.),
            (24., 446.),
            (12., 434.),
            (12., 20.),
        ],
        (560., 500.),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authored_assets_have_a_silhouette_and_monochrome_relief() {
        assert_eq!(mask().len(), (W * H) as usize);
        assert_eq!(mask()[0], 0);
        assert!(mask()[(H as usize / 2) * W as usize + W as usize / 2] > 192);
        assert!(detail().shadow.iter().any(|v| *v != 0));
        assert!(detail().highlight.iter().any(|v| *v != 0));
    }
}
