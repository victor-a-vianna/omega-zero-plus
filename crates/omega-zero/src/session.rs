use std::path::PathBuf;
use std::time::Duration;

use omega_input::{Action, Gestures, Millis, RawEvent};
use omega_retro::{ButtonMask, ComputerInput, Rumble};
use omega_store::Platform;
use omega_ui::FfState;

use crate::app::{App, Phase};
use crate::audio::{open_sink, AudioSink, Ring, Sfx, GBA_HZ};
use crate::core::{open, Setup};
use crate::emu::{CoreState, EmuHandle, Speed};
use crate::frames::FrameRef;
use crate::input::Pad;
use crate::persist;

/// How many frames a core runs before the card stops noting it as opening: two seconds, well
/// past the first frame, where a core that cannot take its options gives up.
const SETTLED_FRAMES: u64 = 120;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
enum ComputerMode {
    #[default]
    Game,
    Keyboard,
    Mouse,
}

pub struct Session {
    root: PathBuf,
    app: App,
    emu: Option<EmuHandle>,
    sink: Box<dyn AudioSink>,
    gestures: Gestures,
    pad: Pad,
    computer_mode: ComputerMode,
    computer_chord: bool,
    rewinding: bool,
    fast: bool,
    motor: u16,
    reloading: bool,
    driven: bool,
    /// The system and stem of the game the core was last opened for.
    running: Option<(Platform, String)>,
    /// The card notes the core as opening, until it has run a moment.
    starting: bool,
    /// A standalone emulator has been requested and the device frontend must release EGL.
    standalone: bool,
}

impl Drop for Session {
    /// A session put away, as at power-off, leaves no game noted as opening: only one that went
    /// down with its core, and so never got here, does.
    fn drop(&mut self) {
        if self.starting {
            persist::clear_starting(&self.root);
        }
    }
}

impl Session {
    pub fn boot(root: PathBuf) -> Self {
        let mut sink: Box<dyn AudioSink> = open_sink();
        if let Err(e) = sink.open(GBA_HZ) {
            eprintln!("omega-zero: audio: {e}");
        }
        Session {
            app: App::boot(&root),
            root,
            emu: None,
            sink,
            gestures: Gestures::new(),
            pad: Pad::default(),
            computer_mode: ComputerMode::Game,
            computer_chord: false,
            rewinding: false,
            fast: false,
            motor: 0,
            reloading: false,
            driven: false,
            running: None,
            starting: false,
            standalone: false,
        }
    }

    pub fn play_sfx(&mut self, sfx: Sfx) {
        let ring = self.sink.ring();
        let rate = ring.sample_rate();
        if rate == 0 {
            return;
        }
        let mut samples = sfx.render(rate);
        crate::audio::volume::apply(&mut samples, self.app.output_volume());
        ring.mix(&samples);
    }

    pub fn audio_queued(&self) -> usize {
        self.sink.ring().queued_frames()
    }

    pub fn audio_ring(&self) -> std::sync::Arc<Ring> {
        self.sink.ring()
    }

    pub fn rumble(&mut self, strength: u16) {
        if strength == self.motor {
            return;
        }
        self.motor = strength;
        self.app.set_rumble(strength);
    }

    pub fn core_rumble(&self) -> Option<&Rumble> {
        self.emu.as_ref().map(EmuHandle::rumble)
    }

    pub fn app(&self) -> &App {
        &self.app
    }

    pub fn app_mut(&mut self) -> &mut App {
        &mut self.app
    }

    pub fn emu(&self) -> Option<&EmuHandle> {
        self.emu.as_ref()
    }

    pub fn set_driven(&mut self, driven: bool) {
        self.driven = driven;
        if let Some(emu) = &self.emu {
            emu.set_driven(driven);
        }
    }

    pub fn step_emulator(&self, present: Duration, timeout: Duration) -> bool {
        self.emu.as_ref().is_some_and(|emu| {
            emu.tick(present);
            emu.wait_frame(timeout)
        })
    }

