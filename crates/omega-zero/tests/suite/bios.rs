use crate::common;

use std::path::PathBuf;

use common::core_lock;
use omega_retro::{ButtonMask, LibretroCore};
use omega_store::Core;
use omega_zero::core::open_core_for;
use tempfile::tempdir;

#[test]
fn famicom_disk_system_requires_an_8k_disksys_rom() {
    let d = tempdir().unwrap();
    std::fs::create_dir(d.path().join("BIOS")).unwrap();
    assert!(!omega_zero::root::has_fds_bios(d.path()));
    std::fs::write(d.path().join("BIOS/disksys.rom"), vec![0; 8191]).unwrap();
    assert!(!omega_zero::root::has_fds_bios(d.path()));
    std::fs::write(d.path().join("BIOS/disksys.rom"), vec![0; 8192]).unwrap();
    assert!(omega_zero::root::has_fds_bios(d.path()));
}

#[test]
fn sufami_turbo_requires_a_256k_stbios() {
    let d = tempdir().unwrap();
    std::fs::create_dir(d.path().join("BIOS")).unwrap();
    assert!(!omega_zero::root::has_sufami_turbo_bios(d.path()));
    std::fs::write(d.path().join("BIOS/STBIOS.bin"), vec![0; 262_143]).unwrap();
    assert!(!omega_zero::root::has_sufami_turbo_bios(d.path()));
    std::fs::write(d.path().join("BIOS/STBIOS.bin"), vec![0; 262_144]).unwrap();
    assert!(omega_zero::root::has_sufami_turbo_bios(d.path()));
}

#[test]
fn atari_5200_requires_a_2k_5200_rom() {
    let d = tempdir().unwrap();
    std::fs::create_dir(d.path().join("BIOS")).unwrap();
    std::fs::write(d.path().join("BIOS/5200.rom"), vec![0; 2047]).unwrap();
    assert!(!omega_zero::root::has_atari5200_bios(d.path()));
    std::fs::write(d.path().join("BIOS/5200.rom"), vec![0; 2048]).unwrap();
    assert!(omega_zero::root::has_atari5200_bios(d.path()));
}

#[test]
fn sega_cd_bios_is_found_in_the_shared_bios_directory() {
    let d = tempdir().unwrap();
    std::fs::create_dir(d.path().join("BIOS")).unwrap();
    assert!(!omega_zero::root::has_segacd_bios(d.path()));

    std::fs::write(d.path().join("BIOS/bios_CD_U.bin"), b"bios").unwrap();
    assert!(omega_zero::root::has_segacd_bios(d.path()));
}

#[test]
fn cd32_accepts_a_combined_or_split_kickstart_set() {
    let d = tempdir().unwrap();
    std::fs::create_dir(d.path().join("BIOS")).unwrap();
    let kick = d.path().join("BIOS/kick40060.CD32");
    let ext = d.path().join("BIOS/kick40060.CD32.ext");
    assert!(!omega_zero::root::has_cd32_bios(d.path()));

    std::fs::write(&kick, vec![0; 512 * 1024]).unwrap();
    assert!(!omega_zero::root::has_cd32_bios(d.path()));
    std::fs::write(&ext, vec![0; 512 * 1024]).unwrap();
    assert!(omega_zero::root::has_cd32_bios(d.path()));

    std::fs::remove_file(&ext).unwrap();
    std::fs::write(&kick, vec![0; 1024 * 1024]).unwrap();
    assert!(omega_zero::root::has_cd32_bios(d.path()));
}

#[test]
fn pc_engine_cd_bios_is_found_in_the_shared_bios_directory() {
    let d = tempdir().unwrap();
    std::fs::create_dir(d.path().join("BIOS")).unwrap();
    assert!(!omega_zero::root::has_pce_cd_bios(d.path()));

    std::fs::write(d.path().join("BIOS/syscard3.pce"), b"bios").unwrap();
    assert!(omega_zero::root::has_pce_cd_bios(d.path()));
}

#[test]
fn fbneo_finds_neogeo_bioses_in_its_recommended_directory() {
    let d = tempdir().unwrap();
    std::fs::create_dir_all(d.path().join("BIOS/fbneo")).unwrap();
    assert!(!omega_zero::root::has_neogeo_bios(d.path()));
    assert!(!omega_zero::root::has_neogeo_cd_bios(d.path()));

    std::fs::write(d.path().join("BIOS/fbneo/neogeo.zip"), b"bios").unwrap();
    assert!(omega_zero::root::has_neogeo_bios(d.path()));
    assert!(!omega_zero::root::has_neogeo_cd_bios(d.path()));

    std::fs::write(d.path().join("BIOS/fbneo/neocdz.zip"), b"bios").unwrap();
    assert!(omega_zero::root::has_neogeo_cd_bios(d.path()));
}

