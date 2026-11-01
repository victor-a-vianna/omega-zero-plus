use std::sync::OnceLock;

use crate::cart::{Board, Spec, PAD};
use crate::silhouette::{depth_map, rasterise_detail, rasterise_svg, Detail};

const W: u32 = 500;
const H: u32 = 560;
const LABEL: (u32, u32, u32, u32) = (57, 139, 386, 308);

fn mask() -> &'static [u8] {
    static VALUE: OnceLock<Vec<u8>> = OnceLock::new();
    VALUE.get_or_init(|| {
        rasterise_svg(include_str!("../../assets/megaduck_cart.svg"), W, H)
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
        rasterise_detail(include_str!("../../assets/megaduck_cart_detail.svg"), W, H)
            .unwrap_or_else(|| Detail::blank(W, H))
    })
}

pub const THICK: f32 = 0.105;
pub const SHELF_SCALE: f32 = 0.49;

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
        rim: 5,
        board: Board {
            x: (0.12, 0.88),
            top: 0.07,
            pins: (0.14, 0.86),
            contacts_from: 0.86,
            traces_from: 0.78,
        },
    }
}

pub(crate) fn outline() -> (Vec<(f32, f32)>, (f32, f32)) {
    (
        vec![
            (28., 12.),
            (472., 12.),
            (488., 26.),
            (488., 464.),
            (474., 478.),
            (439., 478.),
            (439., 546.),
            (425., 556.),
            (75., 556.),
            (61., 546.),
            (61., 478.),
            (26., 478.),
            (12., 464.),
            (12., 26.),
        ],
        (500., 560.),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authored_assets_have_clear_corners_an_opaque_body_and_relief() {
        let face = mask();
        assert_eq!(face.len(), (W * H) as usize);
        assert_eq!(face[0], 0);
        assert!(face[(H as usize / 2) * W as usize + W as usize / 2] > 192);
        let relief = detail();
        assert!(relief.shadow.iter().any(|v| *v != 0));
        assert!(relief.highlight.iter().any(|v| *v != 0));
    }
}
