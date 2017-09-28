/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use std::collections::BTreeMap;
use plist::Plist;
use audit::{
    Report,
    audit_get_str_value
};

pub struct CustomInfoProperties {
    pub camera_time_zone_name: Option<String>,
    pub picture_time_zone_name: Option<String>,
}

impl CustomInfoProperties {
    pub fn from(dict: &Option<BTreeMap<String, Plist>>,
                mut auditor: &mut Option<&mut Report>) -> Option<CustomInfoProperties> {
       if dict.is_none() {
            return None;
        }
        let dict = dict.as_ref().unwrap();

        let result = Some(CustomInfoProperties{
            camera_time_zone_name: audit_get_str_value(dict, "cameraTimeZoneName", &mut auditor),
            picture_time_zone_name: audit_get_str_value(dict, "pictureTimeZoneName", &mut auditor),
        });
        if auditor.is_some() {
            let ref mut auditor = auditor.as_mut().unwrap();
            auditor.audit_ignored(dict, Some("customInfo"));
        }
        result
    }
}