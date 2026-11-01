//! The North American Sega Genesis cartridge shell.
//!
//! This module only describes the visual model. It deliberately has no ROM, platform or
//! emulator mapping yet, so adding the shell cannot make an unsupported game launchable.

use std::sync::OnceLock;

use crate::cart::{Board, Spec, PAD};
use crate::silhouette::{depth_map, rasterise_detail, rasterise_svg, Detail};

const SHELL_SVG: &str = include_str!("../../assets/genesis_cart.svg");
const DETAIL_SVG: &str = include_str!("../../assets/genesis_cart_detail.svg");

/// Approximate depth-to-width ratio of a 16-bit Genesis cartridge shell.
pub const THICK: f32 = 0.17;

/// Keep the model at its authored pixel size on the shelf.
pub const SHELF_SCALE: f32 = 1.0;

/// Face texture dimensions. The 360:230 ratio follows the low, wide Genesis shell.
pub const W: u32 = 360;
pub const H: u32 = 230;

const UNITS: (f32, f32) = (550.0, 350.0);
/// The large front label used by the standard first-party Genesis shell. It sits left of the
/// front screw and reaches close to the shell's straight bottom edge.
const LABEL: (u32, u32, u32, u32) = (56, 5, 249, 205);
const LABEL_ROUND: u32 = 3;
const RIM: u32 = 6;

/// Full clockwise outline. Unlike a first-party shell, the reference has a grip notch only on
/// its left edge, so this model deliberately is not mirrored.
const OUTLINE: &[(f32, f32)] = &[
    (48.0, 0.0),
    (502.0, 0.0),
    (530.0, 7.0),
    (545.0, 23.0),
    (550.0, 44.0),
    (550.0, 326.0),
    (542.0, 342.0),
    (522.0, 350.0),
    (28.0, 350.0),
    (8.0, 342.0),
    (0.0, 326.0),
    (4.0, 217.0),
    (25.0, 217.0),
    (25.0, 151.0),
    (4.0, 151.0),
    (0.0, 44.0),
    (5.0, 23.0),
    (20.0, 7.0),
];

pub(crate) fn outline() -> (Vec<(f32, f32)>, (f32, f32)) {
    (OUTLINE.to_vec(), UNITS)
}

/// Rasterises the cartridge silhouette at an arbitrary output size.
pub fn silhouette(w: u32, h: u32) -> Vec<u8> {
    rasterise_svg(SHELL_SVG, w, h).unwrap_or_else(|| vec![255; (w * h) as usize])
}

fn mask() -> &'static [u8] {
    static MASK: OnceLock<Vec<u8>> = OnceLock::new();
    MASK.get_or_init(|| silhouette(W, H))
}

fn depth() -> &'static [u8] {
    static DEPTH: OnceLock<Vec<u8>> = OnceLock::new();
    DEPTH.get_or_init(|| depth_map(mask(), W as usize, H as usize))
}

fn detail() -> &'static Detail {
    static DETAIL: OnceLock<Detail> = OnceLock::new();
    DETAIL.get_or_init(|| rasterise_detail(DETAIL_SVG, W, H).unwrap_or_else(|| Detail::blank(W, H)))
}

pub(crate) fn spec() -> Spec {
    Spec {
        w: W,
        h: H,
        label: LABEL,
        label_round: LABEL_ROUND,
        mask: mask(),
        depth: depth(),
        detail: detail(),
        max_h: (LABEL.3 - 2 * PAD) as f32,
        rim: RIM,
        board: Board {
            x: (0.11, 0.89),
            top: 0.12,
            pins: (0.30, 0.70),
            contacts_from: 0.88,
            traces_from: 0.80,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outline_fills_the_authored_box_and_keeps_the_left_notch() {
        let (points, units) = outline();
        assert_eq!(units, UNITS);
        assert!(points.iter().any(|p| p.0 == 0.0));
        assert!(points.iter().any(|p| p.0 == UNITS.0));
        assert!(points.iter().any(|p| p.1 == 0.0));
        assert!(points.iter().any(|p| p.1 == UNITS.1));
        assert!(points.contains(&(25.0, 151.0)));
        assert!(points.contains(&(25.0, 217.0)));
        assert!(!points.contains(&(UNITS.0 - 25.0, 151.0)));
    }

    #[test]
    fn silhouette_has_clear_corners_and_an_opaque_body() {
        let face = silhouette(W, H);
        assert_eq!(face.len(), (W * H) as usize);
        assert_eq!(face[0], 0);
        assert!(face[((H - 1) * W + W / 2) as usize] > 192);
    }
}
