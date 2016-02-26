/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

extern crate plist;

use self::plist::Plist;
use store;
use std::path::Path;
use ::AplibObject;
use ::AplibType;

/*
pub enum FolderType {
    INVALID = 0,
    FOLDER = 1,
    PROJECT = 2,
}
*/

pub struct Folder {
    uuid: String,
    parent_uuid: String,
    model_id: i64,

    pub folder_type: u64,
    pub db_version: i64,
    pub project_version: i64,
    pub path: String,
    pub name: String,
    pub implicit_album_uuid: String,
}

impl AplibObject for Folder {
    fn from_path(plist_path: &Path) -> Folder
    {
        use plutils::*;

        let plist = parse_plist(plist_path);
        return match plist {
            Plist::Dictionary(ref dict) => Folder {
                path: get_str_value(dict, "folderPath"),
                folder_type: get_int_value(dict, "folderType") as u64,
                model_id: get_int_value(dict, "modelId"),
                name: get_str_value(dict, "name"),
                parent_uuid: get_str_value(dict, "parentFolderUuid"),
                uuid: get_str_value(dict, "uuid"),
                implicit_album_uuid: get_str_value(dict, "implicitAlbumUuid"),
                db_version: get_int_value(dict, "version"),
                project_version: get_int_value(dict, "projectVersion")
            },
            _ => Folder {
                uuid: "".to_string(),
                model_id: 0, folder_type: 0,
                db_version: 0, project_version: 0,
                path: "".to_string(), name: "".to_string(),
                parent_uuid: "".to_string(),
                implicit_album_uuid: "".to_string()
            }
        }
    }
    fn obj_type(&self) -> AplibType {
        return AplibType::FOLDER;
    }
    fn uuid(&self) -> &String {
        return &self.uuid;
    }
    fn parent(&self) -> &String {
        return &self.parent_uuid;
    }
    fn model_id(&self) -> i64 {
        return self.model_id;
    }
    fn is_valid(&self) -> bool {
        return !self.uuid.is_empty();
    }
    fn wrap(obj: Folder) -> store::Wrapper {
        store::Wrapper::Folder(Box::new(obj))
    }
}

impl Folder {

}