    pub fn frame(&self) -> Option<FrameRef> {
        self.emu.as_ref().and_then(|e| e.latest_frame())
    }

    pub fn core_settling(&self) -> bool {
        self.emu
            .as_ref()
            .is_some_and(|e| e.state() == CoreState::Loading)
    }

    pub fn has_core(&self) -> bool {
        self.emu.is_some()
    }

    pub fn standalone_requested(&self) -> bool {
        self.standalone
    }

    pub fn frame_ready(&self) -> bool {
        self.emu.as_ref().is_some_and(EmuHandle::frame_ready)
    }

    pub fn frames_published(&self) -> u64 {
        self.emu.as_ref().map_or(0, EmuHandle::published_count)
    }

    pub fn observed_speed(&self) -> Option<Speed> {
        self.emu.as_ref().map(EmuHandle::observed_speed)
    }

    pub fn frames_taken(&self) -> u64 {
        self.emu.as_ref().map_or(0, EmuHandle::frames_taken)
    }

    pub fn game_visible(&self) -> bool {
        self.app.game_visible()
    }

    pub fn feed(&mut self, events: impl IntoIterator<Item = RawEvent>, now: Millis) {
        let keys = self.app.keys();
        self.pad.set_keys(keys);
        let mut actions = self.gestures.set_keys(keys);
        for ev in events {
            actions.extend(self.gestures.feed(ev, now));
        }
        actions.extend(self.gestures.tick(now));
        for action in actions {
            self.act(action);
        }
        self.sync_pad();
    }

    fn sync_pad(&mut self) {
        for btn in self.app.taken_buttons() {
            self.pad.apply(Action::GbaUp(*btn));
        }
        if let Some(emu) = &self.emu {
            let raw = self.pad.mask();
            let platform = self.app.platform();
            let computer = matches!(platform, Platform::Pc8800 | Platform::Pc9800);
            let keyboard_chord =
                raw.0 & (ButtonMask::SELECT | ButtonMask::Y) == ButtonMask::SELECT | ButtonMask::Y;
            let mouse_chord = platform == Platform::Pc9800
                && raw.0 & (ButtonMask::SELECT | ButtonMask::X)
                    == ButtonMask::SELECT | ButtonMask::X;
            let chord = keyboard_chord || mouse_chord;
            if computer && chord && !self.computer_chord {
                let requested = if keyboard_chord {
                    ComputerMode::Keyboard
                } else {
                    ComputerMode::Mouse
                };
                self.computer_mode = if self.computer_mode == requested {
                    ComputerMode::Game
                } else {
                    requested
                };
                eprintln!(
                    "omega-zero: {} input mode: {:?}",
                    platform.dir_name(),
                    self.computer_mode
                );
            }
            self.computer_chord = chord;
            if !computer {
                self.computer_mode = ComputerMode::Game;
                self.computer_chord = false;
            }

            let mut aux = ComputerInput::default();
            let game = match self.computer_mode {
                ComputerMode::Game => raw,
                ComputerMode::Keyboard => {
                    if raw.0 & ButtonMask::UP != 0 {
                        aux.keys |= ComputerInput::UP;
                    }
                    if raw.0 & ButtonMask::DOWN != 0 {
                        aux.keys |= ComputerInput::DOWN;
                    }
                    if raw.0 & ButtonMask::LEFT != 0 {
                        aux.keys |= ComputerInput::LEFT;
                    }
                    if raw.0 & ButtonMask::RIGHT != 0 {
                        aux.keys |= ComputerInput::RIGHT;
                    }
                    if raw.0 & ButtonMask::A != 0 {
                        aux.keys |= ComputerInput::ENTER;
                    }
                    if raw.0 & ButtonMask::B != 0 {
                        aux.keys |= ComputerInput::SPACE;
                    }
                    if raw.0 & ButtonMask::X != 0 {
                        aux.keys |= ComputerInput::ESCAPE;
                    }
                    if raw.0 & ButtonMask::Y != 0 {
                        aux.keys |= ComputerInput::TAB;
                    }
                    if raw.0 & ButtonMask::L != 0 {
                        aux.keys |= ComputerInput::SHIFT;
                    }
                    if raw.0 & ButtonMask::R != 0 {
                        aux.keys |= ComputerInput::CONTROL;
                    }
                    if raw.0 & ButtonMask::START != 0 {
                        aux.keys |= ComputerInput::BACKSPACE;
                    }
                    ButtonMask(0)
                }
                ComputerMode::Mouse => {
                    const STEP: i16 = 6;
                    if raw.0 & ButtonMask::LEFT != 0 {
                        aux.mouse_x -= STEP;
                    }
                    if raw.0 & ButtonMask::RIGHT != 0 {
                        aux.mouse_x += STEP;
                    }
                    if raw.0 & ButtonMask::UP != 0 {
                        aux.mouse_y -= STEP;
                    }
                    if raw.0 & ButtonMask::DOWN != 0 {
                        aux.mouse_y += STEP;
                    }
                    if raw.0 & ButtonMask::A != 0 {
                        aux.mouse_buttons |= ComputerInput::MOUSE_LEFT;
                    }
                    if raw.0 & ButtonMask::B != 0 {
                        aux.mouse_buttons |= ComputerInput::MOUSE_RIGHT;
                    }
                    ButtonMask(0)
                }
            };
            emu.set_computer_input(aux);
            emu.set_input(if chord { ButtonMask(0) } else { game });
        }
    }

