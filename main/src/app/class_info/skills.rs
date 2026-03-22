use yew::prelude::*;
use crate::app::class_info::class::ClassSettings;
use backend::damage::{Skill};
#[derive(Properties, PartialEq)]
pub struct SkillProps {
    pub settings: ClassSettings,
}



#[function_component(Skills)]
pub fn skills(props: &SkillProps) -> Html {
    let s = &props.settings;

    // 1. Calculate the 'Base' for the skills 
    // Usually AP = Str * 2 or SP = Int * 2 depending on Class Model
    let base_power = (s.primary_stats.strength as f64 * 2.0) + (s.level.level as f64 * 1.5);

    // 2. Define the skill list (Replace with your actual backend skill data if available)
    let skills = vec![
        ("Auto Attack", 1.0, "Physical"),
        ("Skill 2", 1.5, "Magical"),
        ("Skill 3", 2.2, "Physical"),
        ("Skill 4", 0.0, "Buff/Utility"), // 0 multiplier for non-damage skills
        ("Ultimate", 4.5, "Hybrid"),
    ];

    html! {
        <div class="skills-section">
            <h3>{"Combat Skills"}</h3>
            <div class="skills-grid">
                { for skills.into_iter().map(|(name, mult, damage_type)| {
                    let damage = base_power * mult;
                    
                    html! {
                        <div class="skill-card" key={name}>
                            <div class="skill-info">
                                <span class="skill-name">{ name }</span>
                                <span class="skill-type">{ damage_type }</span>
                            </div>
                            <div class="skill-damage">
                                { if mult > 0.0 {
                                    format!("{:.0}", damage)
                                } else {
                                    "---".to_string()
                                }}
                            </div>
                        </div>
                    }
                })}
            </div>
        </div>
    }
}