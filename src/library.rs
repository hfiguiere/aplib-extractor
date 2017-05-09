/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use std::fs;
use std::path::{Path, PathBuf};
use std::collections::{HashMap,HashSet};

use plist::Plist;
use folder::Folder;
use album::Album;
use version::Version;
use master::Master;
use audit::{Reporter,Report,SkipReason,Auditable};
use keyword::{parse_keywords,Keyword};
use store;
use plutils;
use AplibObject;

// This is mostly from db_version = 110

const INFO_PLIST: &'static str = "Info.plist";

const BUNDLE_IDENTIFIER: &'static str = "com.apple.Aperture.library";

const DATABASE_DIR: &'static str = "Database";

// in Database
const DATAMODEL_VERSION_PLIST: &'static str = "DataModelVersion.plist";
const KEYWORDS_PLIST: &'static str = "Keywords.plist";
const ALBUMS_DIR: &'static str = "Albums";
const FOLDERS_DIR: &'static str = "Folders";
const VERSIONS_BASE_DIR: &'static str = "Versions";

/// Info of the library data model
pub struct ModelInfo {
    pub is_iphoto_library: Option<bool>,
    pub db_version: Option<i64>,
    pub db_minor_back_compatible_version: Option<i64>,
    pub db_minor_version: Option<i64>,
    pub db_uuid: Option<String>,
    pub create_date: Option<String>,
    pub image_io_version: Option<String>,
    pub raw_camera_bundle_version: Option<String>,
    pub touched_by_aperture: Option<bool>,
    pub master_count: Option<i64>,
    pub version_count: Option<i64>,
    pub project_compat_back_to_version: Option<i64>,
    pub project_version: Option<i64>,
}

impl ModelInfo {
    fn parse(plist : &Plist) -> Option<ModelInfo>
    {
        use plutils::{get_int_value,get_bool_value,get_str_value};

        match *plist {
            Plist::Dictionary(ref dict) => Some(ModelInfo {
                db_uuid: get_str_value(dict, "databaseUuid"),
                db_minor_back_compatible_version: get_int_value(
                    dict, "DatabaseCompatibleBackToMinorVersion"),
                db_minor_version: get_int_value(
                    dict, "DatabaseMinorVersion"),
                db_version: get_int_value(dict, "DatabaseVersion"),
                is_iphoto_library: get_bool_value(dict, "isIPhotoLibrary"),
                create_date: get_str_value(dict, "createDate"),
                image_io_version: get_str_value(dict, "imageIOVersion"),
                raw_camera_bundle_version: get_str_value(
                    dict, "rawCameraBundleVersion"),
                touched_by_aperture: get_bool_value(
                    dict, "touchedByAperture"),
                master_count: get_int_value(dict, "masterCount"),
                version_count: get_int_value(dict, "versionCount"),
                project_version: get_int_value(dict, "projectVersion"),
                project_compat_back_to_version: get_int_value(
                    dict, "projectCompatibleBackToVersion"),
            }),
            _ => None
        }
    }
}

/// Library is the Aperture library.
pub struct Library {
    /// The path to the .aplib bundle (the directory)
    path: String,

    /// It's version string (displayed by get info in the Finder)
    version: String,

    /// All the folders UUID
    folders: HashSet<String>,
    /// All the albums UUID
    albums: HashSet<String>,
    //    keywords: HashSet<String>,
    /// All the masters UUID
    masters: HashSet<String>,
    /// All the version UUID
    versions: HashSet<String>,

    /// The object store. The key is the UUID
    objects: HashMap<String, store::Wrapper>,
    /// Auditor for the audit mode.
    auditor: Option<Reporter>,
}

impl Library {

    /// Create a new library object from the exist path to
    /// the bundle directory.
    pub fn new(p: &str) -> Library
    {
        Library {
            path: p.to_owned(),
            version: "".to_owned(),

            folders: HashSet::new(),
            albums: HashSet::new(),
//            keywords: HashSet::new(),
            masters: HashSet::new(),
            versions: HashSet::new(),

            objects: HashMap::new(),
            auditor: None,
        }
    }

    /// Set an auditor.
    pub fn set_auditor(&mut self, auditor: Option<Reporter>) {
        self.auditor = auditor;
    }
    /// Get the auditor
    pub fn get_auditor(&self) -> Option<&Reporter> {
        self.auditor.as_ref()
    }

