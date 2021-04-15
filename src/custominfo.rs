/*
 This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

use audit::{audit_get_str_value, Report};

pub struct CustomInfoProperties {
    pub camera_time_zone_name: Option<String>,
    pub picture_time_zone_name: Option<String>,
}

impl CustomInfoProperties {
    pub fn from(
        dict: &Option<plist::Dictionary>,
        mut auditor: &mut Option<&mut Report>,
    ) -> Option<CustomInfoProperties> {
        if let Some(dict) = dict.as_ref() {
            let result = Some(CustomInfoProperties {
                camera_time_zone_name: audit_get_str_value(
                    dict,
                    "cameraTimeZoneName",
                    &mut auditor,
                ),
                picture_time_zone_name: audit_get_str_value(
                    dict,
                    "pictureTimeZoneName",
                    &mut auditor,
                ),
            });
            if let Some(auditor) = &mut auditor.as_mut() {
                auditor.audit_ignored(dict, Some("customInfo"));
            }
            result
        } else {
            None
        }
    }
}
