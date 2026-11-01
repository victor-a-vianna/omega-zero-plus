use crate::cart::{Board, Spec, PAD};
use crate::silhouette::{depth_map, rasterise_detail, rasterise_svg, Detail};
use std::sync::OnceLock;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum Sega8Shell {
    GameGear,
    MasterSystem,
    Sg1000,
}
pub const SHELF_SCALE: f32 = 0.58;
pub const THICK: f32 = 0.16;

macro_rules! shell {
 ($spec:ident,$outline:ident,$mask:ident,$depth:ident,$detail:ident,$svg:literal,$det:literal,$w:expr,$h:expr,$label:expr) => {
  fn $mask()->&'static[u8]{static V:OnceLock<Vec<u8>>=OnceLock::new();V.get_or_init(||rasterise_svg(include_str!($svg),$w,$h).unwrap_or_else(||vec![255;($w*$h)as usize]))}
  fn $depth()->&'static[u8]{static V:OnceLock<Vec<u8>>=OnceLock::new();V.get_or_init(||depth_map($mask(),$w as usize,$h as usize))}
  fn $detail()->&'static Detail{static V:OnceLock<Detail>=OnceLock::new();V.get_or_init(||rasterise_detail(include_str!($det),$w,$h).unwrap_or_else(||Detail::blank($w,$h)))}
  fn $spec()->Spec{let l:(u32,u32,u32,u32)=$label;Spec{w:$w,h:$h,label:l,label_round:6,mask:$mask(),depth:$depth(),detail:$detail(),max_h:(l.3-2*PAD)as f32,rim:5,board:Board{x:(0.1,0.9),top:0.1,pins:(0.2,0.8),contacts_from:0.9,traces_from:0.8}}}
  #[allow(dead_code)] fn $outline()->(Vec<(f32,f32)>,(f32,f32)){(vec![($w as f32*0.06,0.0),($w as f32*0.94,0.0),($w as f32,$h as f32*0.06),($w as f32,$h as f32*0.94),($w as f32*0.94,$h as f32),($w as f32*0.06,$h as f32),(0.0,$h as f32*0.94),(0.0,$h as f32*0.06)],($w as f32,$h as f32))}
 }
}
shell!(
    gg_spec,
    gg_outline,
    gg_mask,
    gg_depth,
    gg_detail,
    "../../assets/game_gear_cart.svg",
    "../../assets/game_gear_cart_detail.svg",
    500,
    500,
    (47, 104, 406, 278)
);
shell!(
    sms_spec,
    sms_outline_unused,
    sms_mask,
    sms_depth,
    sms_detail,
    "../../assets/master_system_cart.svg",
    "../../assets/master_system_cart_detail.svg",
    600,
    360,
    (18, 15, 564, 80)
);
fn sms_outline() -> (Vec<(f32, f32)>, (f32, f32)) {
    (
        vec![
            (273.0, 0.0),
            (327.0, 0.0),
            (333.0, 8.0),
            (591.0, 8.0),
            (600.0, 17.0),
            (600.0, 112.0),
            (594.0, 112.0),
            (594.0, 348.0),
            (582.0, 360.0),
            (18.0, 360.0),
            (6.0, 348.0),
            (6.0, 112.0),
            (0.0, 112.0),
            (0.0, 17.0),
            (9.0, 8.0),
            (267.0, 8.0),
        ],
        (600.0, 360.0),
    )
}
shell!(
    sg_spec,
    sg_outline,
    sg_mask,
    sg_depth,
    sg_detail,
    "../../assets/sg1000_cart.svg",
    "../../assets/sg1000_cart_detail.svg",
    420,
    500,
    (55, 17, 310, 279)
);
pub(crate) fn spec(s: Sega8Shell) -> Spec {
    match s {
        Sega8Shell::GameGear => gg_spec(),
        Sega8Shell::MasterSystem => sms_spec(),
        Sega8Shell::Sg1000 => sg_spec(),
    }
}
pub(crate) fn outline(s: Sega8Shell) -> (Vec<(f32, f32)>, (f32, f32)) {
    match s {
        Sega8Shell::GameGear => gg_outline(),
        Sega8Shell::MasterSystem => sms_outline(),
        Sega8Shell::Sg1000 => sg_outline(),
    }
}
