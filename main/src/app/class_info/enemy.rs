use yew::prelude::*;
use backend::enemy::EnemySecondaryStats;
use crate::app::class_info::class::ClassSettings;

#[derive(Properties, PartialEq)]
pub struct EnemyProps {
    pub settings: ClassSettings,
    pub on_update_enemy: Callback<EnemySecondaryStats>,
}

/// Generates an `oninput` callback that updates one f32 field on EnemySecondaryStats.
macro_rules! f32_cb {
    ($enemy:expr, $on_update:expr, $field:ident) => {{
        let e = $enemy.clone();
        let u = $on_update.clone();
        Callback::from(move |ev: InputEvent| {
            let mut ne = e.clone();
            if let Ok(val) = ev
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value()
                .parse::<f32>()
            {
                ne.$field = val;
                u.emit(ne);
            }
        })
    }};
}

/// Generates an `oninput` callback that updates one i32 field on EnemySecondaryStats.
macro_rules! i32_cb {
    ($enemy:expr, $on_update:expr, $field:ident) => {{
        let e = $enemy.clone();
        let u = $on_update.clone();
        Callback::from(move |ev: InputEvent| {
            let mut ne = e.clone();
            if let Ok(val) = ev
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value()
                .parse::<i32>()
            {
                ne.$field = val;
                u.emit(ne);
            }
        })
    }};
}

#[function_component(EnemyPanel)]
pub fn enemy_panel(props: &EnemyProps) -> Html {
    let enemy = props.settings.enemy.clone();
    let on_update = props.on_update_enemy.clone();

    let cb_all_in      = f32_cb!(enemy, on_update, all_in);
    let cb_phy_in      = f32_cb!(enemy, on_update, phy_in);
    let cb_mag_in      = f32_cb!(enemy, on_update, mag_in);
    let cb_dot_in      = f32_cb!(enemy, on_update, dot_in);
    let cb_heal_in     = f32_cb!(enemy, on_update, heal_in);
    let cb_all_out     = f32_cb!(enemy, on_update, all_out);
    let cb_phy_out     = f32_cb!(enemy, on_update, phy_out);
    let cb_mag_out     = f32_cb!(enemy, on_update, mag_out);
    let cb_dot_out     = f32_cb!(enemy, on_update, dot_out);
    let cb_heal_out    = f32_cb!(enemy, on_update, heal_out);
    let cb_crit_chance = f32_cb!(enemy, on_update, crit_chance);
    let cb_crit_mod    = f32_cb!(enemy, on_update, crit_mod);
    let cb_haste       = f32_cb!(enemy, on_update, haste);
    let cb_dodge       = f32_cb!(enemy, on_update, dodge);
    let cb_hit_chance  = f32_cb!(enemy, on_update, hit_chance);
    let cb_ap          = f32_cb!(enemy, on_update, attack_power);
    let cb_sp          = f32_cb!(enemy, on_update, spell_power);
    let cb_mana_con    = f32_cb!(enemy, on_update, mana_consumption);

    let cb_hp = i32_cb!(enemy, on_update, hp);
    let cb_mp = i32_cb!(enemy, on_update, mp);

    macro_rules! stat_row_f32 {
        ($label:expr, $val:expr, $step:expr, $cb:expr) => {
            html! {
                <div class="stat-row">
                    <span class="label">{$label}</span>
                    <input
                        type="number"
                        step={$step}
                        class="table-input"
                        style="width: 90px; text-align: right; color: #7ee787; background: transparent; border: none; border-bottom: 1px dashed var(--border-color);"
                        value={format!("{:.2}", $val)}
                        oninput={$cb}
                    />
                </div>
            }
        };
    }

    macro_rules! stat_row_i32 {
        ($label:expr, $val:expr, $cb:expr) => {
            html! {
                <div class="stat-row">
                    <span class="label">{$label}</span>
                    <input
                        type="number"
                        step="1"
                        class="table-input"
                        style="width: 90px; text-align: right; color: #7ee787; background: transparent; border: none; border-bottom: 1px dashed var(--border-color);"
                        value={$val.to_string()}
                        oninput={$cb}
                    />
                </div>
            }
        };
    }

    html! {
        <div class="enemy-panel panel-right-section" style="margin-bottom: 20px;">
            <h3>{"Enemy Configuration"}</h3>
            <div class="stats-screen">
                <div class="stat-block">
                    <h4>{"Secondary"}</h4>

                    { stat_row_f32!("All In (%)",         enemy.all_in,          "0.1", cb_all_in)      }
                    { stat_row_f32!("Phy In (%)",         enemy.phy_in,          "0.1", cb_phy_in)      }
                    { stat_row_f32!("Mag In (%)",         enemy.mag_in,          "0.1", cb_mag_in)      }
                    { stat_row_f32!("DoT In (%)",         enemy.dot_in,          "0.1", cb_dot_in)      }
                    { stat_row_f32!("Heal In (%)",        enemy.heal_in,         "0.1", cb_heal_in)     }
                    { stat_row_f32!("All Out (%)",        enemy.all_out,         "0.1", cb_all_out)     }
                    { stat_row_f32!("Phy Out (%)",        enemy.phy_out,         "0.1", cb_phy_out)     }
                    { stat_row_f32!("Mag Out (%)",        enemy.mag_out,         "0.1", cb_mag_out)     }
                    { stat_row_f32!("DoT Out (%)",        enemy.dot_out,         "0.1", cb_dot_out)     }
                    { stat_row_f32!("Heal Out (%)",       enemy.heal_out,        "0.1", cb_heal_out)    }
                    { stat_row_f32!("Crit Chance (%)",    enemy.crit_chance,     "0.1", cb_crit_chance) }
                    { stat_row_f32!("Crit Modifier (%)",  enemy.crit_mod,        "0.1", cb_crit_mod)    }
                    { stat_row_f32!("Haste (%)",          enemy.haste,           "0.1", cb_haste)       }
                    { stat_row_f32!("Dodge (%)",          enemy.dodge,           "0.1", cb_dodge)       }
                    { stat_row_f32!("Hit Chance (%)",     enemy.hit_chance,      "0.1", cb_hit_chance)  }
                    { stat_row_f32!("Attack Power",       enemy.attack_power,    "1",   cb_ap)          }
                    { stat_row_f32!("Spell Power",        enemy.spell_power,     "1",   cb_sp)          }
                    { stat_row_f32!("Mana Con (%)",       enemy.mana_consumption,"0.1", cb_mana_con)    }
                    { stat_row_i32!("Max HP",             enemy.hp,              cb_hp)                 }
                    { stat_row_i32!("Max MP",             enemy.mp,              cb_mp)                 }
                </div>
            </div>
        </div>
    }
}
