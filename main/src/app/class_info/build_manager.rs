use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::app::class_info::class::ClassSettings;
use crate::app::storage::{load_all_builds, save_all_builds};

#[derive(Properties, PartialEq)]
pub struct BuildManagerProps {
    pub current_settings: ClassSettings,
    pub on_load_build: Callback<ClassSettings>,
}

#[function_component(BuildManager)]
pub fn build_manager(props: &BuildManagerProps) -> Html {

    let saved_names = use_state(Vec::<String>::new);

    // ── Load saved build names on first mount ────────────────────────────────
    {
        let saved_names = saved_names.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                let builds = load_all_builds().await;
                let mut names: Vec<String> = builds.keys().cloned().collect();
                names.sort();
                saved_names.set(names);
            });
            || ()
        });
    }

    // ── Save current settings ────────────────────────────────────────────────
    let on_save = {
        let saved_names = saved_names.clone();
        let current_settings = props.current_settings.clone();

        Callback::from(move |_| {
            let saved_names = saved_names.clone();
            let current_settings = current_settings.clone();

            spawn_local(async move {
                let mut builds = load_all_builds().await;
                let base_name = if current_settings.name.trim().is_empty() {
                    "New Build".to_string()
                } else {
                    current_settings.name.trim().to_string()
                };

                let mut unique_name = base_name.clone();
                let mut counter = 1;
                while builds.contains_key(&unique_name) {
                    unique_name = format!("{}-{}", base_name, counter);
                    counter += 1;
                }

                builds.insert(unique_name, current_settings.clone());
                save_all_builds(&builds).await;

                let mut names: Vec<String> = builds.keys().cloned().collect();
                names.sort();
                saved_names.set(names);
            });
        })
    };

    // ── Load a named build ───────────────────────────────────────────────────
    let load_build = {
        let on_load_build = props.on_load_build.clone();
        move |name: String| {
            let on_load_build = on_load_build.clone();
            spawn_local(async move {
                let builds = load_all_builds().await;
                if let Some(settings) = builds.get(&name) {
                    on_load_build.emit(settings.clone());
                }
            });
        }
    };

    // ── Delete a named build ─────────────────────────────────────────────────
    let delete_build = {
        let saved_names = saved_names.clone();
        move |name: String| {
            let saved_names = saved_names.clone();
            spawn_local(async move {
                let mut builds = load_all_builds().await;
                builds.remove(&name);
                save_all_builds(&builds).await;
                let mut names: Vec<String> = builds.keys().cloned().collect();
                names.sort();
                saved_names.set(names);
            });
        }
    };

    // ── Render ───────────────────────────────────────────────────────────────
    html! {
        <div class="build-manager">
            <div class="build-header">
                <h4>{"System / Builds"}</h4>
                <button class="save-btn" onclick={on_save}>{"[+] Save Current"}</button>
            </div>

            <div class="build-directory">
                { for (*saved_names).iter().map(|name| {
                    let name_clone = name.clone();
                    let load_cb = {
                        let name = name.clone();
                        let load_build = load_build.clone();
                        Callback::from(move |_| load_build(name.clone()))
                    };
                    let delete_cb = {
                        let name = name.clone();
                        let delete_build = delete_build.clone();
                        Callback::from(move |e: MouseEvent| {
                            e.stop_propagation();
                            delete_build(name.clone());
                        })
                    };

                    html! {
                        <div class="build-item" onclick={load_cb}>
                            <span class="build-icon">{"📄"}</span>
                            <span class="build-name">{name_clone}</span>
                            <div class="build-item-actions">
                                <button class="mini-load-btn">{"LOAD"}</button>
                                <button class="mini-delete-btn" onclick={delete_cb}>{"DEL"}</button>
                            </div>
                        </div>
                    }
                })}
            </div>

            <div class="build-io">
                <button class="io-btn">{"Export String"}</button>
                <button class="io-btn">{"Import String"}</button>
            </div>
        </div>
    }
}
