/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use std::path::Path;
use plist::Plist;
use store;
use AplibObject;
use AplibType;

pub enum Subclass {
    Invalid = 0,
    Normal = 1,
    Smart = 2,
}

/// Album object.
pub struct Album {
    /// uuid
    uuid: String,
    /// Folder uuid it represents.
    folder_uuid: String,
    model_id: i64,

    /// Subclass. See ``Subclass`` enum. (Normal, Smart)
    pub subclass: i64,
    /// ```Type```. Seems to always be 1
    pub album_type: i64,
    /// UUID of folder it is querying content of (smart only)
    pub query_folder_uuid: String,
    /// Version of db
    pub db_version: i64,
    /// Sort ascending
    pub sort_asc: bool,
    /// Date key for sort
    pub sort_key: String,
    /// Name of the album.
    pub name: String,
}

impl AplibObject for Album {
    fn from_path(plist_path: &Path) -> Option<Album>
    {
        use plutils::*;

        let plist = parse_plist(plist_path);
        match plist {
            Plist::Dictionary(ref dict) => {
                let dict = get_dict_value(dict, "InfoDictionary");
                Some(Album {
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
                })
            },
            _ =>
                None
        }
    }
    fn obj_type(&self) -> AplibType {
        return AplibType::Album;
    }
    fn uuid(&self) -> &String {
        return &self.uuid;
    }
    fn parent(&self) -> &String {
        return &self.folder_uuid;
    }
    fn model_id(&self) -> i64 {
        return self.model_id;
    }
    fn is_valid(&self) -> bool {
        return !self.uuid.is_empty();
    }
    fn wrap(obj: Album) -> store::Wrapper {
        store::Wrapper::Album(Box::new(obj))
    }
}

impl Album {
}
