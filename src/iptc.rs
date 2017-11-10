/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use std::collections::{BTreeMap,HashMap};
use plist::Plist;
use audit::{
    SkipReason,
    Report
};

lazy_static! {
    /// HashMap for IPTC properties (Aperture) to XMP.
    static ref IPTC_TO_XMP: HashMap<&'static str, &'static str> = hashmap!{
        "Byline" => "dc:creator",
        "BylineTitle" => "photoshop:AuthorsPosition",
        "Caption/Abstract" => "dc:description",
        "CiAdrCity" => "Iptc4xmpCore:CreatorContactInfo/Iptc4xmpCore:CiAdrCity",
        "CiAdrCtry" => "Iptc4xmpCore:CreatorContactInfo/Iptc4xmpCore:CiAdrCtry",
        "CiAdrExtadr" => "Iptc4xmpCore:CreatorContactInfo/Iptc4xmpCore:CiAdrExtad",
        "CiAdrPcode" => "Iptc4xmpCore:CreatorContactInfo/Iptc4xmpCore:CiAdrPcode",
        "CiAdrRegion" => "Iptc4xmpCore:CreatorContactInfo/Iptc4xmpCore:CiAdrRegion",
        "CiEmailWork" => "Iptc4xmpCore:CreatorContactInfo/Iptc4xmpCore:CiEmailWork",
        "CiTelWork" => "Iptc4xmpCore:CreatorContactInfo/Iptc4xmpCore:CiTelWork",
        "City" => "photoshop:City",
        "CiUrlWork" => "Iptc4xmpCore:CreatorContactInfo/Iptc4xmpCore:CiUrlWork",
        "CopyrightNotice" => "dc:rights",
        "Country/PrimaryLocationCode" => "Iptc4xmpCore:CountryCode",
        "Country/PrimaryLocationName" => "photoshop:Country",
        "Credit" => "photoshop:Credit",
//        "DateCreated" => "",
        "Headline" => "photoshop:Headline",
        "Keywords" => "dc:subject",
//        "Label" => "",
        "ObjectAttributeReference" => "Iptc4xmpCore:IntellectualGenre",
        "ObjectName" => "dc:title",
        "OriginalTransmissionReference" => "photoshop:TransmissionReference",
        "Province/State" => "photoshop:State>",
        "Scene" => "Iptc4xmpCore:Scene",
        "Source" => "photoshop:Source",
        "SpecialInstructions" => "photoshop:Instructions",
        "SubjectReference" => "Iptc4xmpCore:SubjectReference",
        "SubLocation" => "Iptc4xmpCore:Location",
        "StarRating" => "xap:Rating",
//        "TimeCreated" => "",
        "UsageTerms" => "xapRights:UsageTerms",
        "Writer/Editor" => "photoshop:CaptionWriter",
    };
}

#[derive(PartialEq)]
pub enum IptcValue {
    None,
    Str(String)
}

pub struct IptcProperties {
    pub bag: BTreeMap<String, IptcValue>,
}

impl IptcProperties {

    pub fn from(dict: &Option<BTreeMap<String, Plist>>,
                auditor: &mut Option<&mut Report>) -> Option<IptcProperties> {
        let dict = try_opt!(dict.as_ref());
        let mut values: BTreeMap<String, IptcValue> = BTreeMap::new();
        for (key, value) in dict {
            match value {
                &Plist::String(ref s) => {
                    values.insert(key.to_owned(), IptcValue::Str(s.to_owned()));
                    if let Some(ref mut r) = *auditor {
                        if !IPTC_TO_XMP.contains_key(&key.as_str()) {
                            r.skip(&format!("Iptc.{}", key),
                                   SkipReason::UnknownProp);
                        }
                    }
                },
                _ => if let Some(ref mut r) = *auditor {
                    r.skip(&format!("Iptc.{}", key), SkipReason::InvalidType);
                }
            };
        }
        Some(IptcProperties{bag: values})
    }

}
