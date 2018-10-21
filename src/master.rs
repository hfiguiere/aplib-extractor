/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use audit::{audit_get_array_value, audit_get_bool_value, audit_get_data_value,
            audit_get_date_value, audit_get_int_value, audit_get_str_value, Report, SkipReason};
use chrono::{DateTime, Utc};
use notes::NotesProperties;
use std::path::Path;
use store;
use AplibObject;
use AplibType;
use PlistLoadable;

/// A `Master` is a file backing an image (`Version`)
pub struct Master {
    uuid: Option<String>,
    model_id: Option<i64>,
    project_uuid: Option<String>,

    /// If it is RAW+JPEG, there is another master.
    pub alternate_master: Option<String>,
    /// uuid of the orignal version
    pub original_version_uuid: Option<String>,
    pub import_group_uuid: Option<String>,
    pub filename: Option<String>,
    pub name: Option<String>,
    pub original_version_name: Option<String>,
    pub db_version: Option<i64>,
    pub master_type: Option<String>,
    pub subtype: Option<String>,
    pub image_path: Option<String>,
    pub is_reference: Option<bool>,
    pub is_truly_raw: Option<bool>,
    pub is_in_trash: Option<bool>,
    pub is_missing: Option<bool>,
    pub is_externaly_editable: Option<bool>,
    pub create_date: Option<DateTime<Utc>>,
    pub image_date: Option<DateTime<Utc>>,
    pub file_creation_date: Option<DateTime<Utc>>,
    pub file_modification_date: Option<DateTime<Utc>>,
    pub original_file_name: Option<String>,
    pub file_size: Option<i64>,
    pub file_volume_uuid: Option<String>,
    pub color_space_name: Option<String>,
    pub pixel_format: Option<i64>,
    pub has_focus_points: Option<i64>,
    pub image_format: Option<i64>, // XXX fix this is a 4char MSB
    pub notes: Option<Vec<NotesProperties>>,
    pub colour_space_definition: Option<Vec<u8>>,
    pub face_detection_state: Option<i64>,
}

