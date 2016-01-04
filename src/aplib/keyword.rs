/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

extern crate plist;

use std::path::Path;
use aplib::plutils::*;
use aplib::{AplibObject,AplibType};
use self::plist::Plist;
use std::collections::BTreeMap;

pub struct Keyword {
    uuid: String,
    model_id: i64,
    parent_uuid: String,

    pub name: String,
    pub children: Vec<Keyword>,
}

impl AplibObject for Keyword {
    fn from_path(plist_path: &Path) -> Keyword {
        assert!(false, "must not be called");
        Keyword { uuid: "".to_string(),
                  model_id: 0,
                  parent_uuid: "".to_string(),
                  name: "".to_string(),
                  children: Vec::new() }
    }
    fn obj_type(&self) -> AplibType {
        return AplibType::KEYWORD;
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

}

/// Parse keywords from the .plist file
pub fn parse_keywords(path: &Path) -> Vec<Keyword>
{
    let plist = parse_plist(path);

    return match plist {
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
        return keywords;
    }

    /// create a new keyword from a plist dictionary
    /// will recursively create the children
    pub fn from(d: &BTreeMap<String, Plist>) -> Keyword
    {
        return Keyword {
            uuid: get_str_value(d, "uuid"),
            model_id: get_int_value(d, "modelId"),
            parent_uuid: get_str_value(d, "parentUuid"),
            name: get_str_value(d, "name"),
            children: Keyword::from_array(get_array_value(d, "zChildren")),
        };
    }
}
