/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use plist::Plist;
use store;
use std::path::Path;
use AplibObject;
use AplibType;
use audit::{Auditable,Report};

/// Type of folder: folder or project
/// Only these are known.
pub enum Type {
    Invalid = 0,
    Folder = 1,
    Project = 2,
}


/// Folder object. This is a container of things in the library.
pub struct Folder {
    /// object uuid
    uuid: String,
    /// parent uuid
    parent_uuid: String,
    /// id in model
    model_id: i64,

    /// Folder type. (Project, Folder)
    pub folder_type: u64,
    /// Db model version
    pub db_version: i64,
    /// Project model version - expected 8
    pub project_version: i64,
    /// Path in the tree (using model_id for each components.
    pub path: String,
    /// Name
    pub name: String,
    /// UUID of the album object that compose this.
    pub implicit_album_uuid: String,
}

impl Auditable for Folder {
    fn audit(&self) -> Report {
        Report::new()
    }
}

impl AplibObject for Folder {
    fn from_path(plist_path: &Path) -> Option<Folder>
    {
        use plutils::*;

        let plist = parse_plist(plist_path);
        match plist {
            Plist::Dictionary(ref dict) => Some(Folder {
                path: get_str_value(dict, "folderPath"),
                folder_type: get_int_value(dict, "folderType") as u64,
                model_id: get_int_value(dict, "modelId"),
                name: get_str_value(dict, "name"),
                parent_uuid: get_str_value(dict, "parentFolderUuid"),
                uuid: get_str_value(dict, "uuid"),
                implicit_album_uuid: get_str_value(dict, "implicitAlbumUuid"),
                db_version: get_int_value(dict, "version"),
                project_version: get_int_value(dict, "projectVersion")
            }),
            _ =>
                None
        }
    }
    fn obj_type(&self) -> AplibType {
        return AplibType::Folder;
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