fn vendored_core_paths() -> Vec<PathBuf> {
    common::vendored_core().into_iter().collect()
}

#[test]
fn a_missing_bios_folder_still_boots_a_core() {
    let _g = core_lock();
    let d = common::tmp_root_with_real_carts(&["Emerald"]);
    std::fs::remove_dir_all(d.path().join("BIOS")).ok();
    let mut core = open_core_for(d.path(), Core::Mgba, "auto", false, &vendored_core_paths());
    core.load(&d.path().join("Games/GBA/Emerald.gba")).unwrap();
    core.run_frame(ButtonMask::default());
}

#[test]
fn an_empty_bios_folder_still_boots_a_core() {
    let _g = core_lock();
    let d = common::tmp_root_with_real_carts(&["Emerald"]);
    std::fs::create_dir_all(d.path().join("BIOS")).unwrap();
    let mut core = open_core_for(d.path(), Core::Mgba, "auto", false, &vendored_core_paths());
    core.load(&d.path().join("Games/GBA/Emerald.gba")).unwrap();
    core.run_frame(ButtonMask::default());
}

#[test]
fn the_core_is_told_the_bios_folder_not_the_dylib_folder() {
    let _g = core_lock();
    let d = tempdir().unwrap();
    let bios = d.path().join("BIOS");
    let saves = d.path().join("Saves");
    std::fs::create_dir_all(&bios).unwrap();
    std::fs::create_dir_all(&saves).unwrap();
    let Some(dylib) = common::vendored_core() else {
        return;
    };
    let core = LibretroCore::open_with(&dylib, &bios, &saves).unwrap();
    assert_eq!(core.reported_system_dir(), bios.to_string_lossy());
    assert_ne!(
        core.reported_system_dir(),
        dylib.parent().unwrap().to_string_lossy()
    );
}

#[test]
fn the_core_is_told_the_saves_folder_too() {
    let _g = core_lock();
    let d = tempdir().unwrap();
    let bios = d.path().join("BIOS");
    let saves = d.path().join("Saves");
    std::fs::create_dir_all(&bios).unwrap();
    std::fs::create_dir_all(&saves).unwrap();
    let Some(dylib) = common::vendored_core() else {
        return;
    };
    let core = LibretroCore::open_with(&dylib, &bios, &saves).unwrap();
    assert_eq!(core.reported_save_dir(), saves.to_string_lossy());
}

#[test]
fn only_a_real_bios_image_turns_the_boot_splash_on() {
    let d = common::tmp_root_with_carts(&["Emerald"]);
    let bios = d.path().join("BIOS").join("gba_bios.bin");

    assert!(
        !omega_zero::root::has_real_bios(d.path()),
        "an empty BIOS folder counted as a BIOS"
    );

    for (bytes, what) in [
        (vec![], "a zero byte file"),
        (vec![0x18u8; 1024], "a truncated image, right first byte"),
        (vec![0x18u8; 16 * 1024 - 1], "one byte short of an image"),
        (vec![0x18u8; 16 * 1024 + 1], "one byte past an image"),
        (vec![0u8; 16 * 1024], "16 KB that starts like nothing"),
        (vec![0xffu8; 16 * 1024], "16 KB of erased flash"),
    ] {
        std::fs::write(&bios, &bytes).unwrap();
        assert!(
            !omega_zero::root::has_real_bios(d.path()),
            "{what} counted as a BIOS: gpSP would fall back to its built-in one and boot \
             through a splash that does not exist"
        );
    }

    let mut real = vec![0u8; 16 * 1024];
    real[0] = 0x18;
    std::fs::write(&bios, &real).unwrap();
    assert!(
        omega_zero::root::has_real_bios(d.path()),
        "a 16 KB image starting 0x18 is what gpSP itself accepts, and it was refused"
    );
}

#[test]
fn a_missing_bios_folder_turns_the_boot_splash_off() {
    let d = common::tmp_root_with_carts(&["Emerald"]);
    std::fs::remove_dir_all(d.path().join("BIOS")).unwrap();
    assert!(
        !omega_zero::root::has_real_bios(d.path()),
        "a missing BIOS folder counted as a BIOS"
    );
}

#[test]
fn boot_creates_every_content_folder() {
    let d = tempdir().unwrap();
    let _ = omega_zero::app::App::boot(d.path());
    for sub in omega_zero::root::DIRS {
        assert!(d.path().join(sub).is_dir(), "{sub} was not created");
    }
}

#[test]
fn the_content_root_has_no_art_directory() {
    assert!(omega_zero::root::DIRS.contains(&"Labels"));
    assert!(
        !omega_zero::root::DIRS.contains(&"Art"),
        "Art survived the rename"
    );
}
