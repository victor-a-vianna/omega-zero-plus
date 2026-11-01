//! NES/Famicom cartridges share one emulated platform but use their regional physical shells.

use std::sync::OnceLock;

use omega_store::{Cart, Region};

use crate::cart::{Board, Spec, PAD};
use crate::silhouette::{depth_map, rasterise_detail, rasterise_svg, Detail};

const NES_SVG: &str = include_str!("../../assets/nes_cart.svg");
const NES_DETAIL_SVG: &str = include_str!("../../assets/nes_cart_detail.svg");
const FAMICOM_SVG: &str = include_str!("../../assets/famicom_cart.svg");
const FAMICOM_DETAIL_SVG: &str = include_str!("../../assets/famicom_cart_detail.svg");

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum NesShell {
    Nes,
    Famicom,
}

pub const THICK: f32 = 0.14;
pub const SHELF_SCALE: f32 = 0.9;

const NES_W: u32 = 240;
const NES_H: u32 = 272;
const FAMICOM_W: u32 = 360;
const FAMICOM_H: u32 = 225;
const NES_LABEL: (u32, u32, u32, u32) = (98, 11, 105, 172);
const FAMICOM_LABEL: (u32, u32, u32, u32) = (27, 35, 306, 139);
const RIM: u32 = 5;

/// USA releases use the NES shell. Europe and Japan use Famicom, as selected for this frontend;
/// unknown releases also fall back to Famicom.
pub fn shell_of(cart: &Cart) -> NesShell {
    match Region::from_tags(&cart.stem) {
        Some(Region::Usa) => NesShell::Nes,
        _ => NesShell::Famicom,
    }
}

pub(crate) fn outline(shell: NesShell) -> (Vec<(f32, f32)>, (f32, f32)) {
    match shell {
        NesShell::Nes => (
            vec![
                (18.0, 14.0),
                (53.0, 14.0),
                (53.0, 16.0),
                (117.0, 16.0),
                (117.0, 14.0),
                (248.0, 14.0),
                (248.0, 16.0),
                (282.0, 16.0),
                (282.0, 267.0),
                (266.0, 267.0),
                (266.0, 324.0),
                (34.0, 324.0),
                (34.0, 267.0),
                (18.0, 267.0),
            ],
            (300.0, 340.0),
        ),
        NesShell::Famicom => (
            vec![
                (26.0, 4.0),
                (534.0, 4.0),
                (556.0, 26.0),
                (556.0, 311.0),
                (516.0, 311.0),
                (516.0, 346.0),
                (44.0, 346.0),
                (44.0, 311.0),
                (4.0, 311.0),
                (4.0, 26.0),
            ],
            (560.0, 350.0),
        ),
    }
}

pub fn silhouette(shell: NesShell, w: u32, h: u32) -> Vec<u8> {
    let svg = match shell {
        NesShell::Nes => NES_SVG,
        NesShell::Famicom => FAMICOM_SVG,
    };
    rasterise_svg(svg, w, h).unwrap_or_else(|| vec![255; (w * h) as usize])
}

fn face_box(shell: NesShell) -> (u32, u32) {
    match shell {
        NesShell::Nes => (NES_W, NES_H),
        NesShell::Famicom => (FAMICOM_W, FAMICOM_H),
    }
}

fn mask(shell: NesShell) -> &'static [u8] {
    static NES: OnceLock<Vec<u8>> = OnceLock::new();
    static FAMICOM: OnceLock<Vec<u8>> = OnceLock::new();
    let lock = match shell {
        NesShell::Nes => &NES,
        NesShell::Famicom => &FAMICOM,
    };
    lock.get_or_init(|| {
        let (w, h) = face_box(shell);
        silhouette(shell, w, h)
    })
}

fn depth(shell: NesShell) -> &'static [u8] {
    static NES: OnceLock<Vec<u8>> = OnceLock::new();
    static FAMICOM: OnceLock<Vec<u8>> = OnceLock::new();
    let lock = match shell {
        NesShell::Nes => &NES,
        NesShell::Famicom => &FAMICOM,
    };
    lock.get_or_init(|| {
        let (w, h) = face_box(shell);
        depth_map(mask(shell), w as usize, h as usize)
    })
}

fn detail(shell: NesShell) -> &'static Detail {
    static NES: OnceLock<Detail> = OnceLock::new();
    static FAMICOM: OnceLock<Detail> = OnceLock::new();
    let (lock, svg) = match shell {
        NesShell::Nes => (&NES, NES_DETAIL_SVG),
        NesShell::Famicom => (&FAMICOM, FAMICOM_DETAIL_SVG),
    };
    lock.get_or_init(|| {
        let (w, h) = face_box(shell);
        rasterise_detail(svg, w, h).unwrap_or_else(|| Detail::blank(w, h))
    })
}

pub(crate) fn spec(shell: NesShell) -> Spec {
    let (w, h) = face_box(shell);
    let (label, label_round) = match shell {
        NesShell::Nes => (NES_LABEL, 0),
        NesShell::Famicom => (FAMICOM_LABEL, 7),
    };
    Spec {
        w,
        h,
        label,
        label_round,
        mask: mask(shell),
        depth: depth(shell),
        detail: detail(shell),
        max_h: (label.3 - 2 * PAD) as f32,
        rim: RIM,
        board: Board {
            x: (0.1, 0.9),
            top: 0.08,
            pins: (0.25, 0.75),
            contacts_from: 0.88,
            traces_from: 0.78,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use omega_store::{Cart, Platform};
    use std::path::PathBuf;

    fn cart(stem: &str) -> Cart {
        Cart {
            platform: Platform::Nes,
            stem: stem.to_string(),
            rom: PathBuf::from(format!("{stem}.nes")),
            label: None,
            title: String::new(),
            code: String::new(),
            shell: None,
        }
    }

    #[test]
    fn usa_uses_nes_and_europe_and_japan_use_famicom() {
        assert_eq!(shell_of(&cart("Contra (USA)")), NesShell::Nes);
        assert_eq!(shell_of(&cart("Mario Bros. (Europe)")), NesShell::Famicom);
        assert_eq!(shell_of(&cart("Makaimura (Japan)")), NesShell::Famicom);
        assert_eq!(shell_of(&cart("Homebrew")), NesShell::Famicom);
    }
}
