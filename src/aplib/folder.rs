/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

extern crate plist;

use std::fs::File;

use self::plist::Plist;

pub enum FolderType {
    INVALID = 0,
    FOLDER = 1,
    PROJECT = 2,
}

pub struct Folder {
    pub uuid: String,
    pub model_id: i64,
    pub folder_type: u64,
    pub db_version: i64,
    pub project_version: i64,
    pub path: String,
    pub name: String,
    pub parent_uuid: String,
}

impl Folder {

    fn parse_plist(path : &str) -> Plist
    {
        let f = File::open(&path).unwrap();
        Plist::read(f).unwrap()
    }

    pub fn from(plist_path: &str) -> Folder
    {
        use aplib::plutils::{get_int_value,get_str_value};

        let plist = Folder::parse_plist(plist_path);
        match plist {
            Plist::Dictionary(ref dict) => Folder {
                path: get_str_value(dict, "folderPath"),
                folder_type: get_int_value(dict, "folderType") as u64,
                model_id: get_int_value(dict, "modelId"),
                name: get_str_value(dict, "name"),
                parent_uuid: get_str_value(dict, "parentFolderUuid"),
                uuid: get_str_value(dict, "uuid"),
                db_version: get_int_value(dict, "version"),
                project_version: get_int_value(dict, "projectVersion")
            },
            _ => Folder {
                uuid: "".to_string(),
                model_id: 0, folder_type: 0,
                db_version: 0, project_version: 0,
                path: "".to_string(), name: "".to_string(),
                parent_uuid: "".to_string()
            }
        }
    }

}
