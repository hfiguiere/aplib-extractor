/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */


use std::collections::{ HashMap, HashSet };


#[derive(Debug)]
pub enum SkipReason {
    None,
    InvalidType,
    InvalidData,
}

/// The audit reporter.  The idea it too list the properties that are
/// ignored, skipped or parsed.  In order to establish what we are
/// missing.
#[derive(Debug)]
pub struct Reporter {
    ignored: HashSet<String>,
    skipped: HashMap<String, SkipReason>,
    parsed: HashMap<String, Report>,
}

impl Reporter {

    pub fn new() -> Reporter {
        Reporter {
            ignored: HashSet::new(),
            skipped: HashMap::new(),
            parsed: HashMap::new(),
        }
    }

    pub fn ignore(&mut self, key: &str) {
        self.ignored.insert(key.to_owned());
    }
    pub fn skip(&mut self, key: &str, reason: SkipReason) {
        self.skipped.insert(key.to_owned(), reason);
    }
    pub fn parsed(&mut self, key: &str, report: Report) {
        self.parsed.insert(key.to_owned(), report);
    }

}

/// Individual report for an object
#[derive(Debug)]
pub struct Report {
    ignored: HashSet<String>,
    skipped: HashMap<String, SkipReason>,
    parsed: HashSet<String>,
}

impl Report {
    pub fn new() -> Report {
        Report {
            ignored: HashSet::new(),
            skipped: HashMap::new(),
            parsed: HashSet::new(),
        }
    }

    pub fn ignore(&mut self, key: &str) {
        self.ignored.insert(key.to_owned());
    }
    pub fn skip(&mut self, key: &str, reason: SkipReason) {
        self.skipped.insert(key.to_owned(), reason);
    }
    pub fn parsed(&mut self, key: &str) {
        self.parsed.insert(key.to_owned());
    }
}

pub trait Auditable {
    fn audit(&self) -> Report;
}
