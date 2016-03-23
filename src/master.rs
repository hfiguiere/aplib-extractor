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
    uuid: String,
    model_id: i64,
    project_uuid: String,

    pub alternate_master: String,
    pub original_version_uuid: String,
    pub import_group_uuid: String,
    pub filename: String,
    pub name: String,
    pub original_version_name: String,
    pub db_version: i64,
    pub master_type: String,
    pub subtype: String,
    pub image_path: String,
    pub is_reference: bool,
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
        return AplibType::Master;
    }
    fn uuid(&self) -> &String {
        return &self.uuid;
    }
    fn parent(&self) -> &String {
        return &self.project_uuid;
    }
    fn model_id(&self) -> i64 {
        return self.model_id;
    }
    fn is_valid(&self) -> bool {
        return !self.uuid.is_empty();
    }
    fn wrap(obj: Master) -> store::Wrapper {
        store::Wrapper::Master(Box::new(obj))
    }
}

impl Master {

}
