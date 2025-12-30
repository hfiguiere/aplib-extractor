/*
 * Copyright (C) 2017-2025 Hubert Figuière
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use exempi2::Xmp;

/// Define namespace constants until we can get them out of Exempi.
pub mod ns {
    pub const NS_DC: &str = "http://purl.org/dc/elements/1.1/";
    pub const NS_IPTC4XMP: &str = "http://iptc.org/std/Iptc4xmpCore/1.0/xmlns/";
    pub const NS_XMP: &str = "http://ns.adobe.com/xap/1.0/";
    pub const NS_XMP_RIGHTS: &str = "http://ns.adobe.com/xap/1.0/rights/";
    pub const NS_PHOTOSHOP: &str = "http://ns.adobe.com/photoshop/1.0/";
    pub const NS_EXIF: &str = "http://ns.adobe.com/exif/1.0/";
    pub const NS_EXIF_AUX: &str = "http://ns.adobe.com/exif/1.0/aux/";
    pub const NS_TIFF: &str = "http://ns.adobe.com/tiff/1.0/";
}

#[derive(Clone, Debug)]
/// Define a property
pub struct XmpProperty {
    /// The namespace URI
    ns: &'static str,
    /// The property name
    property: &'static str,
    /// The index (if an array)
    index: Option<i32>,
    /// The sub property if applicable
    field: Option<Box<XmpProperty>>,
    /// The array content, split on `, `.
    array: bool,
}

impl XmpProperty {
    /// Create a new basic property.
    pub fn new(ns: &'static str, property: &'static str) -> XmpProperty {
        XmpProperty {
            ns,
            property,
            index: None,
            field: None,
            array: false,
        }
    }

    /// Create a new property that address a struct field.
    pub fn new_field(ns: &'static str, property: &'static str, field: XmpProperty) -> XmpProperty {
        XmpProperty {
            ns,
            property,
            index: None,
            field: Some(Box::new(field)),
            array: false,
        }
    }

    pub fn new_array(ns: &'static str, property: &'static str) -> XmpProperty {
        XmpProperty {
            ns,
            property,
            index: None,
            field: None,
            array: true,
        }
    }

    /// Put the property `value` into the XMP meta.
    pub fn put_into_xmp(&self, value: &str, xmp: &mut Xmp) -> bool {
        if self.array {
            if xmp
                .set_property(
                    self.ns,
                    self.property,
                    "",
                    exempi2::PropFlags::VALUE_IS_ARRAY,
                )
                .is_err()
            {
                return false;
            }
            for element in value.split(", ") {
                if xmp
                    .set_array_item(
                        self.ns,
                        self.property,
                        -1,
                        element,
                        exempi2::PropFlags::ARRAY_INSERT_AFTER,
                    )
                    .is_err()
                {
                    return false;
                }
            }
            return true;
        } else if self.index.is_none() && self.field.is_none() {
            return xmp
                .set_property(self.ns, self.property, value, exempi2::PropFlags::NONE)
                .is_ok();
        } else if let Some(ref field) = self.field {
            // XXX when there is the API in exempi, use it.
            // For now we have to compose the path by hand.
            if let Ok(prefix) = exempi2::namespace_prefix(field.ns) {
                let property = format!("{}/{}{}", self.property, prefix, field.property);
                return xmp
                    .set_property(self.ns, &property, value, exempi2::PropFlags::NONE)
                    .is_ok();
            }
        } else if let Some(index) = self.index {
            return xmp
                .set_array_item(
                    self.ns,
                    self.property,
                    index,
                    value,
                    exempi2::PropFlags::NONE,
                )
                .is_ok();
        }
        false
    }
}

/// Specify a translation method.
pub(crate) enum XmpTranslator {
    /// Simple property mapping.
    Property(XmpProperty),
    /// Custom (TBD).
    Custom,
    /// None. Ignore the property.
    None,
}

/// Trait for conversion to XMP.
pub trait ToXmp {
    /// Push the object properties to the `xmp` XMP meta.
    fn to_xmp(&self, xmp: &mut Xmp) -> bool;
}

#[cfg(test)]
#[test]
fn test_xmp() {
    use exempi2::Xmp;

    let mut xmp = Xmp::new();

    let prop1 = XmpProperty::new(ns::NS_DC, "creator");
    let prop2 = XmpProperty::new_field(
        ns::NS_IPTC4XMP,
        "CreatorContactInfo",
        XmpProperty::new(ns::NS_IPTC4XMP, "CiAdrCity"),
    );
    let prop3 = XmpProperty::new_array(ns::NS_DC, "subject");
    assert!(prop1.put_into_xmp("Batman", &mut xmp));
    assert!(prop2.put_into_xmp("Gotham", &mut xmp));
    assert!(prop3.put_into_xmp("The Joker, Two Faces", &mut xmp));

    let mut options: exempi2::PropFlags = exempi2::PropFlags::NONE;
    let value = xmp.get_property(prop1.ns, prop1.property, &mut options);
    assert!(value.is_ok());
    assert_eq!(value.unwrap().to_str(), Ok("Batman"));
    options = exempi2::PropFlags::NONE;
    let value = xmp.get_array_item(prop3.ns, prop3.property, 1, &mut options);
    assert_eq!(value.unwrap().to_str(), Ok("The Joker"));
    options = exempi2::PropFlags::NONE;
    let value = xmp.get_array_item(prop3.ns, prop3.property, 2, &mut options);
    assert_eq!(value.unwrap().to_str(), Ok("Two Faces"));
}
