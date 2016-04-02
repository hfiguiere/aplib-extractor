/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use folder::Folder;
use album::Album;
use version::Version;
use master::Master;
use AplibObject;

/// Wrap an AplibObject to key it into the store.
pub enum Wrapper {
    Album(Box<Album>),
    Folder(Box<Folder>),
    Master(Box<Master>),
    Version(Box<Version>),
    None
}

impl Wrapper {
    /// Extract the uuid from the wrapper
    pub fn uuid(&self) -> Option<&String> {
        match *self {
            Wrapper::Album(ref o) => {
                Some(o.uuid())
            },
            Wrapper::Folder(ref o) => {
                Some(o.uuid())
            },
            Wrapper::Version(ref o) => {
                Some(o.uuid())
            },
            Wrapper::Master(ref o) => {
                Some(o.uuid())
            },
            Wrapper::None => Option::None
        }
    }
}
