/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use store;
use std::path::Path;
use AplibObject;
use AplibType;
use audit::{Report,audit_get_int_value,audit_get_str_value};

/// Type of folder: folder or project
/// Only these are known.
#[derive(Debug, PartialEq)]
pub enum Type {
    Invalid = 0,
    Folder = 1,
    Project = 2,
}

impl Type {
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

    fn from_option(o: Option<i64>) -> Option<Self> {
        if let Some(v) = o {
            Some(Self::from(v))
        } else {
            None
        }
    }

    pub fn to_int(v: &Self) -> i64 {
        match *v {
            Type::Invalid => 0,
            Type::Folder => 1,
            Type::Project => 2,
        }
    }
}


/// Folder object. This is a container of things in the library.
#[derive(Debug,Default)]
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
}

impl AplibObject for Folder {
    fn from_path(plist_path: &Path,
                 mut auditor: Option<&mut Report>) -> Option<Folder> {

        use plutils::*;

        let plist = parse_plist(plist_path);
        match plist {
            Plist::Dictionary(ref dict) => {
                let result = Some(Folder {
                    path: audit_get_str_value(dict, "folderPath", &mut auditor),
                    folder_type: Type::from_option(audit_get_int_value(dict, "folderType", &mut auditor)),
                    model_id: audit_get_int_value(
                        dict, "modelId", &mut auditor),
                    name: audit_get_str_value(dict, "name", &mut auditor),
                    parent_uuid: audit_get_str_value(
                        dict, "parentFolderUuid", &mut auditor),
                    uuid: audit_get_str_value(dict, "uuid", &mut auditor),
                    implicit_album_uuid: audit_get_str_value(
                        dict, "implicitAlbumUuid", &mut auditor),
                    db_version: audit_get_int_value(
                        dict, "version", &mut auditor),
                    project_version: audit_get_int_value(
                        dict, "projectVersion", &mut auditor)
                });
                if auditor.is_some() {
                    let ref mut auditor = auditor.unwrap();
                    auditor.audit_ignored(dict);
                }
                result
            },
            _ =>
                None
        }
    }
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

impl Folder {

}


#[cfg(test)]

#[test]
fn test_folder_parse() {
    use testutils;

    let folder = Folder::from_path(
        testutils::get_test_file_path("a%TX9lmjQVWvuK9u6RNhGQ.apfolder")
            .as_path(), None);
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
    assert_eq!(folder.implicit_album_uuid.as_ref().unwrap(), "J0+f3AmESPer4GHGv4BgAQ");

    // XXX fix when have actual audit.
//    println!("report {:?}", report);
}
