pub mod player;
pub mod error;
pub mod gear;
pub mod damage;
pub mod enemy;


#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    #[test]
    fn test_class_model_from_str() {
        use player::ClassModel;
        assert_eq!(ClassModel::from_str("Tank Melee").unwrap(), ClassModel::TankMelee);
        assert_eq!(ClassModel::from_str("power-caster").unwrap(), ClassModel::PowerCaster);
        assert!(ClassModel::from_str("unknown").is_err());
    }
}

