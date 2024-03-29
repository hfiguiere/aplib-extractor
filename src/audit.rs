/*
 This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Utc};

use crate::plutils::{
    get_array_value, get_bool_value, get_data_value, get_date_value, get_dict_value, get_int_value,
    get_str_value, Value,
};

#[derive(thiserror::Error, Debug)]
pub enum SkipReason {
    #[error("No Reason")]
    /// No reason
    None,
    #[error("Not found")]
    /// Didn't find it
    NotFound,
    #[error("Unknown Property")]
    /// Unknown property
    UnknownProp,
    #[error("Invalid Type")]
    /// Wrong type
    InvalidType,
    #[error("Invalid Data")]
    /// Data is invalid
    InvalidData,
    #[error("Parse Failed")]
    /// Couldn't be parsed
    ParseFailed,
    #[error("Ignore")]
    /// Deliberately ignore
    Ignore,
}

/// The audit reporter.  The idea it too list the properties that are
/// ignored, skipped or parsed.  In order to establish what we are
/// missing.
#[derive(Debug, Default)]
pub struct Reporter {
    ignored: HashSet<String>,
    skipped: HashMap<String, SkipReason>,
    parsed: HashMap<String, Report>,
}

impl Reporter {
    pub fn new() -> Reporter {
        Reporter::default()
    }

    pub fn ignore(&mut self, key: &str) {
        self.ignored.insert(key.to_owned());
    }
    pub fn get_ignored(&self) -> &HashSet<String> {
        &self.ignored
    }
    pub fn ignored_count(&self) -> usize {
        self.ignored.len()
    }
    pub fn skip(&mut self, key: &str, reason: SkipReason) {
        self.skipped.insert(key.to_owned(), reason);
    }
    pub fn get_skipped(&self) -> &HashMap<String, SkipReason> {
        &self.skipped
    }
    pub fn skipped_count(&self) -> usize {
        self.skipped.len()
    }
    pub fn parsed(&mut self, key: &str, report: Report) {
        self.parsed.insert(key.to_owned(), report);
    }
    pub fn get_parsed(&self) -> &HashMap<String, Report> {
        &self.parsed
    }
    pub fn parsed_count(&self) -> usize {
        self.parsed.len()
    }
}

/// Individual report for an object
#[derive(Debug, Default)]
pub struct Report {
    ignored: HashSet<String>,
    skipped: HashMap<String, SkipReason>,
    parsed: HashSet<String>,
}

impl Report {
    pub fn new() -> Report {
        Report::default()
    }

    pub fn ignore(&mut self, key: &str) {
        self.ignored.insert(key.to_owned());
    }
    pub fn get_ignored(&self) -> &HashSet<String> {
        &self.ignored
    }
    pub fn ignored_count(&self) -> usize {
        self.ignored.len()
    }

    pub fn skip(&mut self, key: &str, reason: SkipReason) {
        self.skipped.insert(key.to_owned(), reason);
    }
    pub fn get_skipped(&self) -> &HashMap<String, SkipReason> {
        &self.skipped
    }
    pub fn skipped_count(&self) -> usize {
        self.skipped.len()
    }

    pub fn parsed(&mut self, key: &str) {
        self.parsed.insert(key.to_owned());
    }
    pub fn get_parsed(&self) -> &HashSet<String> {
        &self.parsed
    }
    pub fn parsed_count(&self) -> usize {
        self.parsed.len()
    }

    pub fn audit_ignored(&mut self, dict: &plist::Dictionary, ns: Option<&str>) {
        let ignored_keys: HashSet<_> = {
            let skipped_keys: HashSet<_> = self.skipped.keys().cloned().collect();
            let known_keys = self.parsed.union(&skipped_keys).cloned().collect();
            let plist_keys: HashSet<_> = dict.keys().cloned().collect();
            plist_keys.difference(&known_keys).cloned().collect()
        };
        for key in ignored_keys {
            if ns.is_none() {
                self.ignore(&key);
            } else {
                let key = format!("{}.{}", ns.as_ref().unwrap(), key);
                self.ignore(&key);
            }
        }
    }
}

pub fn audit_get_str_value(
    dict: &plist::Dictionary,
    key: &str,
    report: &mut Option<&mut Report>,
) -> Option<String> {
    let value = get_str_value(dict, key);
    if let Some(ref mut report) = *report {
        match value {
            Some(_) => report.parsed(key),
            _ => report.skip(key, SkipReason::NotFound),
        }
    }
    value
}

pub fn audit_get_int_value(
    dict: &plist::Dictionary,
    key: &str,
    report: &mut Option<&mut Report>,
) -> Option<i64> {
    let value = get_int_value(dict, key);
    if let Some(ref mut report) = *report {
        match value {
            Some(_) => report.parsed(key),
            _ => report.skip(key, SkipReason::NotFound),
        }
    }
    value
}

pub fn audit_get_bool_value(
    dict: &plist::Dictionary,
    key: &str,
    report: &mut Option<&mut Report>,
) -> Option<bool> {
    let value = get_bool_value(dict, key);
    if let Some(ref mut report) = *report {
        match value {
            Some(_) => report.parsed(key),
            _ => report.skip(key, SkipReason::NotFound),
        }
    }
    value
}

pub fn audit_get_dict_value(
    dict: &plist::Dictionary,
    key: &str,
    report: &mut Option<&mut Report>,
) -> Option<plist::Dictionary> {
    let value = get_dict_value(dict, key);
    if let Some(ref mut report) = *report {
        match value {
            Some(_) => report.parsed(key),
            _ => report.skip(key, SkipReason::NotFound),
        }
    }
    value
}

pub fn audit_get_array_value(
    dict: &plist::Dictionary,
    key: &str,
    report: &mut Option<&mut Report>,
) -> Option<Vec<Value>> {
    let value = get_array_value(dict, key);
    if let Some(ref mut report) = *report {
        match value {
            Some(_) => report.parsed(key),
            _ => report.skip(key, SkipReason::NotFound),
        }
    }
    value
}

pub fn audit_get_date_value(
    dict: &plist::Dictionary,
    key: &str,
    report: &mut Option<&mut Report>,
) -> Option<DateTime<Utc>> {
    let value = get_date_value(dict, key);
    if let Some(ref mut report) = *report {
        match value {
            Some(_) => report.parsed(key),
            _ => report.skip(key, SkipReason::NotFound),
        }
    }
    value
}

pub fn audit_get_data_value(
    dict: &plist::Dictionary,
    key: &str,
    report: &mut Option<&mut Report>,
) -> Option<Vec<u8>> {
    let value = get_data_value(dict, key);
    if let Some(ref mut report) = *report {
        match value {
            Some(_) => report.parsed(key),
            _ => report.skip(key, SkipReason::NotFound),
        }
    }
    value
}
