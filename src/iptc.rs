/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use std::collections::BTreeMap;
use plist::Plist;
use audit::{
    audit_get_str_value,
    Report
};

pub struct IptcProperties {
    ci_adr_city: Option<String>,
    ci_adr_ctry: Option<String>,
    ci_adr_region: Option<String>,
    ci_email_work: Option<String>,
    ci_url_work: Option<String>,

    city: Option<String>,
    copyright_notice: Option<String>,
    country_primary_loc_code: Option<String>,
    country_primary_loc_name: Option<String>,
    province_state: Option<String>,

    keywords: Option<String>, // XXX change this to a Vec<String>
}

impl IptcProperties {

    pub fn from(dict: &Option<BTreeMap<String, Plist>>,
                mut auditor: &mut Option<&mut Report>) -> Option<IptcProperties> {
        if dict.is_none() {
            return None;
        }
        let dict = dict.as_ref().unwrap();
        let result = Some(IptcProperties{
            ci_adr_city: audit_get_str_value(dict, "CiAdrCity", &mut auditor),
            ci_adr_ctry: audit_get_str_value(dict, "CiAdrCtry", &mut auditor),
            ci_adr_region: audit_get_str_value(dict, "CiAdrRegion", &mut auditor),
            ci_email_work: audit_get_str_value(dict, "CiEmailWork", &mut auditor),
            ci_url_work: audit_get_str_value(dict, "CiUrlWork", &mut auditor),
            city: audit_get_str_value(dict, "City", &mut auditor),
            copyright_notice: audit_get_str_value(dict, "CopyrightNotice", &mut auditor),
            country_primary_loc_code: audit_get_str_value(dict, "Country/PrimaryLocationCode", &mut auditor),
            country_primary_loc_name: audit_get_str_value(dict, "Country/PrimaryLocationName", &mut auditor),
            province_state: audit_get_str_value(dict, "Province/State", &mut auditor),
            keywords: audit_get_str_value(dict, "Keywords", &mut auditor),
        });
        if let Some(ref mut r) = *auditor {
            r.audit_ignored(&dict, Some("Iptc"));
        }
        result
    }

}