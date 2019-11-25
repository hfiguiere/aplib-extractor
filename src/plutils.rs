/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use chrono::{DateTime, Utc};
pub use plist::Value;
use std::collections::BTreeMap;
use std::path::Path;
use std::time::SystemTime;

pub fn parse_plist<P>(path: P) -> Value
where
    P: AsRef<Path>,
{
    let result = Value::from_file(&path);
    match result {
        Ok(v) => v,
        Err(_) => {
            println!("Error from plist::read with file {:?}", path.as_ref());
            Value::Dictionary(BTreeMap::new())
        }
    }
}

pub fn get_str_value(dict: &BTreeMap<String, Value>, key: &str) -> Option<String> {
    match dict.get(key) {
        Some(&Value::String(ref s)) => Some(s.to_owned()),
        _ => None,
    }
}

pub fn get_int_value(dict: &BTreeMap<String, Value>, key: &str) -> Option<i64> {
    match dict.get(key) {
        Some(&Value::Integer(n)) => Some(n),
        _ => None,
    }
}

pub fn get_uint_value(dict: &BTreeMap<String, Value>, key: &str) -> Option<u64> {
    match dict.get(key) {
        Some(&Value::Integer(n)) => Some(n as u64),
        _ => None,
    }
}

pub fn get_bool_value(dict: &BTreeMap<String, Value>, key: &str) -> Option<bool> {
    match dict.get(key) {
        Some(&Value::Boolean(b)) => Some(b),
        _ => None,
    }
}

pub fn get_dict_value(
    dict: &BTreeMap<String, Value>,
    key: &str,
) -> Option<BTreeMap<String, Value>> {
    match dict.get(key) {
        Some(&Value::Dictionary(ref d)) => Some(d.to_owned()),
        _ => None,
    }
}

pub fn get_date_value(dict: &BTreeMap<String, Value>, key: &str) -> Option<DateTime<Utc>> {
    match dict.get(key) {
        Some(&Value::Date(ref d)) => {
            let t: SystemTime = d.clone().into();
            Some(t.into())
        }
        _ => None,
    }
}

pub fn get_data_value(dict: &BTreeMap<String, Value>, key: &str) -> Option<Vec<u8>> {
    match dict.get(key) {
        Some(&Value::Data(ref d)) => Some(d.clone()),
        _ => None,
    }
}

pub fn get_array_value(dict: &BTreeMap<String, Value>, key: &str) -> Option<Vec<Value>> {
    match dict.get(key) {
        Some(&Value::Array(ref a)) => Some(a.to_owned()),
        _ => None,
    }
}
