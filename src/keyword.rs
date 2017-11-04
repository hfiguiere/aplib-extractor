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
use audit::{audit_get_int_value, Report};

/// An Aperture keyword.
pub struct Keyword {
    /// The uuid
    uuid: Option<String>,
    /// The numeric id in the model
    model_id: Option<i64>,
    /// The parent uuid
    parent_uuid: Option<String>,

    /// Name of the keyword
    pub name: Option<String>,
    /// Children keywords. Their parent_uuid will be self.uuid
    pub children: Option<Vec<Keyword>>,
}

impl AplibObject for Keyword {
    #[allow(unused_variables)]
    #[doc(hidden)]
    fn from_path(plist_path: &Path,
                 auditor: Option<&mut Report>) -> Option<Keyword> {
        assert!(false, "must not be called");
        None
    }
    fn obj_type(&self) -> AplibType {
        AplibType::Keyword
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
    #[doc(hidden)]
    fn wrap(_: Keyword) -> store::Wrapper {
        store::Wrapper::None
    }
}

/// Parse keywords from the .plist file
pub fn parse_keywords(path: &Path, auditor: &mut Option<&mut Report>) -> Option<Vec<Keyword>>
{
    let plist = parse_plist(path);

    match plist {
        Plist::Dictionary(ref dict) => {
            let version = try_opt!(audit_get_int_value(dict, "keywords_version", auditor));
            // XXX deal with proper errors here.
            // Version 3.4.5 has version 7.
            if version != 6 && version != 7 {
                println!("Wrong keyword version {} !", version);
            }
            Keyword::from_array(get_array_value(dict, "keywords"))
        },
        _ => None
    }
}

impl Keyword {

    /// convert a Plist array to a vec of keyword.
    fn from_array(oa: Option<Vec<Plist>>) -> Option<Vec<Keyword>>
    {
        let a = try_opt!(oa);

        let mut keywords = Vec::new();
        for item in a {
            match item {
                Plist::Dictionary(ref kw) => {
                    keywords.push(Keyword::from(kw));
                },
                _ => ()
            }
        }
        Some(keywords)
    }

    /// Create a new keyword from a plist dictionary
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
