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
    audit_get_str_value, audit_get_int_value, audit_get_bool_value,
    Report
};

pub struct Master {
    uuid: Option<String>,
    model_id: Option<i64>,
    project_uuid: Option<String>,

    pub alternate_master: Option<String>,
    pub original_version_uuid: Option<String>,
    pub import_group_uuid: Option<String>,
    pub filename: Option<String>,
    pub name: Option<String>,
    pub original_version_name: Option<String>,
    pub db_version: Option<i64>,
    pub master_type: Option<String>,
    pub subtype: Option<String>,
    pub image_path: Option<String>,
    pub is_reference: Option<bool>,
}


impl AplibObject for Master {
    fn from_path(plist_path: &Path,
                 mut auditor: Option<&mut Report>) -> Option<Master> {

        use plutils::*;
        let plist = parse_plist(plist_path);
        match plist {
            Plist::Dictionary(ref dict) => {
                let result = Some(Master {
                    uuid: audit_get_str_value(dict, "uuid", &mut auditor),
                    alternate_master: audit_get_str_value(
                        dict, "alternateMasterUuid", &mut auditor),
                    original_version_uuid: audit_get_str_value(
                        dict, "originalVersionUuid", &mut auditor),
                    project_uuid: audit_get_str_value(
                        dict, "projectUuid", &mut auditor),
                    import_group_uuid: audit_get_str_value(
                        dict, "importGroupUuid", &mut auditor),
                    filename: audit_get_str_value(dict, "fileName", &mut auditor),
                    name: audit_get_str_value(dict, "name", &mut auditor),
                    original_version_name: audit_get_str_value(
                        dict, "originalVersionName", &mut auditor),
                    db_version: audit_get_int_value(dict, "version", &mut auditor),
                    master_type: audit_get_str_value(dict, "type", &mut auditor),
                    subtype: audit_get_str_value(dict, "subtype", &mut auditor),
                    model_id: audit_get_int_value(dict, "modelId", &mut auditor),
                    image_path: audit_get_str_value(
                        dict, "imagePath", &mut auditor),
                    is_reference: audit_get_bool_value(
                        dict, "fileIsReference", &mut auditor),
                });
                if auditor.is_some() {
                    let ref mut auditor = auditor.unwrap();
                    auditor.audit_ignored(dict, None);
                }
                result
            },
            _ =>
                None
        }
    }
    fn obj_type(&self) -> AplibType {
        AplibType::Master
    }
    fn uuid(&self) -> &Option<String> {
        &self.uuid
    }
    fn parent(&self) -> &Option<String> {
        &self.project_uuid
    }
    fn model_id(&self) -> i64 {
        self.model_id.unwrap_or(0)
    }
    fn is_valid(&self) -> bool {
        self.uuid.is_some()
    }
    fn wrap(obj: Master) -> store::Wrapper {
        store::Wrapper::Master(Box::new(obj))
    }
}

impl Master {

}
