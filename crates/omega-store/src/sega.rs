//! Metadata from the common Mega Drive/Genesis ROM header, also used by 32X cartridges.

use std::path::Path;

/// The international title field in a Sega cartridge header.
const TITLE: std::ops::Range<usize> = 0x150..0x180;

/// Reads the international title, trimming the header's space padding and control bytes.
pub fn title(path: &Path) -> Option<String> {
    let rom = std::fs::read(path).ok()?;
    let bytes = rom.get(TITLE)?;
    let title: String = bytes
        .iter()
        .map(|&b| {
            if b.is_ascii_graphic() || b == b' ' {
                b as char
            } else {
                ' '
            }
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    (!title.is_empty()).then_some(title)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_and_cleans_the_international_title() {
        let path =
            std::env::temp_dir().join(format!("omega-sega-header-{}.bin", std::process::id()));
        let mut rom = vec![0; TITLE.end];
        rom[TITLE.start..TITLE.start + 15].copy_from_slice(b"  SONIC   TWO  ");
        std::fs::write(&path, rom).unwrap();
        assert_eq!(title(&path).as_deref(), Some("SONIC TWO"));
        let _ = std::fs::remove_file(path);
    }
}
