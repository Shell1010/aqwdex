use yew::prelude::*;

#[function_component(MonsterPanel)]
pub fn monster_panel() -> Html {
    html! {
        <>
            <div class="card monster-stats">
                <h3 class="card-title">{ "Stats" }</h3>
                <div class="stat-grid">
                    { for ["HP", "RES (MAG)", "RES (PHY)", "DODGE"].iter().map(|s| html! {
                        <div class="stat">
                            <span class="stat-key">{ s.to_string() }</span>
                            <span class="stat-val">{ "0" }</span>
                        </div>
                    }) }
                </div>
            </div>

            <div class="card ai">
                <h3 class="card-title">{ "AI / Notes" }</h3>
                <textarea placeholder="Monster notes or special properties..."></textarea>
            </div>
        </>
    }
}
