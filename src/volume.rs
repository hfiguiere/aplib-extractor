/*
 This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

use std::path::Path;

use crate::audit::{audit_get_int_value, audit_get_str_value, Report};
use crate::store;
use crate::{AplibObject, AplibType, PlistLoadable, Result, SqliteLoadable};

pub struct Volume {
    uuid: Option<String>,
    model_id: Option<i64>,

    pub disk_uuid: Option<String>,
    pub volume_name: Option<String>,
}

impl SqliteLoadable for Volume {
    fn tables() -> &'static str {
        "RKVolume"
    }

    fn columns() -> &'static str {
        "modelId, uuid, name, diskUuid"
    }

    fn from_row(row: &rusqlite::Row) -> Result<Self>
    where
        Self: Sized,
    {
        let model_id = row.get(0)?;
        let uuid = row.get(1)?;
        let volume_name = row.get(2)?;
        let disk_uuid = row.get(3)?;
        Ok(Self {
            model_id,
            uuid,
            volume_name,
            disk_uuid,
        })
    }
}

impl PlistLoadable for Volume {
    /// Load the version object from the plist at plist_path.
    fn from_path<P>(plist_path: P, mut auditor: Option<&mut Report>) -> Option<Volume>
    where
        P: AsRef<Path>,
    {
        use crate::plutils::*;

        let plist = parse_plist(plist_path);
        match plist {
            Value::Dictionary(ref dict) => Some(Volume {
                uuid: audit_get_str_value(dict, "uuid", &mut auditor),
                model_id: audit_get_int_value(dict, "modelId", &mut auditor),
                disk_uuid: audit_get_str_value(dict, "diskUuid", &mut auditor),
                volume_name: audit_get_str_value(dict, "volumeName", &mut auditor),
            }),
            _ => None,
        }
    }
}
impl AplibObject for Volume {
    fn obj_type(&self) -> AplibType {
        AplibType::Volume
    }
    fn uuid(&self) -> &Option<String> {
        &self.uuid
    }
    fn parent(&self) -> &Option<String> {
        &None
    }
    fn model_id(&self) -> i64 {
        self.model_id.unwrap_or(0)
    }
    fn is_valid(&self) -> bool {
        self.uuid.is_some()
    }
    fn wrap(obj: Volume) -> store::Wrapper {
        store::Wrapper::Volume(Box::new(obj))
    }
}
