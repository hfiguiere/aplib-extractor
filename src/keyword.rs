/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use std::path::Path;
use std::collections::BTreeMap;

use plutils::*;
use store;
use plist::Plist;
use AplibObject;
use AplibType;

/// A Keyword
pub struct Keyword {
    /// The uuid
    uuid: String,
    /// The numeric id in the model
    model_id: i64,
    /// The parent uuid
    parent_uuid: String,

    /// Name of the keyword
    pub name: String,
    /// Children keywords. Their parent_uuid will be self.uuid
    pub children: Vec<Keyword>,
}

impl AplibObject for Keyword {
    #[allow(unused_variables)]
    #[doc(hidden)]
    fn from_path(plist_path: &Path) -> Option<Keyword> {
        assert!(false, "must not be called");
        None
    }
    fn obj_type(&self) -> AplibType {
        AplibType::Keyword
    }
    fn uuid(&self) -> &String {
        &self.uuid
    }
    fn parent(&self) -> &String {
        &self.parent_uuid
    }
    fn model_id(&self) -> i64 {
        self.model_id
    }
    fn is_valid(&self) -> bool {
        !self.uuid.is_empty()
    }
    #[allow(unused_variables)]
    #[doc(hidden)]
    fn wrap(obj: Keyword) -> store::Wrapper {
        store::Wrapper::None
    }
}

/// Parse keywords from the .plist file
pub fn parse_keywords(path: &Path) -> Vec<Keyword>
{
    let plist = parse_plist(path);

    match plist {
        Plist::Dictionary(ref dict) => {
            let version = get_int_value(dict, "keywords_version");
            // XXX deal with proper errors here.
            if version != 6 {
                println!("Wrong keyword version !");
            }

            Keyword::from_array(get_array_value(dict, "keywords"))
        },
        _ => Vec::new()
    }
}

impl Keyword {

    /// convert a Plist array to a vec of keyword.
    fn from_array(a: Vec<Plist>) -> Vec<Keyword>
    {
        let mut keywords = Vec::new();
        for item in a {
            match item {
                Plist::Dictionary(ref kw) => {
                    keywords.push(Keyword::from(kw));
                },
                _ => ()
            }
        }
        keywords
    }

    /// create a new keyword from a plist dictionary
    /// will recursively create the children
    pub fn from(d: &BTreeMap<String, Plist>) -> Keyword
    {
        Keyword {
            uuid: get_str_value(d, "uuid"),
            model_id: get_int_value(d, "modelId"),
            parent_uuid: get_str_value(d, "parentUuid"),
            name: get_str_value(d, "name"),
            children: Keyword::from_array(get_array_value(d, "zChildren")),
        }
    }
}
