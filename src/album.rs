/*
 This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

use std::path::Path;

use chrono::{DateTime, Utc};
use num_traits::FromPrimitive;

use crate::audit::{
    audit_get_bool_value, audit_get_date_value, audit_get_int_value, audit_get_str_value, Report,
    SkipReason,
};
use crate::plutils::{get_array_value, Value};
use crate::store;
use crate::AplibObject;
use crate::AplibType;
use crate::PlistLoadable;

#[derive(
    Clone, Copy, Debug, Default, num_derive::FromPrimitive, num_derive::ToPrimitive, PartialEq,
)]
#[repr(u32)]
/// Subclass for album
pub enum Subclass {
    #[default]
    /// Invalid.
    Invalid = 0,
    /// Implicit - used for folders
    Implicit = 1,
    /// Smart -
    Smart = 2,
    /// User - user album with explicit content.
    User = 3,
}

/// Album object.
#[derive(Debug, Default)]
pub struct Album {
    /// uuid
    uuid: Option<String>,
    /// Folder uuid it represents.
    folder_uuid: Option<String>,
    model_id: Option<i64>,

    /// Subclass. See ``Subclass`` enum. (Implicit, Smart, User)
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
    pub custom_sort_available: Option<bool>,
    pub colour_label_index: Option<i64>,
    pub create_date: Option<DateTime<Utc>>,
    pub is_hidden: Option<bool>,
    pub is_magic: Option<bool>,
    pub is_favourite: Option<bool>,
    pub is_in_trash: Option<bool>,
    pub selected_track_path_uuid: Option<String>,
    /// Content list - for `User` subclass
    pub content: Option<Vec<String>>,
}

impl PlistLoadable for Album {
    fn from_path<P>(plist_path: P, mut auditor: Option<&mut Report>) -> Option<Album>
    where
        P: AsRef<Path>,
    {
        use crate::plutils::*;

        let plist = parse_plist(plist_path);
        match plist {
            Value::Dictionary(ref dict) => {
                let info_dict = get_dict_value(dict, "InfoDictionary")?;
                let subclass = audit_get_int_value(&info_dict, "albumSubclass", &mut auditor)
                    .and_then(Subclass::from_i64);
                let result = Some(Album {
                    uuid: audit_get_str_value(&info_dict, "uuid", &mut auditor),
                    folder_uuid: audit_get_str_value(&info_dict, "folderUuid", &mut auditor),
                    subclass,
                    album_type: audit_get_int_value(&info_dict, "albumType", &mut auditor),
                    db_version: audit_get_int_value(&info_dict, "version", &mut auditor),
                    model_id: audit_get_int_value(&info_dict, "modelId", &mut auditor),
                    sort_asc: audit_get_bool_value(&info_dict, "sortAscending", &mut auditor),
                    sort_key: audit_get_str_value(&info_dict, "sortKeyPath", &mut auditor),
                    name: audit_get_str_value(&info_dict, "name", &mut auditor),
                    query_folder_uuid: audit_get_str_value(
                        &info_dict,
                        "queryFolderUuid",
                        &mut auditor,
                    ),
                    create_date: audit_get_date_value(&info_dict, "createDate", &mut auditor),
                    colour_label_index: audit_get_int_value(
                        &info_dict,
                        "colorLabelIndex",
                        &mut auditor,
                    ),
                    custom_sort_available: audit_get_bool_value(
                        &info_dict,
                        "customSortAvailable",
                        &mut auditor,
                    ),
                    is_hidden: audit_get_bool_value(&info_dict, "isHidden", &mut auditor),
                    is_magic: audit_get_bool_value(&info_dict, "isMagic", &mut auditor),
                    is_favourite: audit_get_bool_value(&info_dict, "isFavorite", &mut auditor),
                    is_in_trash: audit_get_bool_value(&info_dict, "isInTrash", &mut auditor),
                    selected_track_path_uuid: audit_get_str_value(
                        &info_dict,
                        "selectedTrackPathUuid",
                        &mut auditor,
                    ),
                    content: Album::content_from(dict, &subclass, &mut auditor),
                });
                if let Some(ref mut auditor) = auditor {
                    auditor.audit_ignored(&info_dict, None);
                }
                result
            }
            _ => None,
        }
    }
}

impl AplibObject for Album {
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
    /// Load album content. `dict` should contain the "versionUuids" key.
    /// and the subclass should be `Subclass::User`.
    fn content_from(
        dict: &plist::Dictionary,
        subclass: &Option<Subclass>,
        auditor: &mut Option<&mut Report>,
    ) -> Option<Vec<String>> {
        let array = get_array_value(dict, "versionUuids")?;
        if *subclass == Some(Subclass::User) {
            let content: Vec<String> = array
                .iter()
                .filter_map(|elem| match *elem {
                    Value::String(ref s) => Some(s.to_owned()),
                    _ => None,
                })
                .collect();
            if let Some(ref mut report) = *auditor {
                report.parsed("versionUuids");
            }
            Some(content)
        } else {
            if let Some(ref mut report) = *auditor {
                report.skip("versionUuids", SkipReason::InvalidData);
            }
            None
        }
    }
}

#[cfg(test)]
#[test]
fn test_album_parse() {
    use crate::testutils;

    let album = Album::from_path(
        testutils::get_test_file_path("gOnttfpzQoOxcwLpFS9DQg.apalbum").as_path(),
        None,
    );
    assert!(album.is_some());
    let album = album.unwrap();

    assert_eq!(album.uuid.as_ref().unwrap(), "gOnttfpzQoOxcwLpFS9DQg");
    assert_eq!(album.folder_uuid.as_ref().unwrap(), "TopLevelAlbums");
    assert_eq!(album.model_id.unwrap(), 601);
    assert_eq!(*album.subclass.as_ref().unwrap(), Subclass::Implicit);
    assert_eq!(album.album_type.unwrap(), 1);
    assert!(album.query_folder_uuid.is_none());
    assert_eq!(album.db_version.unwrap(), 110);
    assert!(album.sort_asc.unwrap());
    assert_eq!(album.sort_key.as_ref().unwrap(), "exifProperties.ImageDate");
    assert!(album.name.is_none());

    //    let report = album.audit();
    // XXX fix when have actual audit.
    //    println!("report {:?}", report);
}

#[cfg(test)]
#[test]
fn test_album_content_parse() {
    use crate::testutils;

    let album = Album::from_path(
        testutils::get_test_file_path("x6yNun58SB2sImfCarTJHA.apalbum").as_path(),
        None,
    );
    assert!(album.is_some());
    let album = album.unwrap();

    assert_eq!(album.uuid.as_ref().unwrap(), "x6yNun58SB2sImfCarTJHA");
    assert_eq!(album.folder_uuid.as_ref().unwrap(), "TopLevelAlbums");
    assert_eq!(album.model_id.unwrap(), 181);
    assert_eq!(*album.subclass.as_ref().unwrap(), Subclass::User);
    assert_eq!(album.album_type.unwrap(), 1);
    assert!(album.query_folder_uuid.is_none());
    assert_eq!(album.db_version.unwrap(), 110);
    assert!(album.sort_asc.unwrap());
    assert_eq!(album.sort_key.as_ref().unwrap(), "exifProperties.ImageDate");
    assert_eq!(album.name.as_ref().unwrap(), "Flickr");
    assert!(album.content.is_some());
    let content = &album.content.as_ref().unwrap();
    assert_eq!(content.len(), 1);
    assert_eq!(content[0], "BF6nuoBnTumzoXyexdmXlw");

    //    let report = album.audit();
    // XXX fix when have actual audit.
    //    println!("report {:?}", report);
}
