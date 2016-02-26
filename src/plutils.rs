/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

extern crate plist;

use std::fs::File;
use std::path::Path;
use self::plist::Plist;
use std::collections::BTreeMap;

pub fn parse_plist(path : &Path) -> Plist
{
    let f = File::open(&path).unwrap();
    let result = Plist::read(f);
    return match result {
        Ok(v) => v,
        Err(_) => {
            println!("Error from plist::read with file {:?}", path);
            Plist::Dictionary(BTreeMap::new())
        }
    };
}

pub fn get_str_value(dict: &BTreeMap<String, Plist>, key: &str) -> String {
    return match dict.get(key) {
        Some(&Plist::String(ref s)) => s.to_owned(),
        _ => "".to_string()
    };
}

pub fn get_int_value(dict: &BTreeMap<String, Plist>, key: &str) -> i64 {
    return match dict.get(key) {
        Some(&Plist::Integer(n)) => n,
        _ => 0
    };
}

pub fn get_bool_value(dict: &BTreeMap<String, Plist>, key: &str) -> bool {
    return match dict.get(key) {
        Some(&Plist::Boolean(b)) => b,
        _ => false
    };
}

pub fn get_dict_value(dict: &BTreeMap<String, Plist>,
                      key: &str) -> BTreeMap<String, Plist> {
    return match dict.get(key) {
        Some(&Plist::Dictionary(ref d)) => d.to_owned(),
        _ => BTreeMap::new()
    };
}

pub fn get_array_value(dict: &BTreeMap<String, Plist>,
                       key: &str) -> Vec<Plist> {
    return match dict.get(key) {
        Some(&Plist::Array(ref a)) => a.to_owned(),
        _ => Vec::new()
    };
}
