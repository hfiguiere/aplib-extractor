/*
 This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

use chrono::{DateTime, Utc};
use exempi::Xmp;
use std::path::Path;

use crate::audit::{
    audit_get_array_value, audit_get_bool_value, audit_get_date_value, audit_get_dict_value,
    audit_get_int_value, audit_get_str_value, Report, SkipReason,
};
use crate::custominfo::CustomInfoProperties;
use crate::exif::ExifProperties;
use crate::iptc::IptcProperties;
use crate::plutils::Value;
use crate::store;
use crate::xmp::ToXmp;
use crate::AplibObject;
use crate::AplibType;
use crate::PlistLoadable;

/// A rendered image. There is one for the orignal, and one per
/// actual version. `Version` are associated to a `Master`.
pub struct Version {
    uuid: Option<String>,
    model_id: Option<i64>,
    /// The associated `Master`.
    master_uuid: Option<String>,

    /// uuid of the `Folder` project this reside in.
    pub project_uuid: Option<String>,
    /// uuid of the raw `Master`.
    pub raw_master_uuid: Option<String>,
    /// uuid of the non raw `Master`.
    pub nonraw_master_uuid: Option<String>,
    pub timezone_name: Option<String>,
    pub create_date: Option<DateTime<Utc>>,
    pub image_date: Option<DateTime<Utc>>,
    pub export_image_change_date: Option<DateTime<Utc>>,
    pub export_metadata_change_date: Option<DateTime<Utc>>,
    pub version_number: Option<i64>,
    pub db_version: Option<i64>,
    pub db_minor_version: Option<i64>,
    pub is_flagged: Option<bool>,
    /// Indicate the version is the original.
    pub is_original: Option<bool>,
    pub is_editable: Option<bool>,
    pub is_hidden: Option<bool>,
    pub is_in_trash: Option<bool>,
    pub file_name: Option<String>,
    pub name: Option<String>,
    pub rating: Option<i64>,
    pub rotation: Option<i64>,
    pub colour_label_index: Option<i64>,

    pub iptc: Option<IptcProperties>,
    pub exif: Option<ExifProperties>,
    pub custom_info: Option<CustomInfoProperties>,
    pub keywords: Option<Vec<Value>>,
}

impl PlistLoadable for Version {
    /// Load the version object from the plist at plist_path.
    fn from_path<P>(plist_path: P, mut auditor: Option<&mut Report>) -> Option<Version>
    where
        P: AsRef<Path>,
    {
        use crate::plutils::*;

        let plist = parse_plist(plist_path);
        match plist {
            Value::Dictionary(ref dict) => {
                let iptc = audit_get_dict_value(dict, "iptcProperties", &mut auditor);
                let exif = audit_get_dict_value(dict, "exifProperties", &mut auditor);
                let custom_info = audit_get_dict_value(dict, "customInfo", &mut auditor);
                let result = Some(Version {
                    uuid: audit_get_str_value(dict, "uuid", &mut auditor),
                    master_uuid: audit_get_str_value(dict, "masterUuid", &mut auditor),
                    project_uuid: audit_get_str_value(dict, "projectUuid", &mut auditor),
                    raw_master_uuid: audit_get_str_value(dict, "rawMasterUuid", &mut auditor),
                    nonraw_master_uuid: audit_get_str_value(dict, "nonRawMasterUuid", &mut auditor),
                    timezone_name: audit_get_str_value(dict, "imageTimeZoneName", &mut auditor),
                    create_date: audit_get_date_value(dict, "createDate", &mut auditor),
                    image_date: audit_get_date_value(dict, "imageDate", &mut auditor),
                    export_image_change_date: audit_get_date_value(
                        dict,
                        "exportImageChangeDate",
                        &mut auditor,
                    ),
                    export_metadata_change_date: audit_get_date_value(
                        dict,
                        "exportMetadataChangeDate",
                        &mut auditor,
                    ),
                    version_number: audit_get_int_value(dict, "versionNumber", &mut auditor),
                    db_version: audit_get_int_value(dict, "version", &mut auditor),
                    db_minor_version: audit_get_int_value(dict, "minorVersion", &mut auditor),
                    is_flagged: audit_get_bool_value(dict, "isFlagged", &mut auditor),
                    is_original: audit_get_bool_value(dict, "isOriginal", &mut auditor),
                    is_editable: audit_get_bool_value(dict, "isEditable", &mut auditor),
                    is_hidden: audit_get_bool_value(dict, "isHidden", &mut auditor),
                    is_in_trash: audit_get_bool_value(dict, "isInTrash", &mut auditor),
                    file_name: audit_get_str_value(dict, "fileName", &mut auditor),
                    name: audit_get_str_value(dict, "name", &mut auditor),
                    model_id: audit_get_int_value(dict, "modelId", &mut auditor),
                    rating: audit_get_int_value(dict, "mainRating", &mut auditor),
                    rotation: audit_get_int_value(dict, "rotation", &mut auditor),
                    colour_label_index: audit_get_int_value(dict, "colorLabelIndex", &mut auditor),
                    iptc: IptcProperties::from(&iptc, &mut auditor),
                    exif: ExifProperties::from(&exif, &mut auditor),
                    custom_info: CustomInfoProperties::from(&custom_info, &mut auditor),
                    keywords: audit_get_array_value(dict, "keywords", &mut auditor),
                });
                if let Some(auditor) = &mut auditor {
                    auditor.skip("statistics", SkipReason::Ignore);
                    auditor.skip("thumbnailGroup", SkipReason::Ignore);
                    auditor.skip("faceDetectionIsFromPreview", SkipReason::Ignore);
                    auditor.skip("processedHeight", SkipReason::Ignore);
                    auditor.skip("processedWidth", SkipReason::Ignore);
                    auditor.skip("masterHeight", SkipReason::Ignore);
                    auditor.skip("masterWidth", SkipReason::Ignore);
                    auditor.skip("supportedStatus", SkipReason::Ignore);
                    auditor.skip("showInLibrary", SkipReason::Ignore);

                    auditor.skip("adjustmentProperties", SkipReason::Ignore); // don't know what to do yet
                    auditor.skip("RKImageAdjustments", SkipReason::Ignore);
                    auditor.skip("hasAdjustments", SkipReason::Ignore);
                    auditor.skip("hasEnabledAdjustments", SkipReason::Ignore);
                    auditor.skip("renderVersion", SkipReason::Ignore);

                    auditor.skip("imageProxyState", SkipReason::Ignore);
                    auditor.skip("plistWriteTimestamp", SkipReason::Ignore);
                    auditor.audit_ignored(dict, None);
                }
                result
            }
            _ => None,
        }
    }
}

impl AplibObject for Version {
    fn obj_type(&self) -> AplibType {
        AplibType::Version
    }
    fn uuid(&self) -> &Option<String> {
        &self.uuid
    }
    fn parent(&self) -> &Option<String> {
        &self.master_uuid
    }
    fn model_id(&self) -> i64 {
        self.model_id.unwrap_or(0)
    }
    fn is_valid(&self) -> bool {
        self.uuid.is_some()
    }
    fn wrap(obj: Version) -> store::Wrapper {
        store::Wrapper::Version(Box::new(obj))
    }
}

impl ToXmp for Version {
    fn to_xmp(&self, xmp: &mut Xmp) -> bool {
        // Here we make sure the Exif data are
        // processed before Iptc.
        if let Some(ref exif) = self.exif {
            exif.to_xmp(xmp);
        }
        if let Some(ref iptc) = self.iptc {
            iptc.to_xmp(xmp);
        }
        true
    }
}

#[cfg(test)]
#[test]
fn test_version_parse() {
    use crate::testutils;
    use crate::xmp;
    use exempi;

    let version = Version::from_path(
        testutils::get_test_file_path("Version-0.apversion").as_path(),
        None,
    );
    assert!(version.is_some());
    let version = version.unwrap();

    assert_eq!(version.uuid.as_ref().unwrap(), "MHMIbw5CQaiMgQ3n7g2w2A");
    assert!(version.is_original.unwrap());
    assert_eq!(
        version.master_uuid.as_ref().unwrap(),
        "WZMCPPRHR%C3nffgeeS4IQ"
    );
    assert_eq!(version.name.as_ref().unwrap(), "img_3136");
    assert!(version.iptc.is_some());
    let iptc = version.iptc.as_ref().unwrap();
    assert!(iptc.bag.contains_key("Byline"));
    assert!(iptc.bag.contains_key("CiAdrCity"));
    let exif = version.exif.as_ref().unwrap();
    assert!(exif.bag.contains_key("ApertureValue"));
    assert!(exif.bag.contains_key("Depth"));
    // XXX fix when have actual audit.
    //    println!("report {:?}", report);

    exempi::init();

    let mut xmp = Xmp::new();

    let result = version.to_xmp(&mut xmp);
    assert!(result);

    let mut options: exempi::PropFlags = exempi::PROP_NONE;
    let value = xmp.get_property(xmp::ns::NS_DC, "creator", &mut options);
    assert!(value.is_ok());
    assert_eq!(value.unwrap().to_str(), "Hubert Figuiere");

    options = exempi::PROP_NONE;
    let value = xmp.get_property(xmp::ns::NS_EXIF, "ApertureValue", &mut options);
    assert!(value.is_ok());
    assert_eq!(value.unwrap().to_str(), "4");
}
