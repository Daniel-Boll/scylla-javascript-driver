use napi::bindgen_prelude::{BigInt, Either13, Either14};
use scylla::frame::response::result::CqlValue;

use std::collections::HashMap;

use crate::types::{
  decimal::Decimal, duration::Duration, float::Float, list::List, map::Map, set::Set, uuid::Uuid,
  varint::Varint,
};

use super::to_cql_value::ToCqlValue;

macro_rules! define_expected_type {
    ($lifetime:lifetime, $($t:ty),+) => {
      pub type ParameterNativeTypes<$lifetime> = Either13<$($t),+>;
      pub type ParameterWithMapType<$lifetime> = Either14<$($t),+, HashMap<String, ParameterNativeTypes<$lifetime>>>;
      pub type JSQueryParameters<$lifetime> = napi::Result<Vec<HashMap<String, ParameterWithMapType<$lifetime>>>>;
    };
}

define_expected_type!('a, u32, String, &'a Uuid, BigInt, &'a Duration, &'a Decimal, bool, Vec<u32>, &'a Float, &'a Varint, &'a List, &'a Set, &'a Map);

impl<'a> ToCqlValue for ParameterWithMapType<'a> {
  fn to_cql_value(&self) -> CqlValue {
    match self {
      ParameterWithMapType::A(num) => num.to_cql_value(),
      ParameterWithMapType::B(str) => str.to_cql_value(),
      ParameterWithMapType::C(uuid) => uuid.to_cql_value(),
      ParameterWithMapType::D(bigint) => bigint.to_cql_value(),
      ParameterWithMapType::E(duration) => duration.to_cql_value(),
      ParameterWithMapType::F(decimal) => decimal.to_cql_value(),
      ParameterWithMapType::G(bool_val) => bool_val.to_cql_value(),
      ParameterWithMapType::H(buffer) => buffer.to_cql_value(),
      ParameterWithMapType::I(float) => float.to_cql_value(),
      ParameterWithMapType::J(varint) => varint.to_cql_value(),
      ParameterWithMapType::K(list) => list.to_cql_value(),
      ParameterWithMapType::L(set) => set.to_cql_value(),
      ParameterWithMapType::M(map) => map.to_cql_value(),
      ParameterWithMapType::N(map) => CqlValue::UserDefinedType {
        // TODO: think a better way to fill this info here
        keyspace: "keyspace".to_string(),
        type_name: "type_name".to_string(),
        fields: map
          .iter()
          .map(|(key, value)| (key.clone(), Some(value.to_cql_value())))
          .collect::<Vec<(String, Option<CqlValue>)>>(),
      },
    }
  }
}

impl<'a> ToCqlValue for ParameterNativeTypes<'a> {
  fn to_cql_value(&self) -> CqlValue {
    match self {
      ParameterNativeTypes::A(num) => num.to_cql_value(),
      ParameterNativeTypes::B(str) => str.to_cql_value(),
      ParameterNativeTypes::C(uuid) => uuid.to_cql_value(),
      ParameterNativeTypes::D(bigint) => bigint.to_cql_value(),
      ParameterNativeTypes::E(duration) => duration.to_cql_value(),
      ParameterNativeTypes::F(decimal) => decimal.to_cql_value(),
      ParameterNativeTypes::G(bool_val) => bool_val.to_cql_value(),
      ParameterNativeTypes::H(buffer) => buffer.to_cql_value(),
      ParameterNativeTypes::J(varint) => varint.to_cql_value(),
      ParameterNativeTypes::I(float) => float.to_cql_value(),
      ParameterNativeTypes::K(list) => list.to_cql_value(),
      ParameterNativeTypes::L(set) => set.to_cql_value(),
      ParameterNativeTypes::M(map) => map.to_cql_value(),
    }
  }
}
