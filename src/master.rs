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
    fn from_path(plist_path: &Path) -> Option<Master>
    {
        use plutils::*;
        let plist = parse_plist(plist_path);
        match plist {
            Plist::Dictionary(ref dict) => Some(Master {
                uuid: get_str_value(dict, "uuid"),
                alternate_master: get_str_value(dict, "alternateMasterUuid"),
                original_version_uuid: get_str_value(dict,
                                                     "originalVersionUuid"),
                project_uuid: get_str_value(dict, "projectUuid"),
                import_group_uuid: get_str_value(dict, "importGroupUuid"),
                filename: get_str_value(dict, "fileName"),
                name: get_str_value(dict, "name"),
                original_version_name: get_str_value(dict,
                                                     "originalVersionName"),
                db_version: get_int_value(dict, "version"),
                master_type: get_str_value(dict, "type"),
                subtype: get_str_value(dict, "subtype"),
                model_id: get_int_value(dict, "modelId"),
                image_path: get_str_value(dict, "imagePath"),
                is_reference: get_bool_value(dict, "fileIsReference"),
            }),
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