impl PlistLoadable for Master {
    fn from_path<P>(plist_path: P, mut auditor: Option<&mut Report>) -> Option<Master>
    where
        P: AsRef<Path>,
    {
        use plutils::*;
        let plist = parse_plist(plist_path);
        match plist {
            Plist::Dictionary(ref dict) => {
                let notes = audit_get_array_value(dict, "notes", &mut auditor);
                let result = Some(Master {
                    uuid: audit_get_str_value(dict, "uuid", &mut auditor),
                    alternate_master: audit_get_str_value(
                        dict,
                        "alternateMasterUuid",
                        &mut auditor,
                    ),
                    original_version_uuid: audit_get_str_value(
                        dict,
                        "originalVersionUuid",
                        &mut auditor,
                    ),
                    project_uuid: audit_get_str_value(dict, "projectUuid", &mut auditor),
                    import_group_uuid: audit_get_str_value(dict, "importGroupUuid", &mut auditor),
                    filename: audit_get_str_value(dict, "fileName", &mut auditor),
                    name: audit_get_str_value(dict, "name", &mut auditor),
                    original_version_name: audit_get_str_value(
                        dict,
                        "originalVersionName",
                        &mut auditor,
                    ),
                    original_file_name: audit_get_str_value(dict, "originalFileName", &mut auditor),
                    file_volume_uuid: audit_get_str_value(dict, "fileVolumeUuid", &mut auditor),
                    db_version: audit_get_int_value(dict, "version", &mut auditor),
                    master_type: audit_get_str_value(dict, "type", &mut auditor),
                    subtype: audit_get_str_value(dict, "subtype", &mut auditor),
                    model_id: audit_get_int_value(dict, "modelId", &mut auditor),
                    image_path: audit_get_str_value(dict, "imagePath", &mut auditor),
                    file_size: audit_get_int_value(dict, "fileSize", &mut auditor),
                    is_reference: audit_get_bool_value(dict, "fileIsReference", &mut auditor),
                    is_externaly_editable: audit_get_bool_value(
                        dict,
                        "isExternallyEditable",
                        &mut auditor,
                    ),
                    is_in_trash: audit_get_bool_value(dict, "isInTrash", &mut auditor),
                    is_missing: audit_get_bool_value(dict, "isMissing", &mut auditor),
                    is_truly_raw: audit_get_bool_value(dict, "isTrulyRaw", &mut auditor),
                    color_space_name: audit_get_str_value(dict, "colorSpaceName", &mut auditor),
                    create_date: audit_get_date_value(dict, "createDate", &mut auditor),
                    image_date: audit_get_date_value(dict, "imageDate", &mut auditor),
                    file_creation_date: audit_get_date_value(
                        dict,
                        "fileCreationDate",
                        &mut auditor,
                    ),
                    file_modification_date: audit_get_date_value(
                        dict,
                        "fileModificationDate",
                        &mut auditor,
                    ),
                    has_focus_points: audit_get_int_value(dict, "hasFocusPoints", &mut auditor),
                    image_format: audit_get_int_value(dict, "imageFormat", &mut auditor),
                    pixel_format: audit_get_int_value(dict, "pixelFormat", &mut auditor),
                    colour_space_definition: audit_get_data_value(
                        dict,
                        "colorSpaceDefinition",
                        &mut auditor,
                    ),
                    notes: NotesProperties::from(&notes, &mut auditor),
                    face_detection_state: audit_get_int_value(
                        dict,
                        "faceDetectionState",
                        &mut auditor,
                    ),
                });
                if let Some(auditor) = &mut auditor {
                    auditor.skip("fileAliasData", SkipReason::Ignore);
                    auditor.skip("importedBy", SkipReason::Ignore);
                    auditor.skip("importGroup", SkipReason::Ignore);
                    auditor.skip("plistWriteTimestamp", SkipReason::Ignore);

                    auditor.audit_ignored(dict, None);
                }
                result
            }
            _ => None,
        }
    }
}

impl AplibObject for Master {
    fn obj_type(&self) -> AplibType {
        AplibType::Master
    }
    fn uuid(&self) -> &Option<String> {
        &self.uuid
    }
    fn parent(&self) -> &Option<String> {
        &self.project_uuid
    }
    fn model_id(&self) -> i64 {
        self.model_id.unwrap_or(0)
    }
    fn is_valid(&self) -> bool {
        self.uuid.is_some()
    }
    fn wrap(obj: Master) -> store::Wrapper {
        store::Wrapper::Master(Box::new(obj))
    }
}

impl Master {}

#[cfg(test)]
#[test]
fn test_master_parse() {
    use testutils;

    let master = Master::from_path(
        testutils::get_test_file_path("Master.apmaster").as_path(),
        None,
    );
    assert!(master.is_some());
    let master = master.unwrap();

    assert_eq!(master.uuid.as_ref().unwrap(), "JpLq7STrRMmgm5YZTm6IzA");
    assert_eq!(
        master.project_uuid.as_ref().unwrap(),
        "evHgvM2oQ3GR0j6gEMnNTQ"
    );
    assert_eq!(
        master.original_version_uuid.as_ref().unwrap(),
        "VF%CkiTKQy+h53Oyr7KCOA"
    );
    assert_eq!(
        master.color_space_name.as_ref().unwrap(),
        "kCGColorSpaceGenericHDR"
    );
    assert!(master.is_reference.unwrap());
    assert_eq!(master.filename.as_ref().unwrap(), "img_8826.cr2");
    assert_eq!(master.master_type.as_ref().unwrap(), "IMGT");
    assert_eq!(master.subtype.as_ref().unwrap(), "RAWST");

    // XXX fix when have actual audit.
    //    println!("report {:?}", report);
}
