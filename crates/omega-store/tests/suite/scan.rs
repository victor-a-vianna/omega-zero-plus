use crate::common;

use common::tmp_root;
use omega_store::{scan, Platform};
use tempfile::TempDir;

fn write_rom(d: &TempDir, rel: &str, title: &str) {
    let mut rom = vec![0u8; 0x100];
    rom[0xa0..0xa0 + title.len()].copy_from_slice(title.as_bytes());
    std::fs::write(d.path().join("Games").join(rel), rom).expect("write rom");
}

fn write_png(d: &TempDir, rel: &str) {
    let path = d.path().join("Labels").join(rel);
    std::fs::create_dir_all(path.parent().expect("label has a parent")).expect("create dir");
    std::fs::write(path, b"\x89PNG\r\n\x1a\n").expect("write png");
}

#[test]
fn a_png_in_labels_is_paired_to_its_rom_by_stem() {
    let d = tmp_root();
    write_rom(&d, "GBA/Pokemon Emerald.gba", "POKEMON EMER");
    write_png(&d, "GBA/Pokemon Emerald.png");
    write_rom(&d, "GBA/Advance Wars.gba", "ADVANCEWARS");
    let carts = scan(d.path()).unwrap();
    assert_eq!(carts.len(), 2);
    assert_eq!(carts[0].stem, "Advance Wars");
    assert!(carts[0].label.is_none());
    assert!(
        carts[1].label.is_some(),
        "a label in Labels/ was not picked up"
    );
    assert_eq!(carts[1].title, "POKEMON EMER");
}

#[test]
fn scan_ignores_non_gba_files() {
    let d = tmp_root();
    write_rom(&d, "GBA/Real.gba", "REAL");
    std::fs::write(d.path().join("Games/GBA/notes.txt"), "hi").unwrap();
    assert_eq!(scan(d.path()).unwrap().len(), 1);
}

#[test]
fn a_pico8_png_cart_drops_both_extensions_from_its_name() {
    let d = tmp_root();
    std::fs::write(d.path().join("Games/PICO8/Celeste.p8.png"), b"png cart").unwrap();

    let cart = scan(d.path())
        .unwrap()
        .into_iter()
        .find(|cart| cart.platform == Platform::Pico8)
        .unwrap();

    assert_eq!(cart.stem, "Celeste");
    assert_eq!(cart.rom, d.path().join("Games/PICO8/Celeste.p8.png"));
}

#[test]
fn daphne_shelves_a_game_directory_only_when_its_framefile_is_present() {
    let d = tmp_root();
    let game = d.path().join("Games/DAPHNE/lair.daphne");
    let incomplete = d.path().join("Games/DAPHNE/ace.daphne");
    std::fs::create_dir_all(&game).unwrap();
    std::fs::create_dir_all(&incomplete).unwrap();
    std::fs::write(game.join("lair.txt"), "0 lair.m2v\n").unwrap();

    let carts = scan(d.path()).unwrap();
    let cart = carts
        .iter()
        .find(|cart| cart.platform == Platform::Daphne)
        .unwrap();
    assert_eq!(cart.stem, "lair");
    assert_eq!(cart.rom, game);
    assert_eq!(
        carts
            .iter()
            .filter(|cart| cart.platform == Platform::Daphne)
            .count(),
        1
    );
}

#[test]
fn an_appledouble_sidecar_is_not_shelved_as_a_cart() {
    let d = tmp_root();
    write_rom(&d, "GBA/Metroid Fusion.gba", "METROID");
    write_rom(&d, "GBA/._Metroid Fusion.gba", "METROID");
    let carts = scan(d.path()).unwrap();
    assert_eq!(carts.len(), 1, "an AppleDouble sidecar reached the shelf");
    assert_eq!(carts[0].stem, "Metroid Fusion");
}

#[test]
fn header_title_of_a_truncated_rom_is_none_not_a_panic() {
    let d = tmp_root();
    std::fs::write(d.path().join("Games/GBA/Tiny.gba"), [0u8; 8]).unwrap();
    assert!(scan(d.path()).unwrap()[0].title.is_empty());
}

#[test]
fn a_header_title_that_is_not_text_is_dropped_rather_than_mangled() {
    let d = tmp_root();
    let mut rom = vec![0u8; 0x100];
    rom[0xa0..0xac].copy_from_slice(&[0xffu8; 12]);
    std::fs::write(d.path().join("Games/GBA/Garbage.gba"), rom).unwrap();
    assert!(scan(d.path()).unwrap()[0].title.is_empty());
}

