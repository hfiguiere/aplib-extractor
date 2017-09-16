/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use std::collections::BTreeMap;
use plist::Plist;
use audit::{
    audit_get_str_value,
    Report
};

pub struct ExifProperties {

}

impl ExifProperties {
    pub fn from(dict: &Option<BTreeMap<String, Plist>>,
                mut auditor: &mut Option<&mut Report>) -> Option<ExifProperties> {
       if dict.is_none() {
            return None;
        }
        let dict = dict.as_ref().unwrap();
        let result = Some(ExifProperties{
        });
        if let Some(ref mut r) = *auditor {
            r.audit_ignored(&dict, Some("Exif"));
        }
        result
    }
}