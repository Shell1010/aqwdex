use yew::prelude::*;
use crate::app::{player::PlayerPanel, monster::MonsterPanel};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="aqwdex page-container">
            <header class="aqw-header center">
                <h1>{ "AQWDex" }</h1>
                
                <h4 class="smol">{ "Made by Archfishy"}</h4>
            </header>

            <main class="aqw-main">
                <section class="player-panel">
                    <h2 class="pane-title">{ "Player" }</h2>
                    <PlayerPanel />
                </section>

                <section class="pane monster">
                    <h2 class="pane-title">{ "Monster" }</h2>
                    <MonsterPanel />
                </section>
            </main>

            
        </div>
    }
}
