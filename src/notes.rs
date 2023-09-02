/*
 This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

use chrono::{DateTime, Utc};

use crate::audit::{
    audit_get_data_value, audit_get_date_value, audit_get_int_value, audit_get_str_value, Report,
    SkipReason,
};
use crate::plutils::Value;

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
    pub fn from_array_element(
        dict: &plist::Dictionary,
        mut auditor: &mut Option<&mut Report>,
    ) -> NotesProperties {
        let result = NotesProperties {
            attached_to_uuid: audit_get_str_value(dict, "attachedToUuid", auditor),
            create_date: audit_get_date_value(dict, "createDate", auditor),
            data: audit_get_data_value(dict, "data", auditor),
            model_id: audit_get_int_value(dict, "modelId", auditor),
            note: audit_get_str_value(dict, "note", auditor),
            property_key: audit_get_str_value(dict, "propertyKey", auditor),
            uuid: audit_get_str_value(dict, "uuid", auditor),
        };
        if let Some(auditor) = &mut auditor {
            auditor.audit_ignored(dict, Some("notes"));
        }
        result
    }

    pub fn from(
        array: &Option<Vec<Value>>,
        mut auditor: &mut Option<&mut Report>,
    ) -> Option<Vec<NotesProperties>> {
        if let Some(array) = array.as_ref() {
            let mut result: Vec<NotesProperties> = vec![];

            for (counter, value) in array.iter().enumerate() {
                match *value {
                    Value::Dictionary(ref d) => {
                        result.push(NotesProperties::from_array_element(d, auditor))
                    }
                    _ => {
                        if let Some(auditor) = &mut auditor {
                            auditor.skip(
                                format!("notes[{}]", counter).as_ref(),
                                SkipReason::InvalidType,
                            );
                        }
                    }
                }
            }
            Some(result)
        } else {
            None
        }
    }
}
