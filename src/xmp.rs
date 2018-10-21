/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use exempi;
use exempi::Xmp;

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

/// Define a property
#[derive(Clone, Debug)]
pub struct XmpProperty {
    /// The namespace URI
    ns: &'static str,
    /// The property name
    property: &'static str,
    /// The index (if an array)
    index: Option<i32>,
    /// The sub property if applicable
    field: Option<Box<XmpProperty>>,
}

impl XmpProperty {
    /// Create a new basic property.
    pub fn new(ns: &'static str, property: &'static str) -> XmpProperty {
        XmpProperty {
            ns,
            property,
            index: None,
            field: None,
        }
    }

    /// Create a new property that address a struct field.
    pub fn new_field(ns: &'static str, property: &'static str, field: XmpProperty) -> XmpProperty {
        XmpProperty {
            ns,
            property,
            index: None,
            field: Some(Box::new(field)),
        }
    }

    /// Put the property `value` into the XMP meta.
    pub fn put_into_xmp(&self, value: &str, xmp: &mut Xmp) -> bool {
        if self.index.is_none() && self.field.is_none() {
            return xmp.set_property(self.ns, self.property, value, exempi::PROP_NONE);
        } else if let Some(ref field) = self.field {
            // XXX when there is the API in exempi, use it.
            // For now we have to compose the path by hand.
            if let Some(prefix) = exempi::namespace_prefix(field.ns) {
                let property = format!("{}/{}{}", self.property, prefix, field.property);
                return xmp.set_property(self.ns, &property, value, exempi::PROP_NONE);
            }
        } else if let Some(index) = self.index {
            return xmp.set_array_item(self.ns, self.property, index, value, exempi::PROP_NONE);
        }
        false
    }
}

/// Specify a translation method.
pub enum XmpTranslator {
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
    use exempi::Xmp;

    exempi::init();

    let mut xmp = Xmp::new();

    let prop1 = XmpProperty::new(ns::NS_DC, "creator");
    let prop2 = XmpProperty::new_field(
        ns::NS_IPTC4XMP,
        "CreatorContactInfo",
        XmpProperty::new(ns::NS_IPTC4XMP, "CiAdrCity"),
    );
    assert!(prop1.put_into_xmp("Batman", &mut xmp));
    assert!(prop2.put_into_xmp("Gotham", &mut xmp));

    let mut options: exempi::PropFlags = exempi::PROP_NONE;
    let value = xmp.get_property(prop1.ns, prop1.property, &mut options);
    assert!(value.is_some());
    assert_eq!(value.unwrap().to_str(), "Batman");
}