#[test]
fn a_root_with_no_games_directory_scans_as_empty() {
    let d = tempfile::tempdir().unwrap();
    assert!(scan(d.path()).unwrap().is_empty());
}

#[test]
fn a_cart_takes_the_platform_of_the_folder_it_is_in() {
    let d = tmp_root();
    write_rom(&d, "GBA/Metroid Fusion.gba", "METROID");
    std::fs::write(d.path().join("Games/GB/Tetris.gb"), vec![0u8; 0x150]).unwrap();
    std::fs::write(d.path().join("Games/GBC/Chromatic.gbc"), vec![0u8; 0x150]).unwrap();
    std::fs::write(d.path().join("Games/GENESIS/Sonic.md"), vec![0u8; 0x180]).unwrap();
    std::fs::write(d.path().join("Games/32X/Chaotix.32x"), vec![0u8; 0x180]).unwrap();
    std::fs::write(d.path().join("Games/SEGACD/Sonic CD.chd"), b"chd").unwrap();
    std::fs::write(d.path().join("Games/NES/Super Mario Bros.nes"), b"nes").unwrap();

    let carts = scan(d.path()).unwrap();

    let by_stem = |s: &str| carts.iter().find(|c| c.stem == s).unwrap().platform;
    assert_eq!(by_stem("Metroid Fusion"), Platform::Gba);
    assert_eq!(by_stem("Tetris"), Platform::Gb);
    assert_eq!(by_stem("Chromatic"), Platform::Gbc);
    assert_eq!(by_stem("Sonic"), Platform::Genesis);
    assert_eq!(by_stem("Chaotix"), Platform::Sega32X);
    assert_eq!(by_stem("Sonic CD"), Platform::SegaCd);
    assert_eq!(by_stem("Super Mario Bros"), Platform::Nes);
}

#[test]
fn a_gba_rom_in_the_game_boy_folder_does_not_appear() {
    let d = tmp_root();
    std::fs::write(d.path().join("Games/GB/Wrong.gba"), vec![0u8; 0x150]).unwrap();
    assert!(scan(d.path()).unwrap().is_empty());
}

#[test]
fn sega_cd_discs_share_one_game_and_a_platform_specific_playlist() {
    let d = tmp_root();
    let games = d.path().join("Games/SEGACD");
    std::fs::write(games.join("Lunar (Disc 1).chd"), b"disc one").unwrap();
    std::fs::write(games.join("Lunar (Disc 2).chd"), b"disc two").unwrap();

    let carts = scan(d.path()).unwrap();
    let lunar = carts.iter().find(|cart| cart.stem == "Lunar").unwrap();

    assert_eq!(lunar.platform, Platform::SegaCd);
    assert_eq!(
        lunar.rom,
        d.path().join("Config/playlists/SEGACD/Lunar.m3u")
    );
    assert_eq!(omega_store::ps1::discs(&lunar.rom).len(), 2);
}

#[test]
fn an_unreadable_platform_folder_does_not_take_the_rest_of_the_library_with_it() {
    let d = tmp_root();
    write_rom(&d, "GBA/Metroid Fusion.gba", "METROID");
    std::fs::write(d.path().join("Games/GB/Tetris.gb"), vec![0u8; 0x150]).unwrap();
    std::fs::remove_dir(d.path().join("Games/GBC")).unwrap();
    std::fs::write(d.path().join("Games/GBC"), b"not a directory").unwrap();

    let carts = scan(d.path()).expect("one bad folder must not fail the whole scan");

    assert_eq!(carts.len(), 2, "the readable shelves were lost too");
    assert!(carts.iter().any(|c| c.stem == "Metroid Fusion"));
    assert!(carts.iter().any(|c| c.stem == "Tetris"));
}

#[test]
fn the_same_stem_on_two_platforms_is_two_carts_with_two_labels() {
    let d = tmp_root();
    write_rom(&d, "GBA/Tetris.gba", "TETRIS");
    std::fs::write(d.path().join("Games/GB/Tetris.gb"), vec![0u8; 0x150]).unwrap();
    std::fs::create_dir_all(d.path().join("Labels/GB")).unwrap();
    std::fs::write(d.path().join("Labels/GB/Tetris.png"), b"\x89PNG\r\n\x1a\n").unwrap();

    let carts = scan(d.path()).unwrap();

    assert_eq!(carts.len(), 2);
    let gb = carts.iter().find(|c| c.platform == Platform::Gb).unwrap();
    let gba = carts.iter().find(|c| c.platform == Platform::Gba).unwrap();
    assert!(gb.label.is_some(), "the Game Boy label was not found");
    assert!(
        gba.label.is_none(),
        "the GBA cart borrowed the Game Boy label"
    );
}
