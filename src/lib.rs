/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

extern crate plist;
extern crate chrono;
#[macro_use]
extern crate try_opt;

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


/// `AplibObject` types.
pub enum AplibType {
    /// Album
    Album,
    /// Folder (or Project
    Folder,
    /// Keyword
    Keyword,
    /// Master image
    Master,
    /// Version
    Version,
}

/// Basic trait from the library objects.
pub trait AplibObject {
    /// Load object from plist `plist_path`
    fn from_path(plist_path: &Path,
                 auditor: Option<&mut Report>) -> Option<Self>
        where Self: Sized;
    /// Wrap it for storage
    fn wrap(obj: Self) -> store::Wrapper;
    /// Type of object.
    fn obj_type(&self) -> AplibType;
    /// Is object valid
    fn is_valid(&self) -> bool;
    /// Object uuid
    fn uuid(&self) -> &Option<String>;
    /// uuid of parent object
    fn parent(&self) -> &Option<String>;
    /// Model id (numerical id)
    fn model_id(&self) -> i64;
}

