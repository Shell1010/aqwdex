use yew::prelude::*;
use crate::app::class_info::class::ClassSettings;
use crate::app::storage::{load_all_builds, save_all_builds};

#[derive(Properties, PartialEq)]
pub struct BuildManagerProps {
    pub current_settings: ClassSettings,
    pub on_load_build: Callback<ClassSettings>,
}

#[function_component(BuildManager)]
pub fn build_manager(props: &BuildManagerProps) -> Html {
    let selected_load = use_state(|| "".to_string());

    let saved_names = use_state(|| {
        let builds = load_all_builds();
        let mut names: Vec<String> = builds.keys().cloned().collect();
        names.sort();
        names
    });

    let on_save = {
        let saved_names = saved_names.clone();
        let current_settings = props.current_settings.clone();
    
        Callback::from(move |_| {
            let mut builds = load_all_builds();
            let base_name = current_settings.name.trim();
            let base_name = if base_name.is_empty() { "New Build" } else { base_name };
            
            let mut unique_name = base_name.to_string();
            let mut counter = 1;
            while builds.contains_key(&unique_name) {
                unique_name = format!("{}-{}", base_name, counter);
                counter += 1;
            }
    
            builds.insert(unique_name, current_settings.clone());
            save_all_builds(&builds);
    
            let mut names: Vec<String> = builds.keys().cloned().collect();
            names.sort();
            saved_names.set(names);
        })
    };

    let on_load = {
        let selected_load = selected_load.clone();
        let on_load_build = props.on_load_build.clone();

        Callback::from(move |_| {
            let name = (*selected_load).clone();
            if name.is_empty() { return; }

            let builds = load_all_builds();
            if let Some(settings) = builds.get(&name) {
                on_load_build.emit(settings.clone());
            }
        })
    };

    html! {
        <div class="build-manager">
            <h4>{"Save / Load Builds"}</h4>
            <button class="save-btn" onclick={on_save}>{"Save Build"}</button>

            <div class="input-field">
                <label>{"Load Saved Build: "}</label>
                <div class="form-row">
                    <select onchange={
                        let selected_load = selected_load.clone();
                        Callback::from(move |e: Event| {
                            let select = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                            selected_load.set(select.value());
                        })
                    }>
                        <option value="">{"-- Select --"}</option>
                        { for (*saved_names).iter().map(|name| html! {
                            <option value={name.clone()} selected={*selected_load == *name}>{name}</option>
                        })}
                    </select>
                    <button class="load-btn" onclick={on_load} disabled={(*selected_load).is_empty()}>{"Load"}</button>
                </div>
            </div>
        </div>
    }
}