/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use std::collections::BTreeMap;
use plist::Plist;
use audit::{
    SkipReason,
    Report
};

#[derive(PartialEq)]
pub enum IptcValue {
    None,
    Str(String)
}

pub struct IptcProperties {
    bag: BTreeMap<String, IptcValue>,
}

impl IptcProperties {

    pub fn from(dict: &Option<BTreeMap<String, Plist>>,
                mut auditor: &mut Option<&mut Report>) -> Option<IptcProperties> {
        if dict.is_none() {
            return None;
        }
        let dict = dict.as_ref().unwrap();
        let mut values: BTreeMap<String, IptcValue> = BTreeMap::new();
        for (key, value) in dict {
            let iv = match value {
                &Plist::String(ref s) => IptcValue::Str(s.to_owned()),
                _ => IptcValue::None,
            };
            if iv != IptcValue::None {
                values.insert(key.to_owned(), iv);
            } else if let Some(ref mut r) = *auditor {
                r.skip(&format!("Iptc.{}", key), SkipReason::InvalidType);
            }
        }
        Some(IptcProperties{bag: values})
    }

}