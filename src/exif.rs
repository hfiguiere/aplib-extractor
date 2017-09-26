/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use std::collections::BTreeMap;
use chrono::{DateTime,Utc};
use plist::Plist;
use audit::{
    SkipReason,
    Report
};

#[derive(PartialEq)]
pub enum ExifValue {
    None,
    Int(i64),
    Str(String),
    Date(DateTime<Utc>),
    Real(f64),
}

pub struct ExifProperties {
    bag: BTreeMap<String, ExifValue>,
}

impl ExifProperties {
    pub fn from(dict: &Option<BTreeMap<String, Plist>>,
                mut auditor: &mut Option<&mut Report>) -> Option<ExifProperties> {
       if dict.is_none() {
            return None;
        }
        let dict = dict.as_ref().unwrap();
        let mut values: BTreeMap<String,ExifValue> = BTreeMap::new();
        for (key, value) in dict {
            let ev = match value {
                &Plist::Integer(n) => ExifValue::Int(n),
                &Plist::Real(f) => ExifValue::Real(f),
                &Plist::String(ref s) => ExifValue::Str(s.to_owned()),
                &Plist::Date(ref d) => ExifValue::Date(d.clone().into()),
                _ => ExifValue::None,
            };
            if ev != ExifValue::None {
                values.insert(key.to_owned(), ev);
            } else if let Some(ref mut r) = *auditor {
                r.skip(&format!("Exif.{}", key), SkipReason::InvalidType);
            }
        }
        Some(ExifProperties{bag: values})
    }
}