    /// Store the wrapped object.
    /// Return true if the object was stored
    /// Return false if there already was an object with the same uuid
    /// or if the uuid in invalid.
    pub fn store(&mut self, obj: store::Wrapper) -> bool
    {
        let uuid_str = {
            let uuid = obj.uuid();
            if uuid.is_none() {
                return false;
            }
            uuid.unwrap().to_owned()
        };
        match self.objects.insert(uuid_str, obj) {
            None => true,
            _ => false,
        }
    }

    /// Get an object out of the store by UUID
    pub fn get(&self, uuid: &str) -> Option<&store::Wrapper>
    {
        self.objects.get(uuid)
    }

    /// Get the library version. Will parse the plist for that
    /// if needed.
    pub fn library_version(&mut self) -> &String
    {
        if self.version.is_empty()
        {
            let plist_path = self.build_path(INFO_PLIST, false);
            let plist = plutils::parse_plist(&plist_path);
            let mut report = Report::new();
            let audit = self.auditor.is_some();


            match plist {
                Plist::Dictionary(ref dict) => {
                    for (key, value) in dict.iter() {

                        match key.as_ref() {
                            "CFBundleIdentifier" => {
                                if let &Plist::String(ref s) = value {
                                    let bundle_id = s.to_owned();
                                    if bundle_id != BUNDLE_IDENTIFIER {
                                        report.skip(key,
                                                    SkipReason::InvalidData);
                                        println!("FATAL not a library");
                                        return &self.version;
                                    }
                                    if audit {
                                        report.parsed(key);
                                    }
                                } else if audit {
                                    report.skip(key, SkipReason::InvalidType);
                                }
                            },
                            "CFBundleShortVersionString" => {
                                if let &Plist::String(ref s) = value {
                                    self.version = s.to_owned();
                                    if audit {
                                        report.parsed(key);
                                    }
                                } else if audit {
                                    report.skip(key, SkipReason::InvalidType);
                                }
                            },
                            _ => {
                                if audit {
                                    report.ignore(key);
                                }
                            },
                        }
                    }
                    if audit {
                        self.auditor.as_mut().unwrap().parsed(
                            &plist_path.to_string_lossy(), report);
                    }

                },
                _ => {
                    if audit {
                        self.auditor.as_mut().unwrap().skip(
                            &plist_path.to_string_lossy(),
                            SkipReason::InvalidType);
                    }
                }
            }

        }
        &self.version
    }

    /// Build a path from the bundle root.
    /// If database is true, will be from the Database subdirectory.
    fn build_path(&self, dir: &str, database: bool) -> PathBuf
    {
        let mut ppath = PathBuf::from(self.path.to_owned());
        if database {
            ppath.push(DATABASE_DIR);
        }
        ppath.push(dir);

        ppath
    }

    /// list items in dir with extension ext.
    /// Return a vector with full path for each.
    fn list_items(&self, dir: &str, ext: &str) -> Vec<PathBuf>
    {
        let ppath = self.build_path(dir, true);
        let mut list: Vec<PathBuf> = Vec::new();

        if !fs::metadata(&ppath).unwrap().is_dir() {
            // XXX return a Result
            return list;
        }

        for entry in fs::read_dir(&ppath).unwrap() {
            let p = entry.unwrap().path();
            if p.extension().unwrap() == ext {
                list.push(p.to_owned());
            }
        }

        list
    }

    pub fn get_model_info(&self) -> Option<ModelInfo>
    {
        let ppath = self.build_path(DATAMODEL_VERSION_PLIST, true);
        let plist = plutils::parse_plist(ppath.as_ref());

        ModelInfo::parse(&plist)
    }

    fn load_items<T: AplibObject + Auditable, F: FnMut(u64) -> bool>(
        &mut self, dir: &str, ext: &str, set: &mut HashSet<String>, pg: &mut F)
    {
        let file_list = self.list_items(dir, ext);
        let audit = self.auditor.is_some();
        for file in file_list {
            if let Some(obj) = T::from_path(file.as_ref()) {
                let mut store = false;
                if let Some(ref uuid) = *obj.uuid() {
                    set.insert(uuid.to_owned());
                    if audit {
                        let report = obj.audit();
                        self.auditor.as_mut().unwrap().parsed(
                            &file.to_string_lossy(), report);
                    }
                    store = true;
                }
                if store {
                    self.store(T::wrap(obj));
                }
            } else {
                if audit {
                    self.auditor.as_mut().unwrap().skip(
                        &file.to_string_lossy(),
                        SkipReason::ParseFailed);
                }
                println!("Failed to decode object from {:?}", file);
            }
            if !pg(1) {
                println!("Cancelled");
                return;
            }
        }
    }

