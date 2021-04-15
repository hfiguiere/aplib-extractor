/*
 This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

use std::collections::{BTreeMap, HashMap};

use exempi::Xmp;
use plist::Value;

use crate::audit::{Report, SkipReason};
use crate::xmp::ns::*;
use crate::xmp::{ToXmp, XmpProperty, XmpTranslator};

lazy_static! {
    /// HashMap for IPTC properties (Aperture) to XMP.
    static ref IPTC_TO_XMP: HashMap<&'static str, XmpTranslator> = hashmap!{
        "Byline" => XmpTranslator::Property(XmpProperty::new(
            NS_DC, "creator")),
        "BylineTitle" => XmpTranslator::Property(XmpProperty::new(
            NS_PHOTOSHOP, "AuthorsPosition")),
        "Caption/Abstract" => XmpTranslator::Property(XmpProperty::new(
            NS_DC, "description")),
        "CiAdrCity" => XmpTranslator::Property(XmpProperty::new_field(
            NS_IPTC4XMP, "CreatorContactInfo", XmpProperty::new(
                NS_IPTC4XMP, "CiAdrCity"))),
        "CiAdrCtry" => XmpTranslator::Property(XmpProperty::new_field(
            NS_IPTC4XMP, "CreatorContactInfo", XmpProperty::new(
                NS_IPTC4XMP, "CiAdrCtry"))),
        "CiAdrExtadr" => XmpTranslator::Property(XmpProperty::new_field(
            NS_IPTC4XMP, "CreatorContactInfo", XmpProperty::new(
                NS_IPTC4XMP, "CiAdrExtadr"))),
        "CiAdrPcode" => XmpTranslator::Property(XmpProperty::new_field(
            NS_IPTC4XMP, "CreatorContactInfo", XmpProperty::new(
                NS_IPTC4XMP, "CiAdrPcode"))),
        "CiAdrRegion" => XmpTranslator::Property(XmpProperty::new_field(
            NS_IPTC4XMP, "CreatorContactInfo", XmpProperty::new(
                NS_IPTC4XMP, "CiAdrRegion"))),
        "CiEmailWork" => XmpTranslator::Property(XmpProperty::new_field(
            NS_IPTC4XMP, "CreatorContactInfo", XmpProperty::new(
                NS_IPTC4XMP, "CiEmailWork"))),
        "CiTelWork" => XmpTranslator::Property(XmpProperty::new_field(
            NS_IPTC4XMP, "CreatorContactInfo", XmpProperty::new(
                NS_IPTC4XMP, "CiTelWork"))),
        "City" => XmpTranslator::Property(XmpProperty::new(
            NS_PHOTOSHOP, "City")),
        "CiUrlWork" => XmpTranslator::Property(XmpProperty::new_field(
            NS_IPTC4XMP, "CreatorContactInfo", XmpProperty::new(
                NS_IPTC4XMP, "CiUrlWork"))),
        "CopyrightNotice" => XmpTranslator::Property(XmpProperty::new(
            NS_DC, "rights")),
        "Country/PrimaryLocationCode" => XmpTranslator::Property(
            XmpProperty::new(NS_IPTC4XMP, "CountryCode")),
        "Country/PrimaryLocationName" => XmpTranslator::Property(
            XmpProperty::new(NS_PHOTOSHOP, "Country")),
        "Credit" => XmpTranslator::Property(XmpProperty::new(
            NS_PHOTOSHOP, "Credit")),
//        "DateCreated" => "",
        "Headline" => XmpTranslator::Property(XmpProperty::new(
            NS_PHOTOSHOP, "Headline")),
        "Keywords" => XmpTranslator::Property(XmpProperty::new(
            NS_DC, "subject")),
//        "Label" => "",
        "ObjectAttributeReference" => XmpTranslator::Property(
            XmpProperty::new(NS_IPTC4XMP, "IntellectualGenre")),
        "ObjectName" => XmpTranslator::Property(XmpProperty::new(
            NS_DC, "title")),
        "OriginalTransmissionReference" => XmpTranslator::Property(
            XmpProperty::new(NS_PHOTOSHOP, "TransmissionReference")),
        "Province/State" => XmpTranslator::Property(XmpProperty::new(
            NS_PHOTOSHOP, "State")),
        "Scene" => XmpTranslator::Property(XmpProperty::new(
            NS_IPTC4XMP, "Scene")),
        "Source" => XmpTranslator::Property(XmpProperty::new(
            NS_PHOTOSHOP, "Source")),
        "SpecialInstructions" => XmpTranslator::Property(
            XmpProperty::new(NS_PHOTOSHOP, "Instructions")),
        "SubjectReference" => XmpTranslator::Property(XmpProperty::new(
            NS_IPTC4XMP, "SubjectReference")),
        "SubLocation" => XmpTranslator::Property(XmpProperty::new(
            NS_IPTC4XMP, "Location")),
        "StarRating" => XmpTranslator::Property(XmpProperty::new(
            NS_XMP, "Rating")),
        //        "TimeCreated" => "",
        "UsageTerms" => XmpTranslator::Property(XmpProperty::new(
            NS_XMP_RIGHTS, "UsageTerms")),
        "Writer/Editor" => XmpTranslator::Property(XmpProperty::new(
            NS_PHOTOSHOP, "CaptionWriter")),
    };
}

#[derive(PartialEq)]
pub enum IptcValue {
    None,
    Str(String),
}

pub struct IptcProperties {
    pub bag: BTreeMap<String, IptcValue>,
}

impl IptcProperties {
    pub fn from(
        dict: &Option<plist::Dictionary>,
        auditor: &mut Option<&mut Report>,
    ) -> Option<IptcProperties> {
        let dict = dict.as_ref()?;
        let mut values: BTreeMap<String, IptcValue> = BTreeMap::new();
        for (key, value) in dict {
            match *value {
                Value::String(ref s) => {
                    values.insert(key.to_owned(), IptcValue::Str(s.to_owned()));
                    if let Some(ref mut r) = *auditor {
                        if !IPTC_TO_XMP.contains_key(&key.as_str()) {
                            r.skip(&format!("Iptc.{}", key), SkipReason::UnknownProp);
                        }
                    }
                }
                _ => {
                    if let Some(ref mut r) = *auditor {
                        r.skip(&format!("Iptc.{}", key), SkipReason::InvalidType);
                    }
                }
            };
        }
        Some(IptcProperties { bag: values })
    }
}

impl ToXmp for IptcProperties {
    fn to_xmp(&self, xmp: &mut Xmp) -> bool {
        for (key, value) in &self.bag {
            let value = match *value {
                IptcValue::Str(ref str) => str,
                _ => continue,
            };
            if let Some(ref translator) = IPTC_TO_XMP.get(&key.as_str()) {
                if let XmpTranslator::Property(ref prop) = *(*translator) {
                    prop.put_into_xmp(&value, xmp);
                }
            }
        }
        true
    }
}
