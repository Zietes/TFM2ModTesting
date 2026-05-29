//! Minimal native Rust mod skeleton for Teamfight Manager 2.
//!
//! This compiles to `example_native.dll` and registers nothing by default --
//! it is a safe, working baseline you can extend. The commented blocks below
//! show where each kind of content plugs in.
//!
//! Build (from the folder that contains the Mod SDK):
//!   build_mod.bat path\to\mods\example_native            (Cargo project)
//!   build_mod.bat path\to\mods\example_native\src\lib.rs (single file)
//!
//! The produced DLL is named after the mod folder, and MOD_ID below MUST match
//! that folder name ("example_native").

use mod_api::*;

/// Must equal the mod folder name and the id used at upload time.
const MOD_ID: &str = "example_native";

/// Entry point. The game calls this once to learn what the mod provides.
///
/// Return a `ModRegistration` describing your content. Start empty and add
/// pieces incrementally so a load failure is easy to localize.
fn init(_ctx: &GameCtx) -> ModRegistration {
    let reg = ModRegistration::new(MOD_ID);

    // --- Add content here as you build it out ----------------------------
    //
    // Champions (implement `ModChampionInfo`):
    //   reg.add_champion(MyChampion);
    //
    // Items (implement `ModItemInfo`):
    //   reg.add_item(MyItem::default());
    //
    // Draft ban/pick AI scoring (implement `ModDraftScoreHook`):
    //   reg.add_draft_score_hook(MyDraftHook);
    //
    // Final player input AI override (implement `ModPlayerInputAi`):
    //   reg.add_player_input_ai(MyInputAi::default());
    //
    // Client-side lifecycle / UI hooks (implement `ModExtension`):
    //   reg.set_extension(MyExtension);
    //
    // Authoritative server-side logic (implement `ModServerExtension`):
    //   reg.set_server_extension(MyServerExtension);
    // ---------------------------------------------------------------------

    reg
}

// Exports the required DLL entry symbols (`tfm2_mod_entry` and the API version
// symbol). Without this macro the game cannot load the mod.
declare_mod!(init);