    fn bridge_link(&mut self, f: impl FnOnce(&mut App)) {
        let had_link = self.app.link_active();
        f(&mut self.app);
        if had_link && !self.app.link_active() {
            if let Some(emu) = &self.emu {
                emu.end_link();
            }
        }
    }

    fn act(&mut self, action: Action) {
        if trace() {
            eprintln!("omega-zero: {action:?} in {:?}", self.app.phase());
        }
        match action {
            Action::RewindStart => self.rewinding = true,
            Action::RewindStop => self.rewinding = false,
            Action::FfStart => self.fast = true,
            Action::FfStop => self.fast = false,
            _ => {}
        }
        let menu = self.overlaid();
        self.bridge_link(|app| app.apply(action));
        if matches!(
            action,
            Action::VolumeUp | Action::VolumeDown | Action::MuteToggle
        ) {
            if let Some(emu) = &self.emu {
                emu.set_volume(self.app.output_volume());
            }
        }
        if menu || self.overlaid() {
            self.pad.clear();
        } else if self.app.takes_from_the_game(action) {
            if let Action::GbaDown(btn) | Action::GbaUp(btn) = action {
                self.pad.apply(Action::GbaUp(btn));
            }
        } else {
            self.pad.apply(action);
        }
        self.sync_rumble();
    }

