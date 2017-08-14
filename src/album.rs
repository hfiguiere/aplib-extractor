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

#[derive(Debug, PartialEq)]
pub enum Subclass {
    Invalid,
    Implicit,
    Smart,
    User
}

impl Subclass {

    fn from(v: i64) -> Self {
        match v {
            0 => Subclass::Invalid,
            1 => Subclass::Implicit,
            2 => Subclass::Smart,
            3 => Subclass::User,
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
            Subclass::Implicit => 1,
            Subclass::Smart => 2,
            Subclass::User => 3,
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
    /// Content list
    pub content: Option<Vec<String>>,
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
                if let Some(info_dict) = get_dict_value(dict, "InfoDictionary") {
                    Some(Album {
                        uuid: get_str_value(&info_dict, "uuid"),
                        folder_uuid: get_str_value(&info_dict, "folderUuid"),
                        subclass: Subclass::from_option(get_int_value(&info_dict, "albumSubclass")),
                        album_type: get_int_value(&info_dict, "albumType"),
                        db_version: get_int_value(&info_dict, "version"),
                        model_id: get_int_value(&info_dict, "modelId"),
                        sort_asc: get_bool_value(&info_dict, "sortAscending"),
                        sort_key: get_str_value(&info_dict, "sortKeyPath"),
                        name: get_str_value(&info_dict, "name"),
                        query_folder_uuid: get_str_value(&info_dict, "queryFolderUuid"),
                        content: {
                            if let Some(array) = get_array_value(&dict, "versionUuids") {
                                let content: Vec<String>;
                                content = array.iter().filter_map(|elem|
                                    match *elem {
                                        Plist::String(ref s) =>
                                            Some(s.to_owned()),
                                        _ =>
                                            None
                                    }
                                ).collect();
                                Some(content)
                            } else {
                                None
                            }
                        },
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
    assert_eq!(*album.subclass.as_ref().unwrap(), Subclass::Implicit);
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

#[cfg(test)]
#[test]
fn test_album_content_parse() {
    use testutils;

    let album = Album::from_path(
        testutils::get_test_file_path("x6yNun58SB2sImfCarTJHA.apalbum")
            .as_path());
    assert!(album.is_some());
    let album = album.unwrap();

    assert_eq!(album.uuid.as_ref().unwrap(), "x6yNun58SB2sImfCarTJHA");
    assert_eq!(album.folder_uuid.as_ref().unwrap(), "TopLevelAlbums");
    assert_eq!(album.model_id.unwrap(), 181);
    assert_eq!(*album.subclass.as_ref().unwrap(), Subclass::User);
    assert_eq!(album.album_type.unwrap(), 1);
    assert!(album.query_folder_uuid.is_none());
    assert_eq!(album.db_version.unwrap(), 110);
    assert_eq!(album.sort_asc.unwrap(), true);
    assert_eq!(album.sort_key.as_ref().unwrap(), "exifProperties.ImageDate");
    assert_eq!(album.name.as_ref().unwrap(), "Flickr");
    assert!(album.content.is_some());
    let content = &album.content.as_ref().unwrap();
    assert_eq!(content.len(), 1);
    assert_eq!(content[0], "BF6nuoBnTumzoXyexdmXlw");

    let report = album.audit();
    // XXX fix when have actual audit.
    println!("report {:?}", report);
}
