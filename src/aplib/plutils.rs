extern crate plist;

use self::plist::Plist;
use std::collections::BTreeMap;

pub fn get_str_value(dict: &BTreeMap<String, Plist>, key: &str) -> String {
    match dict.get(key) {
        Some(&Plist::String(ref s)) => s.clone(),
        _ => "".to_string()
    }
}

pub fn get_int_value(dict: &BTreeMap<String, Plist>, key: &str) -> i64 {
    match dict.get(key) {
        Some(&Plist::Integer(n)) => n,
        _ => 0
    }
}

pub fn get_bool_value(dict: &BTreeMap<String, Plist>, key: &str) -> bool {
    match dict.get(key) {
        Some(&Plist::Boolean(b)) => b,
        _ => false
    }
}
