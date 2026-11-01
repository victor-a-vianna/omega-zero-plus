use tempfile::TempDir;

pub fn tmp_root() -> TempDir {
    let d = tempfile::tempdir().expect("tempdir");
    for sub in [
        "Games",
        "Games/GBA",
        "Games/GB",
        "Games/GBC",
        "Games/GENESIS",
        "Games/32X",
        "Games/SEGACD",
        "Games/NES",
        "Games/PICO8",
        "Labels",
        "Saves",
        "States",
        "System",
        "Config",
    ] {
        std::fs::create_dir_all(d.path().join(sub)).expect("create content dir");
    }
    d
}
