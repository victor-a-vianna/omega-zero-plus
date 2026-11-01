use crate::cart::{Board, Spec, PAD};
use crate::silhouette::{depth_map, rasterise_detail, rasterise_svg, Detail};
use std::sync::OnceLock;
const W: u32 = 430;
const H: u32 = 500;
const LABEL: (u32, u32, u32, u32) = (61, 302, 308, 122);
fn mask() -> &'static [u8] {
    static V: OnceLock<Vec<u8>> = OnceLock::new();
    V.get_or_init(|| {
        rasterise_svg(
            include_str!("../../assets/famicom_disk_system_cart.svg"),
            W,
            H,
        )
        .unwrap_or_else(|| vec![255; (W * H) as usize])
    })
}
fn depth() -> &'static [u8] {
    static V: OnceLock<Vec<u8>> = OnceLock::new();
    V.get_or_init(|| depth_map(mask(), W as usize, H as usize))
}
fn detail() -> &'static Detail {
    static V: OnceLock<Detail> = OnceLock::new();
    V.get_or_init(|| {
        rasterise_detail(
            include_str!("../../assets/famicom_disk_system_cart_detail.svg"),
            W,
            H,
        )
        .unwrap_or_else(|| Detail::blank(W, H))
    })
}
pub const THICK: f32 = 0.08;
pub const SHELF_SCALE: f32 = 0.5;
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
            x: (0.08, 0.92),
            top: 0.08,
            pins: (0.2, 0.8),
            contacts_from: 0.92,
            traces_from: 0.82,
        },
    }
}
pub(crate) fn outline() -> (Vec<(f32, f32)>, (f32, f32)) {
    (
        vec![
            (8., 0.),
            (54., 0.),
            (66., 18.),
            (168., 18.),
            (180., 0.),
            (250., 0.),
            (262., 18.),
            (364., 18.),
            (376., 0.),
            (422., 0.),
            (430., 9.),
            (430., 492.),
            (422., 500.),
            (8., 500.),
            (0., 492.),
            (0., 9.),
        ],
        (430., 500.),
    )
}
