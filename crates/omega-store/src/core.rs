use std::collections::HashMap;
use std::path::Path;

use crate::Platform;

pub const SELECTED_CORE_FILE: &str = "Config/selected_core.ini";

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Core {
    #[default]
    Mgba,
    Gpsp,
    Picodrive,
    Mesen,
    Stella2023,
    Atari800,
    Prosystem,
    Handy,
    BeetleVb,
    PokeMini,
    Gw,
    Arduous,
    BeetleWswan,
    SameDuck,
    Potator,
    Tic80,
    Retro8,
    Jaxe,
    SquirrelJme,
    Vircon32,
    Uzem,
    Hypseus,
    BeetleNgp,
    BigPemu,
    Hatari,
    Quasi88,
    Np2Kai,
    BeetlePceFast,
    BeetleSuperGrafx,
    BeetlePcFx,
    FbNeo,
    Mame2003Plus,
    Gp32Emu,
    Vecx,
    Puae,
    Snes9x,
    Supafaust,
    PcsxRearmed,
}

impl Core {
    pub const ALL: [Core; 38] = [
        Core::Mgba,
        Core::Gpsp,
        Core::Picodrive,
        Core::Mesen,
        Core::Stella2023,
        Core::Atari800,
        Core::Prosystem,
        Core::Handy,
        Core::BeetleVb,
        Core::PokeMini,
        Core::Gw,
        Core::Arduous,
        Core::BeetleWswan,
        Core::SameDuck,
        Core::Potator,
        Core::Tic80,
        Core::Retro8,
        Core::Jaxe,
        Core::SquirrelJme,
        Core::Vircon32,
        Core::Uzem,
        Core::Hypseus,
        Core::BeetleNgp,
        Core::BigPemu,
        Core::Hatari,
        Core::Quasi88,
        Core::Np2Kai,
        Core::BeetlePceFast,
        Core::BeetleSuperGrafx,
        Core::BeetlePcFx,
        Core::FbNeo,
        Core::Mame2003Plus,
        Core::Gp32Emu,
        Core::Vecx,
        Core::Puae,
        Core::Snes9x,
        Core::Supafaust,
        Core::PcsxRearmed,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            Core::Mgba => "mgba",
            Core::Gpsp => "gpsp",
            Core::Picodrive => "picodrive",
            Core::Mesen => "mesen",
            Core::Stella2023 => "stella2023",
            Core::Atari800 => "atari800",
            Core::Prosystem => "prosystem",
            Core::Handy => "handy",
            Core::BeetleVb => "mednafen_vb",
            Core::PokeMini => "pokemini",
            Core::Gw => "gw",
            Core::Arduous => "arduous",
            Core::BeetleWswan => "mednafen_wswan",
            Core::SameDuck => "sameduck",
            Core::Potator => "potator",
            Core::Tic80 => "tic80",
            Core::Retro8 => "retro8",
            Core::Jaxe => "jaxe",
            Core::SquirrelJme => "squirreljme",
            Core::Vircon32 => "vircon32",
            Core::Uzem => "uzem",
            Core::Hypseus => "hypseus",
            Core::BeetleNgp => "mednafen_ngp",
            Core::BigPemu => "bigpemu",
            Core::Hatari => "hatari2014",
            Core::Quasi88 => "quasi88",
            Core::Np2Kai => "np2kai",
            Core::BeetlePceFast => "mednafen_pce_fast",
            Core::BeetleSuperGrafx => "mednafen_supergrafx",
            Core::BeetlePcFx => "mednafen_pcfx",
            Core::FbNeo => "fbneo",
            Core::Mame2003Plus => "mame2003_plus",
            Core::Gp32Emu => "gp32emu",
            Core::Vecx => "vecx",
            Core::Puae => "puae",
            Core::Snes9x => "snes9x",
            Core::Supafaust => "supafaust",
            Core::PcsxRearmed => "pcsx_rearmed",
        }
    }

    /// The libretro name its library is built under, `<name>_libretro.so`.
    pub fn library(&self) -> &'static str {
        match self {
            Core::Supafaust => "mednafen_supafaust",
            other => other.as_str(),
        }
    }

    pub fn index(self) -> usize {
        self as usize
    }

    /// The core's name as its chip in the core picker wears it, in its own case: as long as the
    /// chip has room for, six letters at most, so Supafaust is Faust.
    pub fn text(self) -> &'static str {
        match self {
            Core::Mgba => "mGBA",
            Core::Gpsp => "gpSP",
            Core::Picodrive => "Pico",
            Core::Mesen => "Mesen",
            Core::Stella2023 => "Stel",
            Core::Atari800 => "A800",
            Core::Prosystem => "Pro",
            Core::Handy => "Hndy",
            Core::BeetleVb => "VB",
            Core::PokeMini => "Poke",
            Core::Gw => "GW",
            Core::Arduous => "Ardu",
            Core::BeetleWswan => "WS",
            Core::SameDuck => "Duck",
            Core::Potator => "Pot",
            Core::Tic80 => "TIC",
            Core::Retro8 => "R8",
            Core::Jaxe => "C8",
            Core::SquirrelJme => "SJME",
            Core::Vircon32 => "V32",
            Core::Uzem => "Uzem",
            Core::Hypseus => "DAPH",
            Core::BeetleNgp => "NGP",
            Core::BigPemu => "BigP",
            Core::Hatari => "Hatr",
            Core::Quasi88 => "Q88",
            Core::Np2Kai => "NP2",
            Core::BeetlePceFast => "PCE",
            Core::BeetleSuperGrafx => "SGX",
            Core::BeetlePcFx => "PCFX",
            Core::FbNeo => "FBN",
            Core::Mame2003Plus => "MAME",
            Core::Gp32Emu => "GP32",
            Core::Vecx => "VecX",
            Core::Puae => "PUAE",
            Core::Snes9x => "Snes9x",
            Core::Supafaust => "Faust",
            Core::PcsxRearmed => "PCSX",
        }
    }

    /// The core's whole name, in its own case, as About credits it.
    pub fn name(self) -> &'static str {
        match self {
            Core::Mgba => "mGBA",
            Core::Gpsp => "gpSP",
            Core::Picodrive => "PicoDrive",
            Core::Mesen => "Mesen",
            Core::Stella2023 => "Stella 2023",
            Core::Atari800 => "Atari800",
            Core::Prosystem => "ProSystem",
            Core::Handy => "Handy",
            Core::BeetleVb => "Beetle VB",
            Core::PokeMini => "PokeMini",
            Core::Gw => "Game & Watch",
            Core::Arduous => "Arduous",
            Core::BeetleWswan => "Beetle WonderSwan",
            Core::SameDuck => "SameDuck",
            Core::Potator => "Potator",
            Core::Tic80 => "TIC-80",
            Core::Retro8 => "Retro8",
            Core::Jaxe => "JAXE",
            Core::SquirrelJme => "SquirrelJME",
            Core::Vircon32 => "Vircon32",
            Core::Uzem => "Uzem",
            Core::Hypseus => "Hypseus Singe",
            Core::BeetleNgp => "Beetle NeoPop",
            Core::BigPemu => "BigPEmu",
            Core::Hatari => "Hatari",
            Core::Quasi88 => "QUASI88",
            Core::Np2Kai => "Neko Project II kai",
            Core::BeetlePceFast => "Beetle PCE Fast",
            Core::BeetleSuperGrafx => "Beetle SuperGrafx",
            Core::BeetlePcFx => "Beetle PC-FX",
            Core::FbNeo => "FinalBurn Neo",
            Core::Mame2003Plus => "MAME 2003-Plus",
            Core::Gp32Emu => "GP32emu",
            Core::Vecx => "VecX",
            Core::Puae => "PUAE",
            Core::Snes9x => "Snes9x",
            Core::Supafaust => "Supafaust",
            Core::PcsxRearmed => "PCSX-ReARMed",
        }
    }

    pub fn parse(s: &str) -> Option<Core> {
        match s.trim().to_ascii_lowercase().as_str() {
            "mgba" => Some(Core::Mgba),
            "gpsp" => Some(Core::Gpsp),
            "picodrive" => Some(Core::Picodrive),
            "mesen" => Some(Core::Mesen),
            "stella2023" => Some(Core::Stella2023),
            "atari800" => Some(Core::Atari800),
            "prosystem" => Some(Core::Prosystem),
            "handy" => Some(Core::Handy),
            "mednafen_vb" | "beetle_vb" => Some(Core::BeetleVb),
            "pokemini" => Some(Core::PokeMini),
            "gw" => Some(Core::Gw),
            "arduous" => Some(Core::Arduous),
            "mednafen_wswan" | "beetle_wswan" => Some(Core::BeetleWswan),
            "sameduck" => Some(Core::SameDuck),
            "potator" => Some(Core::Potator),
            "tic80" => Some(Core::Tic80),
            "retro8" => Some(Core::Retro8),
            "jaxe" => Some(Core::Jaxe),
            "squirreljme" | "squirrel_jme" => Some(Core::SquirrelJme),
            "vircon32" => Some(Core::Vircon32),
            "uzem" => Some(Core::Uzem),
            "hypseus" | "daphne" => Some(Core::Hypseus),
            "mednafen_ngp" | "beetle_ngp" => Some(Core::BeetleNgp),
            "bigpemu" | "virtualjaguar" => Some(Core::BigPemu),
            "hatari" | "hatari2014" => Some(Core::Hatari),
            "quasi88" => Some(Core::Quasi88),
            "np2kai" => Some(Core::Np2Kai),
            "mednafen_pce_fast" | "beetle_pce_fast" => Some(Core::BeetlePceFast),
            "mednafen_supergrafx" | "beetle_supergrafx" => Some(Core::BeetleSuperGrafx),
            "mednafen_pcfx" | "beetle_pcfx" => Some(Core::BeetlePcFx),
            "fbneo" => Some(Core::FbNeo),
            "mame2003_plus" | "mame" => Some(Core::Mame2003Plus),
            "gp32emu" | "gp32" => Some(Core::Gp32Emu),
            "vecx" => Some(Core::Vecx),
            "puae" => Some(Core::Puae),
            "snes9x" => Some(Core::Snes9x),
            "supafaust" => Some(Core::Supafaust),
            "pcsx_rearmed" => Some(Core::PcsxRearmed),
            _ => None,
        }
    }

    pub fn runs(self, platform: Platform) -> bool {
        Core::for_platform(platform).contains(&self)
    }

    /// The cores that run `platform`, the default first, in the order the core picker's
    /// sockets show them.
    pub fn for_platform(platform: Platform) -> &'static [Core] {
        match platform {
            Platform::Gba => &[Core::Mgba, Core::Gpsp],
            Platform::Gb | Platform::Gbc => &[Core::Mgba],
            Platform::Genesis | Platform::Sega32X | Platform::SegaCd => &[Core::Picodrive],
            Platform::GameGear | Platform::MasterSystem | Platform::Sg1000 => &[Core::Picodrive],
            Platform::Nes | Platform::FamicomDiskSystem => &[Core::Mesen],
            Platform::Atari2600 => &[Core::Stella2023],
            Platform::Atari800 | Platform::Atari5200 => &[Core::Atari800],
            Platform::Atari7800 => &[Core::Prosystem],
            Platform::AtariLynx => &[Core::Handy],
            Platform::VirtualBoy => &[Core::BeetleVb],
            Platform::PokemonMini => &[Core::PokeMini],
            Platform::GameAndWatch => &[Core::Gw],
            Platform::Arduboy => &[Core::Arduous],
            Platform::WonderSwan | Platform::WonderSwanColor => &[Core::BeetleWswan],
            Platform::MegaDuck => &[Core::SameDuck],
            Platform::Supervision => &[Core::Potator],
            Platform::Tic80 => &[Core::Tic80],
            Platform::Pico8 => &[Core::Retro8],
            Platform::Chip8 => &[Core::Jaxe],
            Platform::Java => &[Core::SquirrelJme],
            Platform::Vircon32 => &[Core::Vircon32],
            Platform::Uzem => &[Core::Uzem],
            Platform::Daphne => &[Core::Hypseus],
            Platform::NeoGeoPocket | Platform::NeoGeoPocketColor => &[Core::BeetleNgp],
            Platform::AtariJaguar | Platform::AtariJaguarCd => &[Core::BigPemu],
            Platform::AtariSt => &[Core::Hatari],
            Platform::Pc8800 => &[Core::Quasi88],
            Platform::Pc9800 => &[Core::Np2Kai],
            Platform::Tg16Pce | Platform::Tg16CdPceCd => &[Core::BeetlePceFast],
            Platform::SuperGrafx => &[Core::BeetleSuperGrafx],
            Platform::PcFx => &[Core::BeetlePcFx],
            Platform::NeoGeo
            | Platform::NeoGeoCd
            | Platform::Cps1
            | Platform::Cps2
            | Platform::Cps3
            | Platform::FbNeo => &[Core::FbNeo],
            Platform::Mame => &[Core::Mame2003Plus],
            Platform::Gp32 => &[Core::Gp32Emu],
            Platform::Vectrex => &[Core::Vecx],
            Platform::Cd32 => &[Core::Puae],
            Platform::SufamiTurbo => &[Core::Snes9x],
            Platform::Snes => &[Core::Snes9x, Core::Supafaust],
            Platform::Ps1 => &[Core::PcsxRearmed],
        }
    }

    pub fn default_for(platform: Platform) -> Core {
        Core::for_platform(platform)[0]
    }
}

pub fn read_selected_cores(root: &Path) -> HashMap<String, Core> {
    crate::ini::read(root, SELECTED_CORE_FILE)
        .into_iter()
        .filter_map(|(stem, name)| Core::parse(&name).map(|core| (stem, core)))
        .collect()
}

pub fn core_for(root: &Path, stem: &str) -> Core {
    crate::ini::value(root, SELECTED_CORE_FILE, stem)
        .as_deref()
        .and_then(Core::parse)
        .unwrap_or_default()
}

pub fn core_for_platform(root: &Path, stem: &str, platform: Platform) -> Core {
    crate::ini::value(root, SELECTED_CORE_FILE, stem)
        .as_deref()
        .and_then(Core::parse)
        .filter(|core| core.runs(platform))
        .unwrap_or_else(|| Core::default_for(platform))
}

pub fn write_selected_core(root: &Path, stem: &str, core: Core) -> std::io::Result<()> {
    crate::ini::write(root, SELECTED_CORE_FILE, stem, core.as_str())
}