    pub fn update(&mut self, dt: f32) {
        self.bridge_link(|app| app.update(dt));
        if let Some(emu) = &self.emu {
            emu.set_volume(self.app.output_volume());
        }
        if let Some((client_id, transport)) = self.app.take_link_transport() {
            match &self.emu {
                Some(emu) => match self.app.link_player() {
                    Some(player) => emu.begin_cable(player, transport),
                    None => emu.begin_link(client_id, transport),
                },
                None => eprintln!("omega-zero: link: a transport arrived with no core to run it"),
            }
        }
        if let Some(on) = self.app.take_colour_correction() {
            if let Some((key, value)) = crate::core::colour_option(self.app.core(), on) {
                if let Some(emu) = &self.emu {
                    emu.set_option(key, value);
                }
            }
        }
        if let Some(disc) = self.app.take_disc_swap() {
            if let Some(emu) = &self.emu {
                emu.set_disc(disc);
            }
        }
        if let Some((stem, serial)) = self.app.take_link_reload() {
            self.reload_for_link(&stem, serial);
        }
        if self.app.link_active() {
            if self.emu.as_ref().is_some_and(EmuHandle::bios_mismatch) {
                self.bridge_link(|app| app.bios_mismatch());
            } else if self.emu.as_ref().is_some_and(EmuHandle::peer_ended) {
                self.bridge_link(|app| app.peer_ended());
            } else if self.emu.as_ref().is_some_and(EmuHandle::link_lost) {
                self.app.peer_lost();
            }
        }
        if let Some(sfx) = self.app.take_sfx() {
            self.play_sfx(sfx);
        }
        self.sync_core();
        self.settle_start();
        self.sync_reload();
        self.keep_swapped_disc();
        if !self.has_core() {
            for action in self.gestures.drop_ff_latch() {
                self.act(action);
            }
        }
        self.app
            .set_game_ready(self.emu.as_ref().is_some_and(EmuHandle::has_published));
        self.sync_speed();
        self.sync_rewind_hud();
        self.sync_ff_hud();
        self.sync_rumble();
        self.sync_pad();
    }

    fn sync_rumble(&mut self) {
        let want = match &self.emu {
            Some(emu) if self.playing() && self.app.rumble_enabled() => emu.rumble().strength(),
            _ => 0,
        };
        self.rumble(want);
    }

    fn sync_ff_hud(&mut self) {
        let ff = match (self.actually_fast_forwarding(), self.gestures.ff_latched()) {
            (false, _) => FfState::Off,
            (true, false) => FfState::Held,
            (true, true) => FfState::Latched,
        };
        self.app.set_ff(ff);
    }

    fn sync_rewind_hud(&mut self) {
        let fill = self
            .actually_rewinding()
            .then(|| self.emu.as_ref().map(EmuHandle::rewind_fill))
            .flatten();
        match fill {
            Some(fill) => self.app.show_rewind(fill),
            None => self.app.hide_rewind(),
        }
    }

    fn actually_rewinding(&self) -> bool {
        self.rewinding && self.playing() && self.app.may_rewind()
    }

    fn actually_fast_forwarding(&self) -> bool {
        self.fast && self.playing() && self.app.may_fast_forward()
    }

    fn inserting(&self) -> bool {
        matches!(self.app.phase(), Phase::Inserting { .. })
    }

    fn showing_polaroids(&self) -> bool {
        matches!(self.app.phase(), Phase::Polaroids { .. })
    }

    fn overlaid(&self) -> bool {
        self.showing_polaroids() || self.held()
    }

    fn playing(&self) -> bool {
        matches!(self.app.phase(), Phase::Playing { .. }) && !self.held()
    }

    fn held(&self) -> bool {
        self.app.power_shown() || self.app.game_menu_open() || self.app.shutting_down()
    }

    fn dozing(&self) -> bool {
        matches!(self.app.phase(), Phase::Doze { .. })
    }

    fn ejecting(&self) -> bool {
        matches!(self.app.phase(), Phase::Ejecting { .. })
    }

    fn sync_speed(&self) {
        if let Some(emu) = &self.emu {
            emu.set_fast_steps(u32::from(self.app.ff_speed()));
            emu.set_ff_sound(self.app.ff_sound());
            emu.set_speed(
                if self.inserting()
                    || self.ejecting()
                    || self.showing_polaroids()
                    || self.dozing()
                    || (self.held() && !self.app.link_active())
                {
                    Speed::Paused
                } else if self.actually_fast_forwarding() {
                    Speed::Fast
                } else {
                    Speed::Normal
                },
            );
            emu.set_rewinding(self.actually_rewinding());
        }
    }

    fn no_core() -> bool {
        std::env::var_os("OMEGA_NO_CORE").is_some_and(|v| v != "0")
    }

