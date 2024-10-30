use crate::common::ProtocolEntityID;

pub const fn to_protocol_entity_id(
    ty: sakura_proto::ProtEntityType,
    index: u32,
) -> ProtocolEntityID {
    ProtocolEntityID(((ty as u32) << 24) | index)
}

#[macro_export]
macro_rules! fight_props {
    ($($prop_ty:ident: $val:expr),*) => {
        FightProperties(::std::collections::HashMap::from([$(
            (::sakura_data::prop_type::FightPropType::$prop_ty, $val),
        )*]))
    };
}

#[macro_export]
macro_rules! int_prop_value {
    ($prop_name:ident, $value:expr) => {
        ::sakura_proto::PropValue {
            r#type: ::sakura_data::prop_type::$prop_name,
            val: $value as i64,
            value: Some(::sakura_proto::prop_value::Value::Ival($value as i64)),
        }
    };
}

#[macro_export]
macro_rules! int_prop_pair {
    ($prop_name:ident, $value:expr) => {
        ::sakura_proto::PropPair {
            r#type: ::sakura_data::prop_type::$prop_name,
            prop_value: Some(::sakura_proto::PropValue {
                r#type: ::sakura_data::prop_type::$prop_name,
                val: $value as i64,
                value: Some(::sakura_proto::prop_value::Value::Ival($value as i64)),
            }),
        }
    };
}

#[macro_export]
macro_rules! int_prop_map {
    ($($prop_name:ident: $value:expr;)*) => {
        ::std::collections::HashMap::from([$((
            ::sakura_data::prop_type::$prop_name,
            ::sakura_proto::PropValue {
                r#type: ::sakura_data::prop_type::$prop_name,
                val: $value as i64,
                value: Some(::sakura_proto::prop_value::Value::Ival($value as i64)),
            },
        ),)*])
    };
}
