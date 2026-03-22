use yew::prelude::*;
use crate::app::class_info::class::ClassSettings;

#[derive(Properties, PartialEq)]
pub struct StatProps {
    pub settings: ClassSettings,
}

#[function_component(StatDisplay)]
pub fn stat_display(props: &StatProps) -> Html {
    let p = &props.settings.primary_stats;
    let s = &props.settings.secondary_stats;

    let primaries = vec![
        ("Strength", p.strength),
        ("Intellect", p.intellect),
        ("Endurance", p.endurance),
        ("Dexterity", p.dexterity),
        ("Wisdom", p.wisdom),
        ("Luck", p.luck),
    ];

    let secondaries = vec![
        ("Hit Points", s.hp as f32),
        ("Mana", 100.0),
        ("Haste", s.haste),
        ("Crit Chance", s.crit_chance),
        ("Hit Chance", s.hit_chance),
        ("Dodge Chance", s.dodge),
        ("Crit Modifier", s.crit_mod),
        ("Mana Consumption", s.mana_consumption),
        ("All Out", s.all_out),
        ("All In", s.all_in),
        ("Phy Out", s.phy_out),
        ("Phy In", s.phy_in),
        ("Mag Out", s.mag_out),
        ("Mag In", s.mag_in),
        ("Heal Out", s.heal_out),
        ("Heal In", s.heal_in),
        ("DoT Out", s.dot_out),
        ("DoT In", s.dot_in),
        ("Attack Power", s.attack_power),
        ("Spell Power", s.spell_power)
    ];

    html! {
        <div class="stats-screen">
            <div class="stat-block">
                <h4>{"Primary"}</h4>
                { for primaries.into_iter().map(|(label, val)| html! {
                    <div class="stat-row">
                        <span class="label">{label}</span>
                        <span class="value">{val}</span>
                    </div>
                })}
            </div>

            <div class="stat-block">
                <h4>{"Secondary"}</h4>
                { for secondaries.into_iter().map(|(label, val)| html! {
                    <div class="stat-row">
                        <span class="label">{label}</span>
                        <span class="value">{ format!("{:.2}%", val * 100.0) }</span>
                    </div>
                })}
            </div>
        </div>
    }
}