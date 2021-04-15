/*
 This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

use crate::album::Album;
use crate::folder::Folder;
use crate::master::Master;
use crate::version::Version;
use crate::AplibObject;

/// Wrap an AplibObject to key it into the store.
pub enum Wrapper {
    Album(Box<Album>),
    Folder(Box<Folder>),
    Master(Box<Master>),
    Version(Box<Version>),
    None,
}

impl Wrapper {
    /// Extract the uuid from the wrapper
    pub fn uuid(&self) -> Option<String> {
        match *self {
            Wrapper::Album(ref o) => o.uuid().clone(),
            Wrapper::Folder(ref o) => o.uuid().clone(),
            Wrapper::Version(ref o) => o.uuid().clone(),
            Wrapper::Master(ref o) => o.uuid().clone(),
            Wrapper::None => None,
        }
    }
}
