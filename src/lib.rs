/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

extern crate plist;
extern crate chrono;

mod album;
pub mod audit;
mod custominfo;
mod exif;
mod folder;
mod iptc;
mod keyword;
mod library;
mod master;
mod notes;
pub mod plutils;
mod store;
mod version;

#[cfg(test)]
mod testutils;

use std::path::Path;

pub use library::Library as Library;
pub use library::ModelInfo as ModelInfo;
pub use folder::Folder as Folder;
pub use folder::Type as FolderType;
pub use album::Album as Album;
pub use album::Subclass as AlbumSubclass;
pub use master::Master as Master;
pub use version::Version as Version;
pub use keyword::Keyword as Keyword;
pub use store::Wrapper as StoreWrapper;
use audit::Report;

pub enum AplibType {
    Album,
    Folder,
    Keyword,
    Master,
    Version,
}

pub trait AplibObject {
    fn from_path(plist_path: &Path,
                 auditor: Option<&mut Report>) -> Option<Self>
        where Self: Sized;
    fn wrap(obj: Self) -> store::Wrapper;
    fn obj_type(&self) -> AplibType;
    fn is_valid(&self) -> bool;
    fn uuid(&self) -> &Option<String>;
    fn parent(&self) -> &Option<String>;
    fn model_id(&self) -> i64;
}