    fn sync_core(&mut self) {
        if Self::no_core() {
            return;
        }
        let stem = match self.app.phase() {
            Phase::Shelf => {
                self.emu = None;
                return;
            }
            Phase::Inserting { cart, .. } => cart.clone(),
            _ => return,
        };
        if self.app.seated_cart().is_some_and(|cart| {
            matches!(
                cart.platform,
                Platform::NeoGeo
                    | Platform::NeoGeoCd
                    | Platform::AtariJaguar
                    | Platform::AtariJaguarCd
                    | Platform::Daphne
                    | Platform::Pc9800
                    | Platform::Cps1
                    | Platform::Cps2
                    | Platform::Cps3
                    | Platform::FbNeo
                    | Platform::Mame
            ) && !self.app.cart_load_ready()
        }) {
            return;
        }
        if self.emu.is_none() {
            let (_, serial) = self.app.link_mode(&stem);
            self.spawn_core(&stem, serial);
        }
        if self.standalone {
            return;
        }
        match self.emu.as_ref().map(EmuHandle::state) {
            Some(CoreState::Loading) => {}
            Some(CoreState::Ready) => self.app.on_core_ready(),
            Some(CoreState::Failed) | None => {
                self.emu = None;
                self.app.on_core_failed();
            }
        }
    }

