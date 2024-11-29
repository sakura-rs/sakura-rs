use crate::common::ProtocolEntityID;

pub const fn to_protocol_entity_id(
    ty: mavuika_proto::ProtEntityType,
    index: u32,
) -> ProtocolEntityID {
    ProtocolEntityID(((ty as u32) << 24) | index)
}

#[macro_export]
macro_rules! fight_props {
    ($($prop_ty:ident: $val:expr),*) => {
        FightProperties(::std::collections::HashMap::from([$(
            (::mavuika_data::prop_type::FightPropType::$prop_ty, $val),
        )*]))
    };
}

#[macro_export]
macro_rules! int_prop_value {
    ($prop_name:ident, $value:expr) => {
        ::mavuika_proto::PropValue {
            r#type: ::mavuika_data::prop_type::$prop_name,
            val: $value as i64,
            value: Some(::mavuika_proto::prop_value::Value::Ival($value as i64)),
        }
    };
}

#[macro_export]
macro_rules! int_prop_pair {
    ($prop_name:ident, $value:expr) => {
        ::mavuika_proto::PropPair {
            r#type: ::mavuika_data::prop_type::$prop_name,
            prop_value: Some(::mavuika_proto::PropValue {
                r#type: ::mavuika_data::prop_type::$prop_name,
                val: $value as i64,
                value: Some(::mavuika_proto::prop_value::Value::Ival($value as i64)),
            }),
        }
    };
}

#[macro_export]
macro_rules! int_prop_map {
    ($($prop_name:ident: $value:expr;)*) => {
        ::std::collections::HashMap::from([$((
            ::mavuika_data::prop_type::$prop_name,
            ::mavuika_proto::PropValue {
                r#type: ::mavuika_data::prop_type::$prop_name,
                val: $value as i64,
                value: Some(::mavuika_proto::prop_value::Value::Ival($value as i64)),
            },
        ),)*])
    };
}
