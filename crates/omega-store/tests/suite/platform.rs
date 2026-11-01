use omega_store::Platform;

#[test]
fn every_platform_has_a_directory() {
    let names: Vec<&str> = Platform::ALL.iter().map(|p| p.dir_name()).collect();
    assert_eq!(
        names,
        vec![
            "GBA",
            "GB",
            "GBC",
            "GAMEGEAR",
            "MASTERSYSTEM",
            "SG1000",
            "GENESIS",
            "32X",
            "SEGACD",
            "NES",
            "FDS",
            "ATARI2600",
            "ATARI800",
            "ATARI5200",
            "ATARI7800",
            "LYNX",
            "VIRTUALBOY",
            "POKEMONMINI",
            "GAMEANDWATCH",
            "ARDUBOY",
            "WS",
            "WSC",
            "MEGADUCK",
            "SUPERVISION",
            "TIC80",
            "PICO8",
            "CHIP8",
            "JAVA",
            "VIRCON32",
            "UZEM",
            "DAPHNE",
            "NEOGEOPOCKET",
            "NEOGEOPOCKETCOLOR",
            "JAGUAR",
            "JAGUARCD",
            "ATARIST",
            "PC8800",
            "PC9800",
            "TG16-PCE",
            "TG16CD-PCECD",
            "SUPERGRAFX",
            "PCFX",
            "NEOGEO",
            "NEOGEOCD",
            "CPS1",
            "CPS2",
            "CPS3",
            "FBNEO",
            "MAME",
            "GP32",
            "VECTREX",
            "CD32",
            "SUFAMITURBO",
            "SNES",
            "PS1",
        ]
    );
}

#[test]
fn every_platform_is_a_shelf_of_its_own() {
    assert_eq!(
        Platform::ALL,
        [
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
            Platform::Ps1
        ]
    );
    for (i, p) in Platform::ALL.iter().enumerate() {
        assert!(
            !Platform::ALL[..i].contains(p),
            "{p:?} is named twice in the ring, so two shelves would hold one platform"
        );
    }
}

#[test]
fn pcfx_accepts_supported_disc_images() {
    assert!(Platform::PcFx.accepts(std::path::Path::new("Blue Breaker.chd")));
    assert!(Platform::PcFx.accepts(std::path::Path::new("Team Innocent.cue")));
    assert!(!Platform::PcFx.accepts(std::path::Path::new("track01.bin")));
}

