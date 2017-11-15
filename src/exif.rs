/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use std::collections::{BTreeMap, HashMap};

use chrono::{DateTime,Utc};
use exempi::Xmp;
use plist::Plist;

use audit::{
    SkipReason,
    Report
};
use xmp::{
    ToXmp,
    XmpProperty,
    XmpTranslator
};
use xmp::ns::*;

lazy_static! {
    /// HashMap for Exif properties (Aperture) to XMP.
    static ref EXIF_TO_XMP: HashMap<&'static str, XmpTranslator> = hashmap!{
        "ApertureValue" => XmpTranslator::Property(XmpProperty::new(
            NS_EXIF, "ApertureValue")),
        "Artist" => XmpTranslator::Property(XmpProperty::new(
            NS_DC, "creator")),
        "CameraSerialNumber" => XmpTranslator::Property(XmpProperty::new(
            NS_EXIF_AUX, "SerialNumber")),
        "CaptureDayOfMonth" => XmpTranslator::Custom,
        "CaptureDayOfWeek" => XmpTranslator::None,
        "CaptureHourOfDay" => XmpTranslator::None,
        "CaptureMinuteOfHour" => XmpTranslator::None,
        "CaptureMonthOfYear" => XmpTranslator::None,
        "CaptureSecondOfMinute" => XmpTranslator::None,
        "CaptureYear" => XmpTranslator::None,
        //"ColorModel"
        "ColorSpace" => XmpTranslator::Property(XmpProperty::new(
            NS_EXIF, "ColorSpace")),
        "Copyright" => XmpTranslator::Property(XmpProperty::new(
            NS_DC, "rights")),
        "Contrast" => XmpTranslator::Property(XmpProperty::new(
            NS_EXIF, "Contrast")),
        "Depth" => XmpTranslator::Property(XmpProperty::new(
            NS_TIFF, "BitsPerSample")),
        "ExifVersion" => XmpTranslator::Property(XmpProperty::new(
            NS_EXIF, "ExifVersion")),
        "ExposureBiasValue" => XmpTranslator::Property(XmpProperty::new(
            NS_EXIF, "ExposureBiasValue")),
        "ExposureMode" => XmpTranslator::Property(XmpProperty::new(
            NS_EXIF, "ExposureMode")),
        "ExposureProgram" => XmpTranslator::Property(XmpProperty::new(
            NS_EXIF, "ExposureProgram")),
        "Firmware" => XmpTranslator::Property(XmpProperty::new(
            NS_EXIF_AUX, "Firmware")),
        "Flash" => XmpTranslator::Custom,
        "FlashExposureComp"  => XmpTranslator::Property(XmpProperty::new(
            NS_EXIF_AUX, "FlashCompensation")),
        "FlashPixVersion" => XmpTranslator::Property(XmpProperty::new(
            NS_EXIF, "FlashPixVersion")),
        "FocalLength" => XmpTranslator::Property(XmpProperty::new(
            NS_EXIF, "FocalLength")),
        "FocusDistance" => XmpTranslator::Property(XmpProperty::new(
            NS_EXIF_AUX, "ApproximateFocusDistance")),
        //    +- Exif.FocusMode
        "ISOSpeedRating" => XmpTranslator::Custom, // ISOSpeedRatings[] as int
        "ImageDate" => XmpTranslator::Property(XmpProperty::new(
            NS_EXIF, "DateTimeOriginal")),
        // It is possible that this will be overwritten by IPTC
        // Also it seems that by default Olympus files have this property
        // set to something irrelevant.
        "ImageDescription" => XmpTranslator::Property(XmpProperty::new(
            NS_DC, "description")),
        "Latitude"  => XmpTranslator::Property(XmpProperty::new(
            NS_EXIF, "GPSLatitude")),
        "LensMaxMM" => XmpTranslator::None,
        "LensMinMM" => XmpTranslator::Custom,
        "LensModel" => XmpTranslator::Property(XmpProperty::new(
            NS_EXIF_AUX, "Lens")),
        "LightSource" => XmpTranslator::Property(XmpProperty::new(
            NS_EXIF, "LightSource")),
        "Longitude" => XmpTranslator::Property(XmpProperty::new(
            NS_EXIF, "GPSLongitude")),
        "Make" =>  XmpTranslator::Property(XmpProperty::new(
            NS_TIFF, "Make")),
        "MaxApertureValue" => XmpTranslator::Property(XmpProperty::new(
            NS_EXIF, "MaxApertureValue")),
        "MeteringMode" => XmpTranslator::Property(XmpProperty::new(
            NS_EXIF, "MeteringMode")),
        "Model" => XmpTranslator::Property(XmpProperty::new(
            NS_TIFF, "Model")),
        "OwnerName" => XmpTranslator::Property(XmpProperty::new(
            NS_EXIF_AUX, "OwnerName")),
        "PixelHeight" => XmpTranslator::Property(XmpProperty::new(
            NS_EXIF, "PixelYDimension")),
        "PixelWidth" => XmpTranslator::Property(XmpProperty::new(
            NS_EXIF, "PixelXDimension")),
        //    +- Exif.ProfileName
        "Saturation"  => XmpTranslator::Property(XmpProperty::new(
            NS_EXIF, "Saturation")),
        "SceneCaptureType" => XmpTranslator::Property(XmpProperty::new(
            NS_EXIF, "SceneCaptureType")),
        "Sharpness" => XmpTranslator::Property(XmpProperty::new(
            NS_EXIF, "Sharpness")),
        "ShutterSpeed" => XmpTranslator::Property(XmpProperty::new(
            NS_EXIF, "ShutterSpeedValue")),
        "Software" => XmpTranslator::Property(XmpProperty::new(
            NS_XMP, "CreatorTool")),
        "WhiteBalance" => XmpTranslator::Property(XmpProperty::new(
            NS_EXIF, "SceneCaptureType")),
        //    +- Exif.WhiteBalanceIndex
    };
}