    pub fn load_albums<F: FnMut(u64) -> bool>(&mut self, pg: &mut F)
    {
        if self.albums.is_empty() {
            let mut albums : HashSet<String> = HashSet::new();
            self.load_items::<Album, F>(ALBUMS_DIR, "apalbum", &mut albums, pg);
            self.albums = albums;
        }
    }

    pub fn get_albums(&self) -> &HashSet<String>
    {
        &self.albums
    }

    pub fn load_folders<F: FnMut(u64) -> bool>(&mut self, pg: &mut F)
    {
        if self.folders.is_empty() {
            let mut folders : HashSet<String> = HashSet::new();
            self.load_items::<Folder, F>(FOLDERS_DIR, "apfolder",
                                         &mut folders, pg);
            self.folders = folders;
        }
    }

    pub fn get_folders(&self) -> &HashSet<String>
    {
        &self.folders
    }

    fn recurse_list_directory(path: &Path, level: i32) -> Vec<PathBuf>
    {
        let mut list: Vec<PathBuf> = Vec::new();
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            if entry.metadata().unwrap().is_dir() {
                if level == 0 {
                    list.push(entry.path());
                } else {
                    let mut sublist =
                        Library::recurse_list_directory(&entry.path(),
                                                        level - 1);
                    list.append(&mut sublist)
                }
            }
        }

        list
    }

    fn list_versions_items_dirs(&self) -> Vec<PathBuf>
    {
        let ppath = self.build_path(VERSIONS_BASE_DIR, true);

        if !fs::metadata(&ppath).unwrap().is_dir() {
            // XXX return a Result
            return Vec::new();
        }

        Library::recurse_list_directory(&ppath, 4)
    }

    fn list_versions_items(&self, ext: &str) -> Vec<PathBuf>
    {
        let list = self.list_versions_items_dirs();
        let mut items = Vec::new();

        for dir in list {

            if !fs::metadata(&dir).unwrap().is_dir() {
                continue;
            }

            for entry in fs::read_dir(&dir).unwrap() {
                let entry = entry.unwrap();
                let p = entry.path();
                if p.extension().unwrap() == ext {
                    items.push(entry.path().to_owned());
                }
            }
        }

        items
    }

    fn load_versions_items<T: AplibObject, F: FnMut(u64) -> bool>(
        &mut self, ext: &str,
        set: &mut HashSet<String>, pg: &mut F)
    {
        let file_list = self.list_versions_items(ext);
        for file in file_list {
            if let Some(obj) = T::from_path(file.as_ref()) {
                let mut store = false;
                if let Some(ref uuid) = *obj.uuid() {
                    set.insert(uuid.to_owned());
                    store = true;
                }
                if store {
                    self.store(T::wrap(obj));
                }
            } else {
                println!("Error decoding object from {:?}", file);
            }
            if !pg(1) {
                println!("Cancelled!");
                break;
            }
        }
    }

    pub fn load_versions<F: FnMut(u64) -> bool>(&mut self, pg: &mut F)
    {
        if self.versions.is_empty() {
            let mut versions: HashSet<String> = HashSet::new();
            self.load_versions_items::<Version, F>(
                "apversion", &mut versions, pg);
            self.versions = versions;
        }
    }

    pub fn load_masters<F: FnMut(u64) -> bool>(&mut self, pg: &mut F)
    {
        if self.masters.is_empty() {
            let mut masters: HashSet<String> = HashSet::new();
            self.load_versions_items::<Master, F>(
                "apmaster", &mut masters, pg);
            self.masters = masters;
        }
    }

    pub fn get_masters(&self) -> &HashSet<String>
    {
        &self.masters
    }

    pub fn get_versions(&self) -> &HashSet<String>
    {
        &self.versions
    }

    pub fn list_keywords(&self) -> Option<Vec<Keyword>>
    {
        parse_keywords(self.build_path(KEYWORDS_PLIST, true).as_ref())
    }
}

