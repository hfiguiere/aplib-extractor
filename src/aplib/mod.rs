/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

pub mod plutils;
pub mod library;
pub mod folder;
pub mod album;
pub mod master;
pub mod version;
pub mod keyword;

use std::path::Path;
use mopa::Any;

pub enum AplibType {
    FOLDER,
    ALBUM,
    KEYWORD,
    MASTER,
    VERSION,
}

pub trait AplibObject: Any + 'static {
    fn from_path(plist_path: &Path) -> Self where Self: Sized;
    fn obj_type(&self) -> AplibType;
    fn is_valid(&self) -> bool;
    fn uuid(&self) -> &String;
    fn parent(&self) -> &String;
    fn model_id(&self) -> i64;
}

mopafy!(AplibObject);