    fn spawn_core(&mut self, stem: &str, serial: &'static str) {
        let Some((rom, platform)) = self
            .app
            .seated_cart()
            .filter(|c| c.stem == stem)
            .map(|c| (c.rom.clone(), c.platform))
        else {
            return;
        };
        let core = omega_store::core_for_platform(&self.root, stem, platform);
        if matches!(platform, Platform::AtariJaguar | Platform::AtariJaguarCd) {
            self.app.set_core(core);
            self.app.set_platform(platform);
            self.app.set_link_loaded(serial);
            let request = self.root.join("System/bigpemu.launch");
            let temporary = request.with_extension("launch.tmp");
            let result = std::fs::write(&temporary, format!("{}\n", rom.display()))
                .and_then(|()| std::fs::rename(&temporary, &request));
            match result {
                Ok(()) => self.standalone = true,
                Err(e) => {
                    let _ = std::fs::remove_file(&temporary);
                    eprintln!("omega-zero: BigPEmu launch request failed: {e}");
                    self.app.on_core_failed();
                }
            }
            return;
        }
        if platform == Platform::Daphne {
            self.app.set_core(core);
            self.app.set_platform(platform);
            self.app.set_link_loaded(serial);
            let request = self.root.join("System/hypseus.launch");
            let temporary = request.with_extension("launch.tmp");
            let result = std::fs::write(&temporary, format!("{}\n", rom.display()))
                .and_then(|()| std::fs::rename(&temporary, &request));
            match result {
                Ok(()) => self.standalone = true,
                Err(e) => {
                    let _ = std::fs::remove_file(&temporary);
                    eprintln!("omega-zero: Hypseus launch request failed: {e}");
                    self.app.on_core_failed();
                }
            }
            return;
        }
        if platform == Platform::SegaCd && !crate::root::has_segacd_bios(&self.root) {
            eprintln!(
                "omega-zero: Sega CD BIOS missing; add bios_CD_U.bin, bios_CD_E.bin or bios_CD_J.bin to BIOS/"
            );
        }
        if platform == Platform::Cd32 && !crate::root::has_cd32_bios(&self.root) {
            eprintln!(
                "omega-zero: CD32 BIOS missing or invalid; add the 1 MiB kick40060.CD32, or the 512 KiB kick40060.CD32 plus kick40060.CD32.ext, to BIOS/"
            );
        }
        if platform == Platform::FamicomDiskSystem && !crate::root::has_fds_bios(&self.root) {
            eprintln!(
                "omega-zero: Famicom Disk System BIOS missing or invalid; add the 8192-byte disksys.rom to BIOS/"
            );
        }
        if platform == Platform::AtariSt && !crate::root::has_atari_st_bios(&self.root) {
            eprintln!("omega-zero: Atari ST BIOS missing; add tos.img to BIOS/");
        }
        if platform == Platform::Pc9800 && !crate::root::has_np2kai_bios(&self.root) {
            eprintln!(
                "omega-zero: PC-9800 BIOS missing; add bios.rom and font.rom (or font.bmp) to BIOS/np2kai/"
            );
        }
        if platform == Platform::Pc8800 && !crate::root::has_quasi88_bios(&self.root) {
            eprintln!(
                "omega-zero: PC-8800 BIOS set missing; QUASI88 will use its fallback BIOS. Add N88.ROM and N88SUB.ROM (or DISK.ROM) to BIOS/quasi88/ for full compatibility"
            );
        }
        if platform == Platform::SufamiTurbo && !crate::root::has_sufami_turbo_bios(&self.root) {
            eprintln!(
                "omega-zero: SuFami Turbo BIOS missing or invalid; add the 262144-byte STBIOS.bin to BIOS/"
            );
        }
        if platform == Platform::Tg16CdPceCd && !crate::root::has_pce_cd_bios(&self.root) {
            eprintln!("omega-zero: PC Engine CD BIOS missing; add syscard3.pce to BIOS/");
        }
        if platform == Platform::PcFx && !crate::root::has_pcfx_bios(&self.root) {
            eprintln!("omega-zero: PC-FX BIOS missing or invalid; add the 1 MiB pcfx.rom to BIOS/");
        }
        if platform == Platform::NeoGeo && !crate::root::has_neogeo_bios(&self.root) {
            eprintln!("omega-zero: Neo Geo BIOS missing; add neogeo.zip to BIOS/fbneo/");
        }
        if platform == Platform::NeoGeoCd && !crate::root::has_neogeo_cd_bios(&self.root) {
            eprintln!(
                "omega-zero: Neo Geo CD BIOS missing; add neogeo.zip and neocdz.zip to BIOS/fbneo/"
            );
        }
        if platform == Platform::Atari5200 && !crate::root::has_atari5200_bios(&self.root) {
            eprintln!("omega-zero: Atari 5200 BIOS missing or invalid; add the 2048-byte 5200.rom to BIOS/");
        }
        self.app.set_core(core);
        self.app.set_platform(platform);
        self.app
            .set_video_mode(crate::video_mode::video_mode_for(&self.root, stem));
        self.app.set_link_loaded(serial);
        // NP2Kai 1ea561c can crash inside retro_unserialize while restoring its OPNA
        // state (FM::Timer::DataLoad with a null destination). Start PC-98 content clean
        // until the core can reject incompatible states without taking down the frontend.
        let resume = (core != omega_store::Core::Np2Kai && !self.app.starting_clean())
            .then(|| persist::read_resume(&self.root, platform, core, stem))
            .flatten();
        let player = self.app.link_player();
        let setup = match platform {
            Platform::Genesis
            | Platform::GameGear
            | Platform::MasterSystem
            | Platform::Sg1000
            | Platform::Sega32X
            | Platform::SegaCd
            | Platform::Nes
            | Platform::FamicomDiskSystem
            | Platform::Atari2600
            | Platform::Atari800
            | Platform::Atari7800
            | Platform::AtariLynx
            | Platform::VirtualBoy
            | Platform::PokemonMini
            | Platform::GameAndWatch
            | Platform::Arduboy
            | Platform::WonderSwan
            | Platform::WonderSwanColor
            | Platform::MegaDuck
            | Platform::Supervision
            | Platform::Tic80
            | Platform::Chip8
            | Platform::Pico8
            | Platform::Java
            | Platform::Vircon32
            | Platform::Uzem
            | Platform::Daphne
            | Platform::NeoGeoPocket
            | Platform::NeoGeoPocketColor
            | Platform::AtariJaguar
            | Platform::AtariJaguarCd
            | Platform::AtariSt
            | Platform::Pc8800
            | Platform::Pc9800
            | Platform::Tg16Pce
            | Platform::Tg16CdPceCd
            | Platform::SuperGrafx
            | Platform::PcFx
            | Platform::NeoGeo
            | Platform::NeoGeoCd
            | Platform::Cps1
            | Platform::Cps2
            | Platform::Cps3
            | Platform::FbNeo
            | Platform::Mame
            | Platform::Gp32
            | Platform::Vectrex
            | Platform::Cd32
            | Platform::SufamiTurbo
            | Platform::Snes
            | Platform::Ps1 => Setup::Console,
            Platform::Atari5200 => Setup::Handheld {
                serial: "5200",
                colour: false,
                link: None,
            },
            Platform::Gba | Platform::Gb | Platform::Gbc => Setup::Handheld {
                serial,
                colour: self.app.colour_correction(),
                link: player,
            },
        };
        let opened = open(&self.root, core, setup);
        self.app.set_named_core(opened.named);
        let mut game = opened.core;
        // a game put away on its second disc comes back on it, for its resume state is of that
        // disc; a game started afresh starts from the first
        if let Some((index, path)) = resume
            .is_some()
            .then(|| persist::read_disc(&self.root, platform, stem))
            .flatten()
        {
            game.set_initial_disc(index, &path);
            self.app.set_disc_in(stem, index);
        }
        let sav = persist::read_sav(&self.root, platform, stem);
        persist::mark_starting(&self.root, platform, stem);
        self.starting = true;
        let ring = self.sink.ring();
        let emu = EmuHandle::spawn_for(platform, game, rom, ring, sav, resume, player);
        emu.set_volume(self.app.output_volume());
        emu.set_driven(self.driven);
        self.app.set_snapshot(Box::new(emu.snapshot()));
        self.emu = Some(emu);
        self.running = Some((platform, stem.to_string()));
    }

