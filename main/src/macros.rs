

#[macro_export]
macro_rules! push_passives {
    // Helper to handle the repetition
    ($list:expr, $( $target:ident, $stat:expr => $val:expr $(, $op:ident)? );* $(;)?) => {
        $(
            $list.push($crate::app::class_info::passive::CustomPassive {
                target_type: $crate::app::class_info::passive::TargetType::$target,
                stat_name: $stat.to_string(),
                value: $val as f32,
                operation_type: push_passives!(@op $($op)?), // Calls the internal @op helper
                ..Default::default()
            });
        )*
    };

    // Internal helper to handle the operation type default
    (@op Add) => { $crate::app::class_info::passive::OperationType::Additive };
    (@op Multi) => { $crate::app::class_info::passive::OperationType::Multiplicative };
    (@op) => { $crate::app::class_info::passive::OperationType::Additive }; // Default if nothing is provided
}