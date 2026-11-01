use crate::cart::{Board, Spec, PAD};
use crate::silhouette::{depth_map, rasterise_detail, rasterise_svg, Detail};
use std::sync::OnceLock;
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum AtariShell {
    Atari2600,
    Atari5200,
    Atari7800,
}
pub const THICK: f32 = 0.17;
pub const SHELF_SCALE: f32 = 0.5;
macro_rules! shell{($spec:ident,$mask:ident,$depth:ident,$detail:ident,$svg:literal,$det:literal,$w:expr,$h:expr,$label:expr)=>{
fn $mask()->&'static[u8]{static V:OnceLock<Vec<u8>>=OnceLock::new();V.get_or_init(||rasterise_svg(include_str!($svg),$w,$h).unwrap_or_else(||vec![255;($w*$h)as usize]))}
fn $depth()->&'static[u8]{static V:OnceLock<Vec<u8>>=OnceLock::new();V.get_or_init(||depth_map($mask(),$w as usize,$h as usize))}
fn $detail()->&'static Detail{static V:OnceLock<Detail>=OnceLock::new();V.get_or_init(||rasterise_detail(include_str!($det),$w,$h).unwrap_or_else(||Detail::blank($w,$h)))}
fn $spec()->Spec{let l:(u32,u32,u32,u32)=$label;Spec{w:$w,h:$h,label:l,label_round:8,mask:$mask(),depth:$depth(),detail:$detail(),max_h:(l.3-2*PAD)as f32,rim:5,board:Board{x:(0.1,0.9),top:0.1,pins:(0.2,0.8),contacts_from:0.9,traces_from:0.8}}}
}}
shell!(
    s2600,
    m2600,
    d2600,
    t2600,
    "../../assets/atari_2600_cart.svg",
    "../../assets/atari_2600_cart_detail.svg",
    430,
    520,
    (31, 30, 368, 452)
);
shell!(
    s5200,
    m5200,
    d5200,
    t5200,
    "../../assets/atari_5200_cart.svg",
    "../../assets/atari_5200_cart_detail.svg",
    560,
    500,
    (54, 45, 452, 340)
);
shell!(
    s7800,
    m7800,
    d7800,
    t7800,
    "../../assets/atari_7800_cart.svg",
    "../../assets/atari_7800_cart_detail.svg",
    450,
    500,
    (35, 27, 380, 438)
);
pub(crate) fn spec(s: AtariShell) -> Spec {
    match s {
        AtariShell::Atari2600 => s2600(),
        AtariShell::Atari5200 => s5200(),
        AtariShell::Atari7800 => s7800(),
    }
}
pub(crate) fn outline(s: AtariShell) -> (Vec<(f32, f32)>, (f32, f32)) {
    let (w, h) = match s {
        AtariShell::Atari2600 => (430., 520.),
        AtariShell::Atari5200 => (560., 500.),
        AtariShell::Atari7800 => (450., 500.),
    };
    (
        vec![
            (w * 0.05, 0.0),
            (w * 0.95, 0.0),
            (w, h * 0.05),
            (w, h * 0.95),
            (w * 0.95, h),
            (w * 0.05, h),
            (0.0, h * 0.95),
            (0.0, h * 0.05),
        ],
        (w, h),
    )
}
