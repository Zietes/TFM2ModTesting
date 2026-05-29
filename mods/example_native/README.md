# Example Native Mod

A minimal native Rust mod skeleton for **Teamfight Manager 2**. It compiles to
`example_native.dll`, loads cleanly, and registers no content yet -- a safe
baseline to build on.

## Layout

```
example_native/
  mod.mod_info     # required metadata; "base" dependency = game version
  Cargo.toml       # crate-type = cdylib; do NOT add mod-api here
  src/lib.rs       # init() + declare_mod!(init)
  .gitignore       # ignores target/, *.dll, mod.workshop_id
```

The **folder name is the mod id** and must match `MOD_ID` in `src/lib.rs`
(`example_native`).

## Building

Place the Mod SDK next to the game executable, then from the SDK folder:

```bat
build_mod.bat path\to\mods\example_native
```

The uploader detects `Cargo.toml` first (use it when you need external crates)
and falls back to `src\lib.rs` otherwise. The resulting DLL lands in this folder,
named after it. Rebuild after any game/SDK update.

> Do **not** add `mod-api` to `[dependencies]`. The SDK injects the matching
> prebuilt `mod_api` crate so the DLL stays compatible with your game version.

## Extending it

Add content inside `init()` in `src/lib.rs`:

| Trait | Registered with | Purpose |
|---|---|---|
| `ModChampionInfo` | `add_champion` | Custom champion with simulation logic |
| `ModItemInfo` | `add_item` | Item with runtime callbacks / upgrade tree |
| `ModDraftScoreHook` | `add_draft_score_hook` | Adjust ban/pick AI scoring |
| `ModPlayerInputAi` | `add_player_input_ai` | Replace final per-tick player input |
| `ModExtension` | `set_extension` | Client lifecycle / UI hooks |
| `ModServerExtension` | `set_server_extension` | Authoritative server-side logic |

## Publishing

Build, then run `TFM2ModUploader.exe` from the game folder and select this
folder. The first upload writes `mod.workshop_id` -- keep it to update the same
Workshop item. The uploader excludes `src/`, `target/`, `Cargo.toml`, and
workshop-id files, so your Rust source stays private.

## Troubleshooting

Most load problems appear in the title-screen diagnostics popup. Common causes:
DLL missing from the folder, mod id not matching the folder name, missing
`declare_mod!`, or an SDK version mismatch. Target Windows `x86_64-pc-windows-msvc`,
release profile.
