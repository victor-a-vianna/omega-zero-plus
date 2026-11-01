use std::sync::OnceLock;

use crate::cart::{Board, Spec, PAD};
use crate::silhouette::{depth_map, rasterise_detail, rasterise_svg, Detail};

const W: u32 = 500;
const H: u32 = 500;
const LABEL: (u32, u32, u32, u32) = (24, 210, 452, 252);

fn mask() -> &'static [u8] {
    static VALUE: OnceLock<Vec<u8>> = OnceLock::new();
    VALUE.get_or_init(|| {
        rasterise_svg(include_str!("../../assets/vectrex_cart.svg"), W, H)
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
        rasterise_detail(include_str!("../../assets/vectrex_cart_detail.svg"), W, H)
            .unwrap_or_else(|| Detail::blank(W, H))
    })
}

pub const THICK: f32 = 0.08;
pub const SHELF_SCALE: f32 = 0.50;

pub(crate) fn spec() -> Spec {
    Spec {
        w: W,
        h: H,
        label: LABEL,
        label_round: 0,
        mask: mask(),
        depth: depth(),
        detail: detail(),
        max_h: (LABEL.3 - 2 * PAD) as f32,
        rim: 5,
        board: Board {
            x: (0.04, 0.96),
            top: 0.05,
            pins: (0.08, 0.92),
            contacts_from: 0.96,
            traces_from: 0.90,
        },
    }
}

pub(crate) fn outline() -> (Vec<(f32, f32)>, (f32, f32)) {
    (
        vec![
            (12., 8.),
            (488., 8.),
            (496., 16.),
            (496., 488.),
            (488., 496.),
            (12., 496.),
            (4., 488.),
            (4., 16.),
        ],
        (500., 500.),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authored_asset_has_a_silhouette_and_monochrome_relief() {
        assert_eq!(mask().len(), (W * H) as usize);
        assert_eq!(mask()[0], 0);
        assert!(mask()[(H as usize / 2) * W as usize + W as usize / 2] > 192);
        assert!(detail().shadow.iter().any(|v| *v != 0));
        assert!(detail().highlight.iter().any(|v| *v != 0));
    }
}
