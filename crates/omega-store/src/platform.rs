use std::path::Path;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Platform {
    #[default]
    Gba,
    Gb,
    Gbc,
    GameGear,
    MasterSystem,
    Sg1000,
    Genesis,
    Sega32X,
    SegaCd,
    Nes,
    FamicomDiskSystem,
    Atari2600,
    Atari800,
    Atari5200,
    Atari7800,
    AtariLynx,
    VirtualBoy,
    PokemonMini,
    GameAndWatch,
    Arduboy,
    WonderSwan,
    WonderSwanColor,
    MegaDuck,
    Supervision,
    Tic80,
    Pico8,
    Chip8,
    Java,
    Vircon32,
    Uzem,
    Daphne,
    NeoGeoPocket,
    NeoGeoPocketColor,
    AtariJaguar,
    AtariJaguarCd,
    AtariSt,
    Pc8800,
    Pc9800,
    Tg16Pce,
    Tg16CdPceCd,
    SuperGrafx,
    PcFx,
    NeoGeo,
    NeoGeoCd,
    Cps1,
    Cps2,
    Cps3,
    FbNeo,
    Mame,
    Gp32,
    Vectrex,
    Cd32,
    SufamiTurbo,
    Snes,
    Ps1,
}

impl Platform {
    pub const ALL: [Platform; 55] = [
        Platform::Gba,
        Platform::Gb,
        Platform::Gbc,
        Platform::GameGear,
        Platform::MasterSystem,
        Platform::Sg1000,
        Platform::Genesis,
        Platform::Sega32X,
        Platform::SegaCd,
        Platform::Nes,
        Platform::FamicomDiskSystem,
        Platform::Atari2600,
        Platform::Atari800,
        Platform::Atari5200,
        Platform::Atari7800,
        Platform::AtariLynx,
        Platform::VirtualBoy,
        Platform::PokemonMini,
        Platform::GameAndWatch,
        Platform::Arduboy,
        Platform::WonderSwan,
        Platform::WonderSwanColor,
        Platform::MegaDuck,
        Platform::Supervision,
        Platform::Tic80,
        Platform::Pico8,
        Platform::Chip8,
        Platform::Java,
        Platform::Vircon32,
        Platform::Uzem,
        Platform::Daphne,
        Platform::NeoGeoPocket,
        Platform::NeoGeoPocketColor,
        Platform::AtariJaguar,
        Platform::AtariJaguarCd,
        Platform::AtariSt,
        Platform::Pc8800,
        Platform::Pc9800,
        Platform::Tg16Pce,
        Platform::Tg16CdPceCd,
        Platform::SuperGrafx,
        Platform::PcFx,
        Platform::NeoGeo,
        Platform::NeoGeoCd,
        Platform::Cps1,
        Platform::Cps2,
        Platform::Cps3,
        Platform::FbNeo,
        Platform::Mame,
        Platform::Gp32,
        Platform::Vectrex,
        Platform::Cd32,
        Platform::SufamiTurbo,
        Platform::Snes,
        Platform::Ps1,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Platform::Gba => "Game Boy Advance",
            Platform::Gb => "Game Boy",
            Platform::Gbc => "Game Boy Color",
            Platform::GameGear => "Sega Game Gear",
            Platform::MasterSystem => "SMS",
            Platform::Sg1000 => "Sega SG-1000",
            Platform::Genesis => "Sega Genesis",
            Platform::Sega32X => "Sega 32X",
            Platform::SegaCd => "Sega CD",
            Platform::Nes => "Nintendo Entertainment System",
            Platform::FamicomDiskSystem => "Famicom Disk System",
            Platform::Atari2600 => "Atari 2600",
            Platform::Atari800 => "Atari 800",
            Platform::Atari5200 => "Atari 5200",
            Platform::Atari7800 => "Atari 7800",
            Platform::AtariLynx => "Atari Lynx",
            Platform::VirtualBoy => "Virtual Boy",
            Platform::PokemonMini => "Pokémon Mini",
            Platform::GameAndWatch => "Game & Watch",
            Platform::Arduboy => "Arduboy",
            Platform::WonderSwan => "WS",
            Platform::WonderSwanColor => "WSC",
            Platform::MegaDuck => "Mega Duck",
            Platform::Supervision => "Watara Supervision",
            Platform::Tic80 => "TIC-80",
            Platform::Pico8 => "PICO-8",
            Platform::Chip8 => "CHIP-8",
            Platform::Java => "Java ME",
            Platform::Vircon32 => "Vircon32",
            Platform::Uzem => "Uzebox",
            Platform::Daphne => "Daphne",
            Platform::NeoGeoPocket => "NGP",
            Platform::NeoGeoPocketColor => "NGPC",
            Platform::AtariJaguar => "Atari Jaguar",
            Platform::AtariJaguarCd => "Atari Jaguar CD",
            Platform::AtariSt => "Atari ST",
            Platform::Pc8800 => "NEC PC-8800",
            Platform::Pc9800 => "NEC PC-9800",
            Platform::Tg16Pce => "TurboGrafx-16 / PC Engine",
            Platform::Tg16CdPceCd => "TurboGrafx-CD / PC Engine CD",
            Platform::SuperGrafx => "PC Engine SuperGrafx",
            Platform::PcFx => "NEC PC-FX",
            Platform::NeoGeo => "Neo Geo AES",
            Platform::NeoGeoCd => "Neo Geo CD",
            Platform::Cps1 => "CPS-1",
            Platform::Cps2 => "CPS-2",
            Platform::Cps3 => "CPS-3",
            Platform::FbNeo => "FBNeo",
            Platform::Mame => "MAME",
            Platform::Gp32 => "GP32",
            Platform::Vectrex => "Vectrex",
            Platform::Cd32 => "CD32",
            Platform::SufamiTurbo => "SuFami Turbo",
            Platform::Snes => "Super Nintendo",
            Platform::Ps1 => "PlayStation",
        }
    }

    pub fn dir_name(self) -> &'static str {
        match self {
            Platform::Gba => "GBA",
            Platform::Gb => "GB",
            Platform::Gbc => "GBC",
            Platform::GameGear => "GAMEGEAR",
            Platform::MasterSystem => "MASTERSYSTEM",
            Platform::Sg1000 => "SG1000",
            Platform::Genesis => "GENESIS",
            Platform::Sega32X => "32X",
            Platform::SegaCd => "SEGACD",
            Platform::Nes => "NES",
            Platform::FamicomDiskSystem => "FDS",
            Platform::Atari2600 => "ATARI2600",
            Platform::Atari800 => "ATARI800",
            Platform::Atari5200 => "ATARI5200",
            Platform::Atari7800 => "ATARI7800",
            Platform::AtariLynx => "LYNX",
            Platform::VirtualBoy => "VIRTUALBOY",
            Platform::PokemonMini => "POKEMONMINI",
            Platform::GameAndWatch => "GAMEANDWATCH",
            Platform::Arduboy => "ARDUBOY",
            Platform::WonderSwan => "WS",
            Platform::WonderSwanColor => "WSC",
            Platform::MegaDuck => "MEGADUCK",
            Platform::Supervision => "SUPERVISION",
            Platform::Tic80 => "TIC80",
            Platform::Pico8 => "PICO8",
            Platform::Chip8 => "CHIP8",
            Platform::Java => "JAVA",
            Platform::Vircon32 => "VIRCON32",
            Platform::Uzem => "UZEM",
            Platform::Daphne => "DAPHNE",
            Platform::NeoGeoPocket => "NEOGEOPOCKET",
            Platform::NeoGeoPocketColor => "NEOGEOPOCKETCOLOR",
            Platform::AtariJaguar => "JAGUAR",
            Platform::AtariJaguarCd => "JAGUARCD",
            Platform::AtariSt => "ATARIST",
            Platform::Pc8800 => "PC8800",
            Platform::Pc9800 => "PC9800",
            Platform::Tg16Pce => "TG16-PCE",
            Platform::Tg16CdPceCd => "TG16CD-PCECD",
            Platform::SuperGrafx => "SUPERGRAFX",
            Platform::PcFx => "PCFX",
            Platform::NeoGeo => "NEOGEO",
            Platform::NeoGeoCd => "NEOGEOCD",
            Platform::Cps1 => "CPS1",
            Platform::Cps2 => "CPS2",
            Platform::Cps3 => "CPS3",
            Platform::FbNeo => "FBNEO",
            Platform::Mame => "MAME",
            Platform::Gp32 => "GP32",
            Platform::Vectrex => "VECTREX",
            Platform::Cd32 => "CD32",
            Platform::SufamiTurbo => "SUFAMITURBO",
            Platform::Snes => "SNES",
            Platform::Ps1 => "PS1",
        }
    }

    /// A directory used by an older build, read during the uppercase Neo Geo migration.
    pub fn legacy_dir_name(self) -> Option<&'static str> {
        match self {
            Platform::NeoGeo => Some("neogeo"),
            Platform::NeoGeoCd => Some("neogeocd"),
            _ => None,
        }
    }

    /// Compact system code used by the shelf pill, which has room for only a short label.
    pub fn pill_name(self) -> &'static str {
        match self {
            Platform::MasterSystem => "SMS",
            Platform::NeoGeoPocket => "NGP",
            Platform::NeoGeoPocketColor => "NGPC",
            _ => self.dir_name(),
        }
    }

    pub fn extensions(self) -> &'static [&'static str] {
        match self {
            Platform::Gba => &["gba"],
            Platform::Gb | Platform::Gbc => &["gb", "gbc"],
            Platform::GameGear => &["gg"],
            Platform::MasterSystem => &["sms"],
            Platform::Sg1000 => &["sg", "sc", "sgd"],
            Platform::Genesis => &["md", "gen", "smd", "bin"],
            Platform::Sega32X => &["32x", "bin"],
            Platform::SegaCd => &["cue", "chd", "m3u"],
            Platform::Nes => &["nes", "unf", "unif"],
            Platform::FamicomDiskSystem => &["fds"],
            Platform::Atari2600 => &["a26", "bin"],
            Platform::Atari800 => &[
                "xfd", "atr", "dcm", "cas", "bin", "zip", "atx", "car", "rom", "com", "xex", "m3u",
            ],
            Platform::Atari5200 => &["a52", "bin"],
            Platform::Atari7800 => &["a78", "bin", "cdf"],
            Platform::AtariLynx => &["lnx", "lyx", "o"],
            Platform::VirtualBoy => &["vb", "vboy", "bin"],
            Platform::PokemonMini => &["min"],
            Platform::GameAndWatch => &["mgw"],
            Platform::Arduboy => &["hex"],
            Platform::WonderSwan => &["ws"],
            Platform::WonderSwanColor => &["wsc"],
            Platform::MegaDuck => &["bin"],
            Platform::Supervision => &["sv", "bin"],
            Platform::Tic80 => &["tic"],
            Platform::Pico8 => &["p8", "png"],
            Platform::Chip8 => &["ch8", "c8", "sc8", "xo8"],
            Platform::Java => &[
                "jar", "sqc", "jam", "jad", "kjx", "adf", "sec", "sto", "sp", "sp0", "sp1", "sp2",
                "sp3", "sp4", "sp5", "sp6", "sp7", "sp8", "sp9",
            ],
            Platform::Vircon32 => &["v32"],
            Platform::Uzem => &["uze"],
            Platform::Daphne => &["daphne"],
            Platform::NeoGeoPocket => &["ngp"],
            Platform::NeoGeoPocketColor => &["ngc", "ngpc", "npc"],
            Platform::AtariJaguar => &["j64", "jag", "rom", "abs", "cof", "bin", "prg"],
            Platform::AtariJaguarCd => &["cue", "cdi", "chd"],
            Platform::AtariSt => &[
                "st", "msa", "zip", "stx", "dim", "ipf", "vhd", "gem", "ide", "m3u",
            ],
            Platform::Pc8800 => &["d88", "m3u", "t88", "cmt"],
            Platform::Pc9800 => &[
                "d88", "88d", "d98", "98d", "fdi", "xdf", "hdm", "dup", "2hd", "tfd", "nfd", "hd4",
                "hd5", "hd9", "fdd", "h01", "hdb", "ddb", "dd6", "dcp", "dcu", "flp", "img", "ima",
                "bin", "fim", "thd", "nhd", "hdi", "vhd", "slh", "hdn", "cmd", "m3u",
            ],
            Platform::Tg16Pce => &["pce"],
            Platform::Tg16CdPceCd => &["cue", "ccd", "chd", "toc", "m3u"],
            Platform::SuperGrafx => &["sgx", "pce"],
            Platform::PcFx => &["cue", "ccd", "toc", "chd"],
            Platform::NeoGeo => &["zip", "7z"],
            Platform::NeoGeoCd => &["cue", "ccd", "chd", "m3u"],
            Platform::Cps1 | Platform::Cps2 | Platform::Cps3 | Platform::FbNeo => &["zip", "7z"],
            Platform::Mame => &["zip"],
            Platform::Gp32 => &["smc", "fxe", "fpk", "zip", "7z"],
            Platform::Vectrex => &["bin", "vec"],
            Platform::Cd32 => &["cue", "ccd", "nrg", "mds", "iso", "chd", "m3u"],
            Platform::SufamiTurbo => &["st", "fig", "smc", "sfc"],
            Platform::Snes => &["sfc", "smc"],
            Platform::Ps1 => &["cue", "chd", "pbp", "m3u"],
        }
    }

    pub fn picture(self) -> (u32, u32) {
        match self {
            Platform::Gba => (240, 160),
            Platform::Gb | Platform::Gbc => (160, 144),
            Platform::GameGear => (160, 144),
            Platform::MasterSystem | Platform::Sg1000 => (256, 192),
            Platform::Genesis | Platform::Sega32X | Platform::SegaCd => (320, 224),
            Platform::Nes
            | Platform::FamicomDiskSystem
            | Platform::SufamiTurbo
            | Platform::Snes => (256, 224),
            Platform::Atari2600 | Platform::Atari7800 => (320, 240),
            Platform::Atari800 | Platform::Atari5200 => (336, 240),
            Platform::AtariLynx => (160, 102),
            Platform::VirtualBoy => (384, 224),
            Platform::PokemonMini => (384, 256),
            Platform::GameAndWatch => (320, 240),
            Platform::Arduboy => (128, 64),
            Platform::WonderSwan | Platform::WonderSwanColor => (224, 144),
            Platform::MegaDuck => (160, 144),
            Platform::Supervision => (160, 160),
            Platform::Tic80 => (240, 136),
            Platform::Pico8 => (128, 128),
            Platform::Chip8 => (128, 64),
            Platform::Java => (240, 320),
            Platform::Vircon32 => (640, 360),
            Platform::Uzem => (240, 224),
            Platform::Daphne => (640, 480),
            Platform::NeoGeoPocket | Platform::NeoGeoPocketColor => (160, 152),
            Platform::AtariJaguar | Platform::AtariJaguarCd => (320, 240),
            Platform::AtariSt => (640, 400),
            Platform::Pc8800 | Platform::Pc9800 => (640, 400),
            Platform::Tg16Pce | Platform::Tg16CdPceCd | Platform::SuperGrafx => (320, 240),
            Platform::PcFx => (320, 240),
            Platform::NeoGeo | Platform::NeoGeoCd => (320, 224),
            Platform::Cps1 | Platform::Cps2 | Platform::Cps3 => (384, 224),
            Platform::FbNeo => (320, 240),
            Platform::Mame => (320, 240),
            Platform::Gp32 => (320, 240),
            Platform::Vectrex => (330, 410),
            Platform::Cd32 => (320, 256),
            Platform::Ps1 => (320, 240),
        }
    }

    /// Whether the frontend keeps L1/R1 while a game of this system plays, for its picture
    /// size: only the Game Boys, whose games have no shoulder buttons.
    pub fn owns_shoulders(self) -> bool {
        matches!(self, Platform::Gb | Platform::Gbc | Platform::GameGear)
    }

    /// Whether held X/Y are turbo A/B: on the handhelds, whose games have no X or Y.
    pub fn turbo_xy(self) -> bool {
        matches!(
            self,
            Platform::Gba | Platform::Gb | Platform::Gbc | Platform::GameGear
        )
    }

    /// Whether the core's colour correction for a handheld's screen applies.
    pub fn colour_correction(self) -> bool {
        matches!(self, Platform::Gba | Platform::Gb | Platform::Gbc)
    }

    /// Whether held L2 rewinds: not on the PlayStation, whose games use L2 and whose states
    /// are too big for the rewind buffer.
    pub fn rewinds(self) -> bool {
        self != Platform::Ps1
    }

    /// Whether L2/R2 belong to the game rather than to rewind and fast-forward.
    pub fn game_owns_l2_r2(self) -> bool {
        self == Platform::Ps1
    }

    pub fn accepts(self, path: &Path) -> bool {
        path.extension()
            .and_then(|e| e.to_str())
            .is_some_and(|ext| {
                self.extensions()
                    .iter()
                    .any(|k| ext.eq_ignore_ascii_case(k))
            })
    }
}