#[derive(PartialEq)]
pub enum ExifValue {
    None,
    Int(i64),
    Str(String),
    Date(DateTime<Utc>),
    Real(f64),
}

pub struct ExifProperties {
    pub bag: BTreeMap<String, ExifValue>,
}

impl ExifProperties {
    pub fn from(dict: &Option<BTreeMap<String, Plist>>,
                auditor: &mut Option<&mut Report>) -> Option<ExifProperties> {
       if dict.is_none() {
            return None;
        }
        let dict = dict.as_ref().unwrap();
        let mut values: BTreeMap<String,ExifValue> = BTreeMap::new();
        for (key, value) in dict {
            let ev = match value {
                &Plist::Integer(n) => ExifValue::Int(n),
                &Plist::Real(f) => ExifValue::Real(f),
                &Plist::String(ref s) => ExifValue::Str(s.to_owned()),
                &Plist::Date(ref d) => ExifValue::Date(d.clone().into()),
                _ => ExifValue::None,
            };
            if ev != ExifValue::None {
                values.insert(key.to_owned(), ev);
                if let Some(ref mut r) = *auditor {
                    if !EXIF_TO_XMP.contains_key(&key.as_str()) {
                        r.skip(&format!("Exif.{}", key),
                               SkipReason::UnknownProp);
                    }
                }
            } else if let Some(ref mut r) = *auditor {
                r.skip(&format!("Exif.{}", key), SkipReason::InvalidType);
            }
        }
        Some(ExifProperties{bag: values})
    }

    pub fn value_to_string(value: &ExifValue) -> Option<String> {
        match value {
            &ExifValue::Str(ref str) => Some(str.clone()),
            &ExifValue::Int(i) => Some(format!("{}", i)),
            _ => None
        }
    }
}

impl ToXmp for ExifProperties {

    fn to_xmp(&self, xmp: &mut Xmp) -> bool {
        for (key, value) in &self.bag {
            if let Some(ref translator) = EXIF_TO_XMP.get(&key.as_str()) {
                match *translator {
                    &XmpTranslator::Property(ref prop) => {

                        if let Some(value) = Self::value_to_string(value) {
                            println!("value = {}", value);

                            let result = prop.put_into_xmp(&value, xmp);
                            if !result {
                                println!("failed to add {}", key.as_str());
                            } else {
                                println!("added {}", key.as_str());
                            }
                        }
                    },
                    _ => {
                        println!("ignored {}", key.as_str());
                    },
                }
            }
        }
        true
    }
}