#[test]
fn each_platform_takes_only_its_own_extensions() {
    assert!(Platform::Gba.accepts(std::path::Path::new("Metroid Fusion.gba")));
    assert!(Platform::AtariSt.accepts(std::path::Path::new("Dungeon Master.st")));
    assert!(Platform::Pc8800.accepts(std::path::Path::new("Ys II.d88")));
    assert!(Platform::GameAndWatch.accepts(std::path::Path::new("Donkey Kong.mgw")));
    assert!(Platform::Arduboy.accepts(std::path::Path::new("Arduventure.hex")));
    assert!(Platform::WonderSwan.accepts(std::path::Path::new("Gunpey.ws")));
    assert!(Platform::WonderSwanColor.accepts(std::path::Path::new("Final Fantasy.wsc")));
    assert!(Platform::MegaDuck.accepts(std::path::Path::new("The Brick Wall.bin")));
    assert!(Platform::Supervision.accepts(std::path::Path::new("Crystball.sv")));
    assert!(Platform::Tic80.accepts(std::path::Path::new("Bunnymark.tic")));
    assert!(Platform::Pico8.accepts(std::path::Path::new("Celeste.p8")));
    assert!(Platform::Pico8.accepts(std::path::Path::new("Celeste.p8.png")));
    assert!(Platform::Chip8.accepts(std::path::Path::new("Space Invaders.ch8")));
    assert!(Platform::Chip8.accepts(std::path::Path::new("Black Rainbow.xo8")));
    assert!(Platform::Java.accepts(std::path::Path::new("Bounce Tales.jar")));
    assert!(Platform::Java.accepts(std::path::Path::new("Opera Mini.jad")));
    assert!(Platform::Vircon32.accepts(std::path::Path::new("Frogster.v32")));
    assert!(Platform::Uzem.accepts(std::path::Path::new("Uze8.uze")));
    assert!(Platform::Pc9800.accepts(std::path::Path::new("Touhou 1.hdi")));
    assert!(Platform::Pc9800.accepts(std::path::Path::new("Touhou Collection.m3u")));
    assert!(Platform::AtariJaguarCd.accepts(std::path::Path::new("Vid Grid.cue")));
    assert!(Platform::AtariJaguarCd.accepts(std::path::Path::new("Vid Grid.chd")));
    assert!(!Platform::Gba.accepts(std::path::Path::new("Tetris.gb")));
    assert!(Platform::Gb.accepts(std::path::Path::new("Tetris.gb")));
    assert!(Platform::Gb.accepts(std::path::Path::new("Tetris.gbc")));
    assert!(!Platform::Gb.accepts(std::path::Path::new("Metroid Fusion.gba")));
    assert!(Platform::Gba.accepts(std::path::Path::new("Shrek.GBA")));
    assert!(Platform::Genesis.accepts(std::path::Path::new("Sonic.md")));
    assert!(Platform::Genesis.accepts(std::path::Path::new("Sonic.smd")));
    assert!(Platform::Genesis.accepts(std::path::Path::new("Sonic.bin")));
    assert!(Platform::Sega32X.accepts(std::path::Path::new("Chaotix.32x")));
    assert!(Platform::SegaCd.accepts(std::path::Path::new("Sonic CD.cue")));
    assert!(Platform::SegaCd.accepts(std::path::Path::new("Sonic CD.chd")));
    assert!(Platform::SegaCd.accepts(std::path::Path::new("Lunar.m3u")));
    assert!(!Platform::SegaCd.accepts(std::path::Path::new("track01.bin")));
    assert!(Platform::Nes.accepts(std::path::Path::new("Super Mario Bros.nes")));
    assert!(Platform::Nes.accepts(std::path::Path::new("Homebrew.unif")));
    assert!(!Platform::Nes.accepts(std::path::Path::new("Super Mario World.sfc")));
    assert!(!Platform::Genesis.accepts(std::path::Path::new("Chaotix.32x")));
    assert!(Platform::SuperGrafx.accepts(std::path::Path::new("1941.pce")));
    assert!(Platform::NeoGeoCd.accepts(std::path::Path::new("The King of Fighters 95.chd")));
    assert!(Platform::Cps1.accepts(std::path::Path::new("sf2.zip")));
    assert!(Platform::Cps2.accepts(std::path::Path::new("mshvsf.7z")));
    assert!(Platform::Cps3.accepts(std::path::Path::new("sfiii3.zip")));
    assert!(Platform::FbNeo.accepts(std::path::Path::new("ddonpach.zip")));
    assert!(Platform::Mame.accepts(std::path::Path::new("pacman.zip")));
    assert!(!Platform::Mame.accepts(std::path::Path::new("pacman.7z")));
    assert!(Platform::Gp32.accepts(std::path::Path::new("gp32.zip")));
    assert!(Platform::Gp32.accepts(std::path::Path::new("game.smc")));
    assert!(Platform::Gp32.accepts(std::path::Path::new("collection.7z")));
    assert!(Platform::Vectrex.accepts(std::path::Path::new("Minestorm.vec")));
    assert!(Platform::Vectrex.accepts(std::path::Path::new("Armor Attack.bin")));
    assert!(Platform::Cd32.accepts(std::path::Path::new("Alien Breed 3D.chd")));
    assert!(Platform::Cd32.accepts(std::path::Path::new("Microcosm.cue")));
    assert!(Platform::Cd32.accepts(std::path::Path::new("Collection.m3u")));
    assert!(!Platform::Cd32.accepts(std::path::Path::new("track01.bin")));
}
