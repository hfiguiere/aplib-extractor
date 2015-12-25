/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

extern crate plist;

use self::plist::Plist;
use std::path::Path;

pub struct Version {
    pub uuid: String,
    pub master_uuid: String,
    pub project_uuid: String,
    raw_master_uuid: String,
    nonraw_master_uuid: String,
    timezone_name: String,
    version_number: i64,
    db_version: i64,
    db_minor_version: i64,
    is_flagged: bool,
    pub is_original: bool,
    file_name: String,
    pub name: String,
    model_id: i64,
    rating: i64,
}

impl Version {

    pub fn from(plist_path: &Path) -> Version
    {
        use aplib::plutils::*;

        let plist = parse_plist(plist_path);
        return match plist {
            Plist::Dictionary(ref dict) => Version {
                uuid: get_str_value(dict, "uuid"),
                master_uuid: get_str_value(dict, "masterUuid"),
                project_uuid: get_str_value(dict, "projectUuid"),
                raw_master_uuid: get_str_value(dict, "rawMasterUuid"),
                nonraw_master_uuid: get_str_value(dict, "nonRawMasterUuid"),
                timezone_name: get_str_value(dict, "imageTimeZoneName"),
                version_number: get_int_value(dict, "versionNumber"),
                db_version: get_int_value(dict, "version"),
                db_minor_version: get_int_value(dict, "minorVersion"),
                is_flagged: get_bool_value(dict, "isFlagged"),
                is_original: get_bool_value(dict, "isOriginal"),
                file_name: get_str_value(dict, "fileName"),
                name: get_str_value(dict, "name"),
                model_id: get_int_value(dict, "modelId"),
                rating: get_int_value(dict, "mainRating"),
            },
            _ => Version {
                uuid: "".to_string(),
                master_uuid: "".to_string(),
                project_uuid: "".to_string(),
                raw_master_uuid: "".to_string(),
                nonraw_master_uuid: "".to_string(),
                timezone_name: "".to_string(),
                version_number: 0,
                db_version: 0,
                db_minor_version: 0,
                is_flagged: false,
                is_original: false,
                file_name: "".to_string(),
                name: "".to_string(),
                model_id: 0,
                rating: 0,

            }
        }
    }

    pub fn is_valid(&self) -> bool {
        return !self.uuid.is_empty();
    }
}
