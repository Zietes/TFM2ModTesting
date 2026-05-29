//! Minimal native Rust mod skeleton for Teamfight Manager 2, with a sample
//! HUD ModExtension for visible testing.
//!
//! Build (from the folder that contains the Mod SDK):
//!   build_mod.bat path\to\mods\example_native            (Cargo project)
//!   build_mod.bat path\to\mods\example_native\src\lib.rs (single file)
//!
//! Everything lives in this one file so the single-file build path keeps
//! working. The produced DLL is named after the mod folder, and MOD_ID below
//! MUST match that folder name ("example_native").

use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};

use mod_api::*;

/// Must equal the mod folder name and the id used at upload time.
const MOD_ID: &str = "example_native";

/// How often the HUD refreshes / logs, in seconds.
const HUD_INTERVAL_SECS: f32 = 1.0;

/// Entry point. The game calls this once to learn what the mod provides.
fn init(_ctx: &GameCtx) -> ModRegistration {
    let mut reg = ModRegistration::new(MOD_ID);

    // Sample extension: a per-frame HUD that reads live game state.
    reg.set_extension(HudExtension::default());

    // --- Other content plugs in the same way --------------------------------
    //   reg.add_champion(MyChampion);
    //   reg.add_item(MyItem::default());
    //   reg.add_draft_score_hook(MyDraftHook);
    //   reg.add_player_input_ai(MyInputAi::default());
    //   reg.set_server_extension(MyServerExtension);
    // -------------------------------------------------------------------------

    reg
}

/// A simple heads-up display extension.
///
/// `ModExtension` methods take `&self`, so all mutable bookkeeping uses atomics
/// (also keeps the type `Send + Sync` regardless of the trait's bounds). The
/// game calls these hooks on the game-loop thread every frame.
#[derive(Default)]
struct HudExtension {
    /// Frames observed since load -- proves `post_update` is really ticking.
    frames: AtomicU64,
    /// Accumulated time since the last HUD refresh, stored as milliseconds.
    elapsed_ms: AtomicU32,
}

impl HudExtension {
    /// Returns `true` once roughly every `HUD_INTERVAL_SECS`, resetting the
    /// accumulator. Lets us throttle work instead of running it every frame.
    fn tick_due(&self, dt: f32) -> bool {
        let added = (dt * 1000.0) as u32;
        let total = self.elapsed_ms.fetch_add(added, Ordering::Relaxed) + added;
        if total >= (HUD_INTERVAL_SECS * 1000.0) as u32 {
            self.elapsed_ms.store(0, Ordering::Relaxed);
            true
        } else {
            false
        }
    }

    /// Builds the HUD text from live game state. Kept tiny and defensive: every
    /// lookup is guarded, so a missing value never panics on the hot loop.
    ///
    /// NOTE: `ClientData` exposes the management/career layer (teams, athletes,
    /// leagues, matches...). Live combat figures (entity hp, tower count, score
    /// diff) come from `GameCtx` inside champion effects / AI hooks, not here.
    fn hud_text(&self, data: &ClientData) -> String {
        let frames = self.frames.load(Ordering::Relaxed);

        // Mirrors the working example from the native-rust-mods guide.
        // (If the SDK rustdoc names these differently, this is the spot to fix.)
        let team = {
            let id = data.player_team_id();
            data.team(id).map(|t| t.name.clone())
        };

        match team {
            Some(name) => format!("[{MOD_ID}] frame {frames} | team: {name}"),
            None => format!("[{MOD_ID}] frame {frames} | (no player team yet)"),
        }
    }
}

impl ModExtension for HudExtension {
    /// Per-frame logic. We only do work once a second, and only when in a game
    /// scene. The log line is the guaranteed-visible test signal.
    fn post_update(&self, scene: &mut Scene, _ui: &mut GameUI, _assets: &mut Assets, dt: f32) {
        self.frames.fetch_add(1, Ordering::Relaxed);

        if !self.tick_due(dt) {
            return;
        }

        let Scene::InGame { data } = scene else {
            // Not in a game scene -- nothing to show.
            return;
        };

        let line = self.hud_text(data);
        // Visible in the game's console / log output.
        println!("{line}");
    }

    /// On-screen overlay goes here. This is the ONE part the public modding
    /// guide does not document -- the exact methods for creating a text/label
    /// `Node` and attaching it via `RenderState`/`GameUI` live in the SDK's own
    /// rustdoc. Left as a compiling no-op until we wire those real calls in.
    ///
    /// TODO(ui): using `Node` / `NodeTemplate` / `RenderState` from `mod_api`,
    /// draw `self.hud_text(data)` in a screen corner when `Scene::InGame`.
    fn post_render(
        &self,
        _scene: &Scene,
        _ui: &GameUI,
        _assets: &Assets,
        _state: &mut RenderState,
    ) {
    }
}

// Exports the required DLL entry symbols. Without this the game cannot load it.
declare_mod!(init);
