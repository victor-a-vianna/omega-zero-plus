use std::path::{Path, PathBuf};

use omega_store::Platform;

/// Platforms retained in the project but omitted from the H700 product profile because their
/// current cores are unstable or too slow on that SoC.
pub const H700_EXCLUDED: [Platform; 8] = [
    Platform::Cd32,
    Platform::Gp32,
    Platform::Pc9800,
    Platform::Pc8800,
    Platform::PcFx,
    Platform::AtariJaguar,
    Platform::AtariJaguarCd,
    Platform::Daphne,
];

pub fn platform_enabled(platform: Platform) -> bool {
    !cfg!(feature = "h700") || !H700_EXCLUDED.contains(&platform)
}

pub const DIRS: [&str; 54] = [
    "BIOS",
    "Config",
    "Games",
    "Games/GBA",
    "Games/GB",
    "Games/GBC",
    "Games/GENESIS",
    "Games/32X",
    "Games/SEGACD",
    "Games/NES",
    "Games/SNES",
    "Games/PS1",
    "Labels",
    "Labels/GBA",
    "Labels/GB",
    "Labels/GBC",
    "Labels/GENESIS",
    "Labels/32X",
    "Labels/SEGACD",
    "Labels/NES",
    "Labels/SNES",
    "Labels/PS1",
    "Overlays",
    "Overlays/GBA",
    "Overlays/GB",
    "Overlays/GBC",
    "Overlays/GENESIS",
    "Overlays/32X",
    "Overlays/SEGACD",
    "Overlays/NES",
    "Overlays/SNES",
    "Overlays/PS1",
    "Saves",
    "Saves/GBA",
    "Saves/GB",
    "Saves/GBC",
    "Saves/GENESIS",
    "Saves/32X",
    "Saves/SEGACD",
    "Saves/NES",
    "Saves/SNES",
    "Saves/PS1",
    "States",
    "States/GBA",
    "States/GB",
    "States/GBC",
    "States/GENESIS",
    "States/32X",
    "States/SEGACD",
    "States/NES",
    "States/SNES",
    "States/PS1",
    "System",
    "Wallpapers",
];

pub fn ensure(root: &Path) {
    for sub in DIRS {
        let _ = std::fs::create_dir_all(root.join(sub));
    }
    for platform in omega_store::Platform::ALL
        .into_iter()
        .filter(|platform| platform_enabled(*platform))
    {
        for parent in ["Games", "Labels", "Overlays", "Saves", "States"] {
            let _ = std::fs::create_dir_all(root.join(parent).join(platform.dir_name()));
        }
    }
    let _ = std::fs::create_dir_all(root.join("BIOS/fbneo"));
    if platform_enabled(Platform::Pc9800) {
        let _ = std::fs::create_dir_all(root.join("BIOS/np2kai"));
    }
    if platform_enabled(Platform::Pc8800) {
        let _ = std::fs::create_dir_all(root.join("BIOS/quasi88"));
    }
    if let Err(e) = omega_store::move_config(root) {
        eprintln!("omega-zero: config: {e}");
    }
}

pub fn bios_dir(root: &Path) -> PathBuf {
    root.join("BIOS")
}

pub const FDS_BIOS_FILE: &str = "disksys.rom";

pub const ATARI_ST_BIOS_FILE: &str = "tos.img";

pub fn has_atari_st_bios(root: &Path) -> bool {
    bios_dir(root).join(ATARI_ST_BIOS_FILE).is_file()
}

pub fn has_np2kai_bios(root: &Path) -> bool {
    let dir = bios_dir(root).join("np2kai");
    dir.join("bios.rom").is_file()
        && (dir.join("font.rom").is_file() || dir.join("font.bmp").is_file())
}

pub fn has_quasi88_bios(root: &Path) -> bool {
    let dir = bios_dir(root).join("quasi88");
    dir.join("N88.ROM").is_file()
        && (dir.join("N88SUB.ROM").is_file() || dir.join("DISK.ROM").is_file())
}