    /// Takes the note of the core opening off the card once it has run a moment, or once there
    /// is no core: it opened, or failed to, without taking omega-zero down.
    fn settle_start(&mut self) {
        let settled = self
            .emu
            .as_ref()
            .is_none_or(|emu| emu.published_count() >= SETTLED_FRAMES);
        if self.starting && settled {
            persist::clear_starting(&self.root);
            self.starting = false;
        }
    }

    /// Keeps the disc a swap put in, so that the game comes back on it.
    fn keep_swapped_disc(&mut self) {
        let (Some(emu), Some((platform, stem))) = (&self.emu, &self.running) else {
            return;
        };
        if let Some((index, path)) = emu.take_swapped_disc() {
            if let Err(e) = persist::write_disc(&self.root, *platform, stem, index, &path) {
                eprintln!("omega-zero: disc: could not keep disc {index} of {stem}: {e}");
            }
        }
    }

    fn reload_for_link(&mut self, stem: &str, serial: &'static str) {
        eprintln!("omega-zero: link: loading {stem} again with gpsp_serial={serial}");
        if self.emu.is_some() {
            self.app.flush_resume();
        }
        self.emu = None;
        self.spawn_core(stem, serial);
        self.reloading = true;
    }

    fn sync_reload(&mut self) {
        if !self.reloading {
            return;
        }
        match self.emu.as_ref().map(EmuHandle::state) {
            Some(CoreState::Loading) => {}
            Some(CoreState::Ready) => {
                self.reloading = false;
                self.app.link_reload_done();
            }
            Some(CoreState::Failed) | None => {
                self.reloading = false;
                self.emu = None;
                self.app.link_reload_failed();
                if let Some((stem, serial)) = self.app.take_link_reload() {
                    self.reload_for_link(&stem, serial);
                }
            }
        }
    }
}

pub(crate) fn trace() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("OMEGA_TRACE").is_some())
}
