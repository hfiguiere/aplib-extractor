

use std::collections::BTreeMap;

use chrono::{DateTime, Utc};

use audit::{
    Report, SkipReason,
    audit_get_str_value, audit_get_int_value, audit_get_data_value,
    audit_get_date_value
};
use plutils::Plist;

#[derive(Debug)]
pub struct NotesProperties {
    attached_to_uuid: Option<String>,
    create_date: Option<DateTime<Utc>>,
    data: Option<Vec<u8>>,
    model_id: Option<i64>,
    note: Option<String>,
    uuid: Option<String>,
    property_key: Option<String>,
}

impl NotesProperties {

    pub fn from_array_element(dict: &BTreeMap<String, Plist>,
                mut auditor: &mut Option<&mut Report>) -> NotesProperties {
        let result = NotesProperties {
            attached_to_uuid: audit_get_str_value(dict, "attachedToUuid", &mut auditor),
            create_date: audit_get_date_value(dict, "createDate", &mut auditor),
            data: audit_get_data_value(dict, "data", &mut auditor),
            model_id: audit_get_int_value(dict, "modelId", &mut auditor),
            note: audit_get_str_value(dict, "note", &mut auditor),
            property_key: audit_get_str_value(dict, "propertyKey", &mut auditor),
            uuid: audit_get_str_value(dict, "uuid", &mut auditor),
        };
        if auditor.is_some() {
            let ref mut auditor = auditor.as_mut().unwrap();
            auditor.audit_ignored(dict, Some("notes"));
        }
        result
    }

    pub fn from(array: &Option<Vec<Plist>>,
                mut auditor: &mut Option<&mut Report>) -> Option<Vec<NotesProperties>> {
        if array.is_none() {
            return None;
        }
        let array = array.as_ref().unwrap();
        let mut result: Vec<NotesProperties> = vec!();

        let mut counter = 0u64;
        for value in array {
            match value {
                &Plist::Dictionary(ref d) =>
                    result.push(NotesProperties::from_array_element(d, auditor)),
                _ => {
                    if auditor.is_some() {
                        let ref mut auditor = auditor.as_mut().unwrap();
                        auditor.skip(format!("notes[{}]", counter).as_ref(), SkipReason::InvalidType);
                    }
                }
            }
            counter += 1;
        }
        Some(result)
    }
}