pub fn has_fds_bios(root: &Path) -> bool {
    bios_dir(root)
        .join(FDS_BIOS_FILE)
        .metadata()
        .is_ok_and(|metadata| metadata.is_file() && metadata.len() == 8192)
}

pub const SUFAMI_TURBO_BIOS_FILE: &str = "STBIOS.bin";

pub fn has_sufami_turbo_bios(root: &Path) -> bool {
    bios_dir(root)
        .join(SUFAMI_TURBO_BIOS_FILE)
        .metadata()
        .is_ok_and(|metadata| metadata.is_file() && metadata.len() == 262_144)
}

pub const ATARI5200_BIOS_FILE: &str = "5200.rom";
pub fn has_atari5200_bios(root: &Path) -> bool {
    bios_dir(root)
        .join(ATARI5200_BIOS_FILE)
        .metadata()
        .is_ok_and(|metadata| metadata.is_file() && metadata.len() == 2048)
}

/// BIOS filenames PicoDrive recognises for the three Sega CD regions.
pub const SEGACD_BIOS_FILES: [&str; 12] = [
    "bios_CD_U.bin",
    "us_scd2_9306.bin",
    "SegaCDBIOS9303.bin",
    "us_scd1_9210.bin",
    "bios_CD_E.bin",
    "eu_mcd2_9306.bin",
    "eu_mcd2_9303.bin",
    "eu_mcd1_9210.bin",
    "bios_CD_J.bin",
    "jp_mcd2_921222.bin",
    "jp_mcd1_9112.bin",
    "jp_mcd1_9111.bin",
];

pub fn has_segacd_bios(root: &Path) -> bool {
    SEGACD_BIOS_FILES
        .iter()
        .any(|name| bios_dir(root).join(name).is_file())
}

pub const PCE_CD_BIOS_FILE: &str = "syscard3.pce";
pub const PCFX_BIOS_FILE: &str = "pcfx.rom";

pub fn has_pce_cd_bios(root: &Path) -> bool {
    bios_dir(root).join(PCE_CD_BIOS_FILE).is_file()
}

pub fn has_pcfx_bios(root: &Path) -> bool {
    bios_dir(root)
        .join(PCFX_BIOS_FILE)
        .metadata()
        .is_ok_and(|metadata| metadata.is_file() && metadata.len() == 1_048_576)
}

pub fn has_cd32_bios(root: &Path) -> bool {
    let bios = bios_dir(root);
    let combined = bios.join("kick40060.CD32");
    combined.metadata().is_ok_and(|m| m.len() == 1_048_576)
        || (combined.metadata().is_ok_and(|m| m.len() == 524_288)
            && bios
                .join("kick40060.CD32.ext")
                .metadata()
                .is_ok_and(|m| m.len() == 524_288))
}

pub fn has_neogeo_bios(root: &Path) -> bool {
    bios_dir(root).join("fbneo/neogeo.zip").is_file() || bios_dir(root).join("neogeo.zip").is_file()
}

pub fn has_neogeo_cd_bios(root: &Path) -> bool {
    has_neogeo_bios(root)
        && (bios_dir(root).join("fbneo/neocdz.zip").is_file()
            || bios_dir(root).join("neocdz.zip").is_file())
}

const BIOS_FILE: &str = "gba_bios.bin";

const BIOS_BYTES: u64 = 16 * 1024;
const BIOS_FIRST_BYTE: u8 = 0x18;

pub fn has_real_bios(root: &Path) -> bool {
    let Ok(mut f) = std::fs::File::open(bios_dir(root).join(BIOS_FILE)) else {
        return false;
    };
    if !f.metadata().is_ok_and(|m| m.len() == BIOS_BYTES) {
        return false;
    }
    let mut first = [0u8; 1];
    std::io::Read::read_exact(&mut f, &mut first).is_ok() && first[0] == BIOS_FIRST_BYTE
}

pub fn saves_dir(root: &Path) -> PathBuf {
    root.join("Saves")
}
