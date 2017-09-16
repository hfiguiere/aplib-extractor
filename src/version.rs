/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */


use std::path::Path;
use chrono::{DateTime,Utc};

use plutils::Plist;
use store;
use iptc::IptcProperties;
use exif::ExifProperties;
use AplibObject;
use AplibType;
use audit::{
    audit_get_str_value,
    audit_get_int_value,
    audit_get_bool_value,
    audit_get_dict_value,
    audit_get_date_value,
    audit_get_array_value,
    SkipReason,
    Report
};

pub struct Version {
    uuid: Option<String>,
    model_id: Option<i64>,
    master_uuid: Option<String>,

    pub project_uuid: Option<String>,
    pub raw_master_uuid: Option<String>,
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
    pub keywords: Option<Vec<Plist>>,
}

impl AplibObject for Version {
    /// Load the version object from the plist at plist_path.
    fn from_path(plist_path: &Path,
                 mut auditor: Option<&mut Report>) -> Option<Version> {

        use plutils::*;

        let plist = parse_plist(plist_path);
        match plist {
            Plist::Dictionary(ref dict) => {
                let iptc = audit_get_dict_value(dict, "iptcProperties", &mut auditor);
                let exif = audit_get_dict_value(dict, "exifProperties", &mut auditor);
                let result = Some(Version {
                    uuid: audit_get_str_value(dict, "uuid", &mut auditor),
                    master_uuid: audit_get_str_value(
                        dict, "masterUuid", &mut auditor),
                    project_uuid: audit_get_str_value(
                        dict, "projectUuid", &mut auditor),
                    raw_master_uuid: audit_get_str_value(
                        dict, "rawMasterUuid", &mut auditor),
                    nonraw_master_uuid: audit_get_str_value(
                        dict, "nonRawMasterUuid", &mut auditor),
                    timezone_name: audit_get_str_value(
                        dict, "imageTimeZoneName", &mut auditor),
                    create_date: audit_get_date_value(
                        dict, "createDate", &mut auditor),
                    image_date: audit_get_date_value(
                        dict, "imageDate", &mut auditor),
                    export_image_change_date: audit_get_date_value(
                        dict, "exportImageChangeDate", &mut auditor),
                    export_metadata_change_date: audit_get_date_value(
                        dict, "exportMetadataChangeDate", &mut auditor),
                    version_number: audit_get_int_value(
                        dict, "versionNumber", &mut auditor),
                    db_version: audit_get_int_value(
                        dict, "version", &mut auditor),
                    db_minor_version: audit_get_int_value(
                        dict, "minorVersion", &mut auditor),
                    is_flagged: audit_get_bool_value(
                        dict, "isFlagged", &mut auditor),
                    is_original: audit_get_bool_value(
                        dict, "isOriginal", &mut auditor),
                    is_editable: audit_get_bool_value(
                        dict, "isEditable", &mut auditor),
                    is_hidden: audit_get_bool_value(
                        dict, "isHidden", &mut auditor),
                    is_in_trash: audit_get_bool_value(
                        dict, "isInTrash", &mut auditor),
                    file_name: audit_get_str_value(
                        dict, "fileName", &mut auditor),
                    name: audit_get_str_value(dict, "name", &mut auditor),
                    model_id: audit_get_int_value(
                        dict, "modelId", &mut auditor),
                    rating: audit_get_int_value(
                        dict, "mainRating", &mut auditor),
                    rotation: audit_get_int_value(
                        dict, "rotation", &mut auditor),
                    colour_label_index: audit_get_int_value(dict, "colorLabelIndex", &mut auditor),
                    iptc: IptcProperties::from(&iptc, &mut auditor),
                    exif: ExifProperties::from(&exif, &mut auditor),
                    keywords: audit_get_array_value(
                        dict, "keywords", &mut auditor),
                });
                if auditor.is_some() {
                    let ref mut auditor = auditor.unwrap();

                    auditor.skip("customInfo", SkipReason::Ignore); // XXX parse.

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
            },
            _ =>
                None
        }
    }
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
