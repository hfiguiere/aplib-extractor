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
use audit::{Auditable,Report};

pub enum Subclass {
    Invalid = 0,
    Normal = 1,
    Smart = 2,
}

/// Album object.
#[derive(Debug)]
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

impl Auditable for Album {
    fn audit(&self) -> Report {
        Report::new()
    }
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


#[cfg(test)]
#[test]
fn test_album_parse() {
    use testutils;

    let album = Album::from_path(
        testutils::get_test_file_path("gOnttfpzQoOxcwLpFS9DQg.apalbum")
            .as_path());
    assert!(album.is_some());
    let album = album.unwrap();

    assert_eq!(album.uuid, "gOnttfpzQoOxcwLpFS9DQg");
    assert_eq!(album.folder_uuid, "TopLevelAlbums");
    assert_eq!(album.model_id, 601);
    assert_eq!(album.subclass, 1);
    assert_eq!(album.album_type, 1);
    assert_eq!(album.query_folder_uuid, "");
    assert_eq!(album.db_version, 110);
    assert_eq!(album.sort_asc, true);
    assert_eq!(album.sort_key, "exifProperties.ImageDate");
    assert_eq!(album.name, "");

    let report = album.audit();
    // XXX fix when have actual audit.
    println!("report {:?}", report);
}
