/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */


use std::path::Path;
use store;
use AplibObject;
use AplibType;
use audit::{
    audit_get_str_value,
    audit_get_int_value,
    audit_get_bool_value,
    Report
};

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
    pub is_editable: Option<bool>,
    pub is_hidden: Option<bool>,
    pub is_in_trash: Option<bool>,
    pub file_name: Option<String>,
    pub name: Option<String>,
    pub rating: Option<i64>,
}

impl AplibObject for Version {
    /// Load the version object from the plist at plist_path.
    fn from_path(plist_path: &Path,
                 mut auditor: Option<&mut Report>) -> Option<Version> {

        use plutils::*;

        let plist = parse_plist(plist_path);
        match plist {
            Plist::Dictionary(ref dict) => {
                let result = Some(Version {
                    uuid: audit_get_str_value(dict, "uuid", &mut auditor),
                    master_uuid: audit_get_str_value(
                        dict, "masterUuid", &mut auditor),
                    project_uuid: audit_get_str_value(
                        dict, "projectUuid", &mut auditor),
                    raw_master_uuid: audit_get_str_value(
                        dict, "rawMasterUuid", &mut auditor),
                    nonraw_master_uuid: audit_get_str_value(
                        dict, "nonRawMasterUuid", &mut auditor),
                    timezone_name: audit_get_str_value(
                        dict, "imageTimeZoneName", &mut auditor),
                    version_number: audit_get_int_value(
                        dict, "versionNumber", &mut auditor),
                    db_version: audit_get_int_value(
                        dict, "version", &mut auditor),
                    db_minor_version: audit_get_int_value(
                        dict, "minorVersion", &mut auditor),
                    is_flagged: audit_get_bool_value(
                        dict, "isFlagged", &mut auditor),
                    is_original: audit_get_bool_value(
                        dict, "isOriginal", &mut auditor),
                    is_editable: audit_get_bool_value(
                        dict, "isEditable", &mut auditor),
                    is_hidden: audit_get_bool_value(
                        dict, "isHidden", &mut auditor),
                    is_in_trash: audit_get_bool_value(
                        dict, "isInTrash", &mut auditor),
                    file_name: audit_get_str_value(
                        dict, "fileName", &mut auditor),
                    name: audit_get_str_value(dict, "name", &mut auditor),
                    model_id: audit_get_int_value(
                        dict, "modelId", &mut auditor),
                    rating: audit_get_int_value(
                        dict, "mainRating", &mut auditor),
                });
                if auditor.is_some() {
                    let ref mut auditor = auditor.unwrap();
                    auditor.audit_ignored(dict);
                }
                result
            },
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
