/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use std::fs::File;
use std::path::Path;
use chrono::{DateTime, Utc};
pub use plist::Plist;
use std::collections::BTreeMap;

pub fn parse_plist(path: &Path) -> Plist {
    let f = File::open(&path).unwrap();
    let result = Plist::read(f);
    match result {
        Ok(v) => v,
        Err(_) => {
            println!("Error from plist::read with file {:?}", path);
            Plist::Dictionary(BTreeMap::new())
        }
    }
}

pub fn get_str_value(dict: &BTreeMap<String, Plist>, key: &str) -> Option<String> {
    match dict.get(key) {
        Some(&Plist::String(ref s)) => Some(s.to_owned()),
        _ => None,
    }
}

pub fn get_int_value(dict: &BTreeMap<String, Plist>, key: &str) -> Option<i64> {
    match dict.get(key) {
        Some(&Plist::Integer(n)) => Some(n),
        _ => None,
    }
}

pub fn get_uint_value(dict: &BTreeMap<String, Plist>, key: &str) -> Option<u64> {
    match dict.get(key) {
        Some(&Plist::Integer(n)) => Some(n as u64),
        _ => None,
    }
}

pub fn get_bool_value(dict: &BTreeMap<String, Plist>, key: &str) -> Option<bool> {
    match dict.get(key) {
        Some(&Plist::Boolean(b)) => Some(b),
        _ => None,
    }
}

pub fn get_dict_value(
    dict: &BTreeMap<String, Plist>,
    key: &str,
) -> Option<BTreeMap<String, Plist>> {
    match dict.get(key) {
        Some(&Plist::Dictionary(ref d)) => Some(d.to_owned()),
        _ => None,
    }
}

pub fn get_date_value(dict: &BTreeMap<String, Plist>, key: &str) -> Option<DateTime<Utc>> {
    match dict.get(key) {
        Some(&Plist::Date(ref d)) => Some(d.clone().into()),
        _ => None,
    }
}

pub fn get_data_value(dict: &BTreeMap<String, Plist>, key: &str) -> Option<Vec<u8>> {
    match dict.get(key) {
        Some(&Plist::Data(ref d)) => Some(d.clone()),
        _ => None,
    }
}

pub fn get_array_value(dict: &BTreeMap<String, Plist>, key: &str) -> Option<Vec<Plist>> {
    match dict.get(key) {
        Some(&Plist::Array(ref a)) => Some(a.to_owned()),
        _ => None,
    }
}
