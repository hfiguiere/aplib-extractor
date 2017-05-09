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

#[derive(Debug)]
pub enum Subclass {
    Invalid,
    Normal,
    Smart,
}

impl Subclass {

    fn from(v: i64) -> Self {
        match v {
            0 => Subclass::Invalid,
            1 => Subclass::Normal,
            2 => Subclass::Smart,
            _ => {
                println!("Unknown subclass value {}", v);
                Subclass::Invalid
            }
        }
    }

    fn from_option(o: Option<i64>) -> Option<Self> {
        if let Some(v) = o {
            Some(Self::from(v))
        } else {
            None
        }
    }

    pub fn to_int(v: &Self) -> i64 {
        match *v {
            Subclass::Invalid => 0,
            Subclass::Normal => 1,
            Subclass::Smart => 2
        }
    }
}

/// Album object.
#[derive(Debug)]
pub struct Album {
    /// uuid
    uuid: Option<String>,
    /// Folder uuid it represents.
    folder_uuid: Option<String>,
    model_id: Option<i64>,

    /// Subclass. See ``Subclass`` enum. (Normal, Smart)
    pub subclass: Option<Subclass>,
    /// ```Type```. Seems to always be 1
    pub album_type: Option<i64>,
    /// UUID of folder it is querying content of (smart only)
    pub query_folder_uuid: Option<String>,
    /// Version of db
    pub db_version: Option<i64>,
    /// Sort ascending
    pub sort_asc: Option<bool>,
    /// Date key for sort
    pub sort_key: Option<String>,
    /// Name of the album.
    pub name: Option<String>,
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
                if let Some(dict) = get_dict_value(dict, "InfoDictionary") {
                    Some(Album {
                        uuid: get_str_value(&dict, "uuid"),
                        folder_uuid: get_str_value(&dict, "folderUuid"),
                        subclass: Subclass::from_option(get_int_value(&dict, "albumSubclass")),
                        album_type: get_int_value(&dict, "albumType"),
                        db_version: get_int_value(&dict, "version"),
                        model_id: get_int_value(&dict, "modelId"),
                        sort_asc: get_bool_value(&dict, "sortAscending"),
                        sort_key: get_str_value(&dict, "sortKeyPath"),
                        name: get_str_value(&dict, "name"),
                        query_folder_uuid: get_str_value(&dict, "queryFolderUuid"),
                    })
                } else {
                    None
                }
            },
            _ =>
                None
        }
    }
    fn obj_type(&self) -> AplibType {
        AplibType::Album
    }
    fn uuid(&self) -> &Option<String> {
        &self.uuid
    }
    fn parent(&self) -> &Option<String> {
        &self.folder_uuid
    }
    fn model_id(&self) -> i64 {
        self.model_id.unwrap_or(0)
    }
    fn is_valid(&self) -> bool {
        self.uuid.is_some()
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

    assert_eq!(album.uuid.as_ref().unwrap(), "gOnttfpzQoOxcwLpFS9DQg");
    assert_eq!(album.folder_uuid.as_ref().unwrap(), "TopLevelAlbums");
    assert_eq!(album.model_id.unwrap(), 601);
    assert_eq!(album.subclass.unwrap(), 1);
    assert_eq!(album.album_type.unwrap(), 1);
    assert!(album.query_folder_uuid.is_none());
    assert_eq!(album.db_version.unwrap(), 110);
    assert_eq!(album.sort_asc.unwrap(), true);
    assert_eq!(album.sort_key.as_ref().unwrap(), "exifProperties.ImageDate");
    assert!(album.name.is_none());

    let report = album.audit();
    // XXX fix when have actual audit.
    println!("report {:?}", report);
}
