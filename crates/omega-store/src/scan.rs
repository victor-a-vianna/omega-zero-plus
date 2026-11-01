use std::fmt;
use std::path::{Path, PathBuf};

use crate::gba::{header_code, header_title};
use crate::platform::Platform;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Cart {
    pub platform: Platform,
    pub stem: String,
    pub rom: PathBuf,
    pub label: Option<PathBuf>,
    pub title: String,
    pub code: String,
    pub shell: Option<crate::ShellChoice>,
}

#[derive(Debug)]
pub enum StoreError {
    Io(std::io::Error),
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StoreError::Io(e) => write!(f, "io: {e}"),
        }
    }
}

impl std::error::Error for StoreError {}

impl From<std::io::Error> for StoreError {
    fn from(e: std::io::Error) -> Self {
        StoreError::Io(e)
    }
}

pub fn scan(root: &Path) -> Result<Vec<Cart>, StoreError> {
    let read = |file: &str| std::fs::read_to_string(root.join(file)).unwrap_or_default();
    let shells = crate::cart_shell::layered(
        &read(crate::CART_SHELL_FILE),
        &read(crate::LABELS_SHELL_FILE),
    );
    let mut carts = Vec::new();
    for platform in Platform::ALL {
        let mut dirs = vec![root.join("Games").join(platform.dir_name())];
        if let Some(legacy) = platform.legacy_dir_name() {
            dirs.push(root.join("Games").join(legacy));
        }
        let mut found = Vec::new();
        for dir in dirs {
            let scanned = match platform {
                Platform::Daphne => daphne(&dir),
                Platform::Ps1 => crate::ps1::games(root, &dir)
                    .map(|games| games.into_iter().map(|g| (g.stem, g.rom)).collect()),
                Platform::SegaCd
                | Platform::AtariJaguarCd
                | Platform::Tg16CdPceCd
                | Platform::NeoGeoCd
                | Platform::PcFx => { crate::ps1::games_for(root, &dir, platform) }
                    .map(|games| games.into_iter().map(|g| (g.stem, g.rom)).collect()),
                _ => flat(&dir, platform),
            };
            match scanned {
                Ok(mut scanned) => found.append(&mut scanned),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
                Err(e) => eprintln!("omega-zero: scan: {}: {e}", dir.display()),
            }
        }
        for (stem, rom) in found {
            let label = label_of(root, platform, &stem, &rom);
            let (title, code) = match platform {
                Platform::Gba => (
                    header_title(&rom).unwrap_or_default(),
                    header_code(&rom).unwrap_or_default(),
                ),
                Platform::Gb | Platform::Gbc => {
                    (crate::gb::title(&rom).unwrap_or_default(), String::new())
                }
                Platform::Genesis
                | Platform::Sega32X
                | Platform::GameGear
                | Platform::MasterSystem
                | Platform::Sg1000 => (crate::sega::title(&rom).unwrap_or_default(), String::new()),
                Platform::SegaCd
                | Platform::Cd32
                | Platform::PcFx
                | Platform::Java
                | Platform::Vircon32
                | Platform::Uzem
                | Platform::Daphne => (String::new(), String::new()),
                Platform::Nes => (String::new(), String::new()),
                Platform::FamicomDiskSystem | Platform::SufamiTurbo => {
                    (String::new(), String::new())
                }
                Platform::Atari2600
                | Platform::Atari800
                | Platform::Atari5200
                | Platform::Atari7800
                | Platform::AtariLynx
                | Platform::VirtualBoy => (String::new(), String::new()),
                Platform::PokemonMini => (String::new(), String::new()),
                Platform::GameAndWatch => (String::new(), String::new()),
                Platform::Arduboy => (String::new(), String::new()),
                Platform::WonderSwan | Platform::WonderSwanColor => (String::new(), String::new()),
                Platform::MegaDuck => (String::new(), String::new()),
                Platform::Supervision => (String::new(), String::new()),
                Platform::Tic80 | Platform::Pico8 | Platform::Chip8 => {
                    (String::new(), String::new())
                }
                Platform::NeoGeoPocket | Platform::NeoGeoPocketColor => {
                    (String::new(), String::new())
                }
                Platform::AtariJaguar
                | Platform::AtariJaguarCd
                | Platform::AtariSt
                | Platform::Pc8800
                | Platform::Pc9800 => (String::new(), String::new()),
                Platform::Tg16Pce | Platform::Tg16CdPceCd | Platform::SuperGrafx => {
                    (String::new(), String::new())
                }
                Platform::NeoGeo
                | Platform::NeoGeoCd
                | Platform::Cps1
                | Platform::Cps2
                | Platform::Cps3
                | Platform::FbNeo
                | Platform::Mame
                | Platform::Gp32
                | Platform::Vectrex => (String::new(), String::new()),
                Platform::Snes => (
                    crate::snes::header(&rom)
                        .map(|h| h.title)
                        .unwrap_or_default(),
                    String::new(),
                ),
                Platform::Ps1 => (String::new(), crate::ps1::serial(&rom).unwrap_or_default()),
            };
            carts.push(Cart {
                platform,
                shell: shells.get(&crate::cart_shell::key(&stem)).copied(),
                stem,
                title,
                code,
                label,
                rom,
            });
        }
    }
    carts.sort_by(|a, b| {
        (a.platform as u8, sort_key(&a.stem)).cmp(&(b.platform as u8, sort_key(&b.stem)))
    });
    Ok(carts)
}

