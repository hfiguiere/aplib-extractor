/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

extern crate plist;

use self::plist::Plist;

pub enum AlbumSubclass {
    INVALID = 0,
    IMPLICIT = 1,
    EXPLICIT = 2,
}

pub struct Album {
    pub subclass: i64,
    pub album_type: i64,
    pub folder_uuid: String,
    pub query_folder_uuid: String,
    pub uuid: String,
    pub db_version: i64,
    pub model_id: i64,
    pub sort_asc: bool,
    pub sort_key: String,
    pub name: String,
}

impl Album {

    pub fn from(plist_path: &str) -> Album
    {
        use aplib::plutils::*;

        let plist = parse_plist(plist_path);
        return match plist {
            Plist::Dictionary(ref dict) => {
                let dict = get_dict_value(dict, "InfoDictionary");
                Album {
                    uuid: get_str_value(&dict, "uuid"),
                    folder_uuid: get_str_value(&dict, "folderUuid"),
                    subclass: get_int_value(&dict, "albumSubclass"),
                    album_type: get_int_value(&dict, "albumType"),
                    db_version: get_int_value(&dict, "version"),
                    model_id: get_int_value(&dict, "modelId"),
                    sort_asc: get_bool_value(&dict, "sortAscending"),
                    sort_key: get_str_value(&dict, "sortKeyPath"),
                    name: get_str_value(&dict, "name"),
                    query_folder_uuid: get_str_value(&dict, "queryFolderUuid"),
                }
            },
            _ => Album {
                name: "".to_string(),
                uuid: "".to_string(),
                folder_uuid: "".to_string(),
                query_folder_uuid: "".to_string(),
                subclass: 0,
                album_type: 0,
                db_version: 0,
                model_id: 0,
                sort_asc: true,
                sort_key: "".to_string()
            }
        }
    }

    pub fn is_valid(&self) -> bool {
        return !self.uuid.is_empty();
    }

}
