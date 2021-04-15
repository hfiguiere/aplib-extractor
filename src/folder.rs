/*
 This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

use std::path::Path;

use chrono::{DateTime, Utc};

use crate::audit::{
    audit_get_array_value, audit_get_bool_value, audit_get_date_value, audit_get_int_value,
    audit_get_str_value, Report, SkipReason,
};
use crate::notes::NotesProperties;
use crate::store;
use crate::AplibObject;
use crate::AplibType;
use crate::PlistLoadable;

/// Type of folder
#[derive(Debug, PartialEq)]
pub enum Type {
    Invalid = 0,
    /// Folder, aka container of things
    Folder = 1,
    /// Project (as in the UI), contains only `Master`s
    Project = 2,
}

impl Type {
    /// `Type` from an integer.
    fn from(v: i64) -> Self {
        match v {
            0 => Type::Invalid,
            1 => Type::Folder,
            2 => Type::Project,
            _ => {
                println!("Unknown folder type {}", v);
                Type::Invalid
            }
        }
    }

    /// `Type` from an optional integer.
    fn from_option(o: Option<i64>) -> Option<Self> {
        let v = o?;
        Some(Self::from(v))
    }

    /// `Type` to an integer.
    pub fn to_int(&self) -> i64 {
        match *self {
            Type::Invalid => 0,
            Type::Folder => 1,
            Type::Project => 2,
        }
    }
}

/// Folder object. This is a container of things in the library.
#[derive(Debug, Default)]
pub struct Folder {
    /// object uuid
    uuid: Option<String>,
    /// parent uuid
    parent_uuid: Option<String>,
    /// id in model
    model_id: Option<i64>,

    /// Folder type. (Project, Folder)
    pub folder_type: Option<Type>,
    /// Db model version
    pub db_version: Option<i64>,
    /// Project model version - expected 8
    pub project_version: Option<i64>,
    /// Path in the tree (using model_id for each components.
    pub path: Option<String>,
    /// Name
    pub name: Option<String>,
    /// UUID of the album object that compose this.
    pub implicit_album_uuid: Option<String>,
    /// index of the colour label
    pub colour_label_index: Option<i64>,
    pub create_date: Option<DateTime<Utc>>,
    pub sort_key_path: Option<String>,
    pub sort_ascending: Option<bool>,
    pub is_hidden: Option<bool>,
    pub is_magic: Option<bool>,
    pub is_favourite: Option<bool>,
    pub is_in_trash: Option<bool>,
    pub is_expanded: Option<bool>,
    pub is_hidden_when_empty: Option<bool>,
    pub poster_version_uuid: Option<String>,
    pub notes: Option<Vec<NotesProperties>>,
}

impl PlistLoadable for Folder {
    fn from_path<P>(plist_path: P, mut auditor: Option<&mut Report>) -> Option<Folder>
    where
        P: AsRef<Path>,
    {
        use crate::plutils::*;

        let plist = parse_plist(plist_path);
        match plist {
            Value::Dictionary(ref dict) => {
                let notes = audit_get_array_value(dict, "notes", &mut auditor);
                let result = Some(Folder {
                    path: audit_get_str_value(dict, "folderPath", &mut auditor),
                    folder_type: Type::from_option(audit_get_int_value(
                        dict,
                        "folderType",
                        &mut auditor,
                    )),
                    model_id: audit_get_int_value(dict, "modelId", &mut auditor),
                    name: audit_get_str_value(dict, "name", &mut auditor),
                    parent_uuid: audit_get_str_value(dict, "parentFolderUuid", &mut auditor),
                    uuid: audit_get_str_value(dict, "uuid", &mut auditor),
                    implicit_album_uuid: audit_get_str_value(
                        dict,
                        "implicitAlbumUuid",
                        &mut auditor,
                    ),
                    db_version: audit_get_int_value(dict, "version", &mut auditor),
                    project_version: audit_get_int_value(dict, "projectVersion", &mut auditor),
                    colour_label_index: audit_get_int_value(dict, "colorLabelIndex", &mut auditor),
                    create_date: audit_get_date_value(dict, "createDate", &mut auditor),
                    sort_key_path: audit_get_str_value(dict, "sortKeyPath", &mut auditor),
                    sort_ascending: audit_get_bool_value(dict, "sortAscending", &mut auditor),
                    is_hidden: audit_get_bool_value(dict, "isHidden", &mut auditor),
                    is_magic: audit_get_bool_value(dict, "isMagic", &mut auditor),
                    is_favourite: audit_get_bool_value(dict, "isFavorite", &mut auditor),
                    is_in_trash: audit_get_bool_value(dict, "isInTrash", &mut auditor),
                    is_expanded: audit_get_bool_value(dict, "isExpanded", &mut auditor),
                    is_hidden_when_empty: audit_get_bool_value(
                        dict,
                        "isHiddenWhenEmpty",
                        &mut auditor,
                    ),
                    poster_version_uuid: audit_get_str_value(
                        dict,
                        "posterVersionUuid",
                        &mut auditor,
                    ),
                    notes: NotesProperties::from(&notes, &mut auditor),
                });
                if let Some(auditor) = &mut auditor {
                    auditor.skip("CustomOrderList", SkipReason::Ignore);
                    auditor.skip("projectCompatibleBackToVersion", SkipReason::Ignore);
                    auditor.skip("automaticallyGenerateFullSizePreviews", SkipReason::Ignore);
                    auditor.audit_ignored(dict, None);
                }
                result
            }
            _ => None,
        }
    }
}

impl AplibObject for Folder {
    fn obj_type(&self) -> AplibType {
        AplibType::Folder
    }
    fn uuid(&self) -> &Option<String> {
        &self.uuid
    }
    fn parent(&self) -> &Option<String> {
        &self.parent_uuid
    }
    fn model_id(&self) -> i64 {
        self.model_id.unwrap_or(0)
    }
    fn is_valid(&self) -> bool {
        self.uuid.is_some()
    }
    fn wrap(obj: Folder) -> store::Wrapper {
        store::Wrapper::Folder(Box::new(obj))
    }
}

impl Folder {}

#[cfg(test)]
#[test]
fn test_folder_parse() {
    use crate::testutils;

    let folder = Folder::from_path(
        testutils::get_test_file_path("a%TX9lmjQVWvuK9u6RNhGQ.apfolder").as_path(),
        None,
    );
    assert!(folder.is_some());
    let folder = folder.unwrap();

    assert_eq!(folder.uuid.as_ref().unwrap(), "a%TX9lmjQVWvuK9u6RNhGQ");
    assert_eq!(folder.parent_uuid.as_ref().unwrap(), "AllProjectsItem");
    assert_eq!(folder.model_id.unwrap(), 333);
    assert_eq!(*folder.folder_type.as_ref().unwrap(), Type::Folder);
    assert_eq!(folder.db_version.unwrap(), 110);
    assert!(folder.project_version.is_none());
    assert_eq!(folder.path.as_ref().unwrap(), "1/3/333/");
    assert_eq!(folder.name.as_ref().unwrap(), "2011");
    assert_eq!(
        folder.implicit_album_uuid.as_ref().unwrap(),
        "J0+f3AmESPer4GHGv4BgAQ"
    );

    // XXX fix when have actual audit.
    //    println!("report {:?}", report);
}