/// Traditional Daphne/Hypseus games are directories named `<driver>.daphne`. Keeping the
/// framefile, MPEG/OGG media and arcade ROM together makes a game portable as one directory.
fn daphne(dir: &Path) -> std::io::Result<Vec<(String, PathBuf)>> {
    let mut found = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let Ok(entry) = entry else { continue };
        let path = entry.path();
        if is_hidden(&path) || !path.is_dir() {
            continue;
        }
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        let Some(stem) = name.strip_suffix(".daphne") else {
            continue;
        };
        if !stem.is_empty() && path.join(format!("{stem}.txt")).is_file() {
            found.push((stem.to_string(), path));
        }
    }
    Ok(found)
}

/// The ROMs directly in `dir` that `platform` takes, each with its file's name.
fn flat(dir: &Path, platform: Platform) -> std::io::Result<Vec<(String, PathBuf)>> {
    let mut found = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let Ok(entry) = entry else {
            continue;
        };
        let rom = entry.path();
        if is_hidden(&rom) || !rom.is_file() || !platform.accepts(&rom) {
            continue;
        }
        if let Some(stem) = rom.file_stem().and_then(|s| s.to_str()) {
            let stem = if platform == Platform::Pico8 {
                stem.strip_suffix(".p8").unwrap_or(stem)
            } else {
                stem
            };
            found.push((stem.to_string(), rom));
        }
    }
    Ok(found)
}

/// The label of the cart named `stem`: `Labels/<system>/<stem>.png`, or for a PlayStation game
/// that has none under its own name, its first disc's.
fn label_of(root: &Path, platform: Platform, stem: &str, rom: &Path) -> Option<PathBuf> {
    let own = root
        .join("Labels")
        .join(platform.dir_name())
        .join(format!("{stem}.png"));
    if own.is_file() {
        return Some(own);
    }
    if let Some(legacy) = platform.legacy_dir_name() {
        let old = root.join("Labels").join(legacy).join(format!("{stem}.png"));
        if old.is_file() {
            return Some(old);
        }
    }
    if !matches!(
        platform,
        Platform::Ps1
            | Platform::SegaCd
            | Platform::Tg16CdPceCd
            | Platform::NeoGeoCd
            | Platform::PcFx
    ) {
        return None;
    }
    let first = crate::ps1::discs(rom).into_iter().next()?;
    Some(crate::ps1::disc_label_for(root, platform, &first)).filter(|l| l.is_file())
}

pub fn sort_key(stem: &str) -> (u8, String) {
    (group_of(stem), stem.to_uppercase())
}

fn group_of(stem: &str) -> u8 {
    match stem.chars().find(|c| !c.is_whitespace()) {
        Some(c) if c.is_ascii_digit() => 0,
        Some(c) if c.is_alphabetic() => 1,
        _ => 2,
    }
}

pub fn initial(stem: &str) -> char {
    match group_of(stem) {
        1 => stem
            .chars()
            .find(|c| !c.is_whitespace())
            .and_then(|c| c.to_uppercase().next())
            .unwrap_or('#'),
        _ => '#',
    }
}

pub fn is_hidden(p: &Path) -> bool {
    p.file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|n| n.starts_with('.'))
}
