/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */


use plist::Plist;
use std::path::Path;
use store;
use AplibObject;
use AplibType;

pub struct Version {
    uuid: Option<String>,
    model_id: Option<i64>,
    master_uuid: Option<String>,

    pub project_uuid: Option<String>,
    pub raw_master_uuid: Option<String>,
    pub nonraw_master_uuid: Option<String>,
    pub timezone_name: Option<String>,
    pub version_number: Option<i64>,
    pub db_version: Option<i64>,
    pub db_minor_version: Option<i64>,
    pub is_flagged: Option<bool>,
    pub is_original: Option<bool>,
    pub file_name: Option<String>,
    pub name: Option<String>,
    pub rating: Option<i64>,
}

impl AplibObject for Version {
    /// Load the version object from the plist at plist_path.
    fn from_path(plist_path: &Path) -> Option<Version>
    {
        use plutils::*;

        let plist = parse_plist(plist_path);
        match plist {
            Plist::Dictionary(ref dict) => Some(Version {
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
            }),
            _ =>
                None
        }
    }
    fn obj_type(&self) -> AplibType {
        AplibType::Version
    }
    fn uuid(&self) -> &Option<String> {
        &self.uuid
    }
    fn parent(&self) -> &Option<String> {
        &self.master_uuid
    }
    fn model_id(&self) -> i64 {
        self.model_id.unwrap_or(0)
    }
    fn is_valid(&self) -> bool {
        self.uuid.is_some()
    }
    fn wrap(obj: Version) -> store::Wrapper {
        store::Wrapper::Version(Box::new(obj))
    }
}
