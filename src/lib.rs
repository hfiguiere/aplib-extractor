/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

extern crate plist;

pub mod plutils;
mod library;
mod folder;
mod album;
mod master;
mod version;
mod keyword;
mod store;

use std::path::Path;

pub use library::Library as Library;
pub use folder::Folder as Folder;
pub use album::Album as Album;
pub use master::Master as Master;
pub use version::Version as Version;
pub use keyword::Keyword as Keyword;
pub use store::Wrapper as StoreWrapper;

pub enum AplibType {
    Album,
    Folder,
    Keyword,
    Master,
    Version,
}

pub trait AplibObject {
    fn from_path(plist_path: &Path) -> Option<Self>
        where Self: Sized;
    fn wrap(obj: Self) -> store::Wrapper;
    fn obj_type(&self) -> AplibType;
    fn is_valid(&self) -> bool;
    fn uuid(&self) -> &String;
    fn parent(&self) -> &String;
    fn model_id(&self) -> i64;
}

