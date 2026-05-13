use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use js_sys::{Object, Reflect, Function, Promise};
use crate::app::class_info::class::ClassSettings;

const STORAGE_KEY: &str = "dex_builds";

// ── Tauri IPC helpers ────────────────────────────────────────────────────────

/// Returns the Tauri 2 `invoke` function if we're running inside a Tauri WebView,
/// or `None` if we're in a plain browser (trunk serve / web build).
fn tauri_invoke_fn() -> Option<Function> {
    let window = web_sys::window()?;
    // Tauri 2 injects its internals under window.__TAURI_INTERNALS__
    let internals = Reflect::get(&window, &JsValue::from_str("__TAURI_INTERNALS__")).ok()?;
    if internals.is_undefined() || internals.is_null() {
        return None;
    }
    let invoke = Reflect::get(&internals, &JsValue::from_str("invoke")).ok()?;
    invoke.dyn_into::<Function>().ok()
}

/// Calls a Tauri command and returns its `Promise`, or `None` if not in Tauri.
fn tauri_call(invoke_fn: &Function, cmd: &str, args: &JsValue) -> Option<Promise> {
    invoke_fn
        .call2(&JsValue::NULL, &JsValue::from_str(cmd), args)
        .ok()?
        .dyn_into::<Promise>()
        .ok()
}

// ── localStorage fallbacks (used in browser / trunk serve) ──────────────────

fn ls_load() -> HashMap<String, ClassSettings> {
    web_sys::window()
        .and_then(|w| w.local_storage().ok().flatten())
        .and_then(|s| s.get_item(STORAGE_KEY).ok().flatten())
        .and_then(|json| serde_json::from_str(&json).ok())
        .unwrap_or_default()
}

fn ls_save(builds: &HashMap<String, ClassSettings>) {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(json) = serde_json::to_string(builds) {
                let _ = storage.set_item(STORAGE_KEY, &json);
            }
        }
    }
}

// ── Public async API ─────────────────────────────────────────────────────────

/// Load all saved builds.
///
/// In Tauri: reads `{app_data_dir}/builds.json` via the `load_builds` command.
/// In browser: reads from `localStorage`.
pub async fn load_all_builds() -> HashMap<String, ClassSettings> {
    if let Some(invoke_fn) = tauri_invoke_fn() {
        // load_builds takes no arguments — pass an empty object
        let args = Object::new();
        if let Some(promise) = tauri_call(&invoke_fn, "load_builds", &args.into()) {
            if let Ok(result) = JsFuture::from(promise).await {
                if let Some(json) = result.as_string() {
                    if let Ok(builds) = serde_json::from_str::<HashMap<String, ClassSettings>>(&json) {
                        return builds;
                    }
                }
            }
        }
    }

    ls_load()
}

/// Persist all builds.
///
/// In Tauri: writes `{app_data_dir}/builds.json` via the `save_builds` command.
/// In browser: writes to `localStorage`.
pub async fn save_all_builds(builds: &HashMap<String, ClassSettings>) {
    if let Some(invoke_fn) = tauri_invoke_fn() {
        if let Ok(json) = serde_json::to_string(builds) {
            // Tauri 2 converts camelCase JS arg names → snake_case Rust params.
            // buildsJson  →  builds_json: String
            let args = Object::new();
            let _ = Reflect::set(
                &args,
                &JsValue::from_str("buildsJson"),
                &JsValue::from_str(&json),
            );
            if let Some(promise) = tauri_call(&invoke_fn, "save_builds", &args.into()) {
                // Await so the write completes before the caller continues.
                let _ = JsFuture::from(promise).await;
                return;
            }
        }
    }

    ls_save(builds);
}
