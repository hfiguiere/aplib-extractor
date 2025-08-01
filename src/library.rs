/*
 This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use once_cell::unsync::OnceCell;
use plist::Value;

use crate::album::Album;
use crate::audit::{audit_get_str_value, Report, Reporter, SkipReason};
use crate::folder::Folder;
use crate::keyword::{parse_keywords, Keyword};
use crate::master::Master;
use crate::plutils;
use crate::store;
use crate::version::Version;
use crate::volume::Volume;
use crate::{AplibObject, PlistLoadable, SqliteLoadable};

// This is mostly from db_version = 110

const INFO_PLIST: &str = "Info.plist";

const BUNDLE_IDENTIFIER: &str = "com.apple.Aperture.library";

const DATABASE_DIR: &str = "Database";

// in Database
const DATAMODEL_VERSION_PLIST: &str = "DataModelVersion.plist";
const KEYWORDS_PLIST: &str = "Keywords.plist";
const ALBUMS_DIR: &str = "Albums";
const FOLDERS_DIR: &str = "Folders";
const VOLUMES_DIR: &str = "Volumes";
const VERSIONS_BASE_DIR: &str = "Versions";

pub const PROGRESS_NONE: Option<fn(u64) -> bool> = None;

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
    fn parse(plist: &Value) -> Option<ModelInfo> {
        use crate::plutils::{get_bool_value, get_int_value, get_str_value};

        match *plist {
            Value::Dictionary(ref dict) => Some(ModelInfo {
                db_uuid: get_str_value(dict, "databaseUuid"),
                db_minor_back_compatible_version: get_int_value(
                    dict,
                    "DatabaseCompatibleBackToMinorVersion",
                ),
                db_minor_version: get_int_value(dict, "DatabaseMinorVersion"),
                db_version: get_int_value(dict, "DatabaseVersion"),
                is_iphoto_library: get_bool_value(dict, "isIPhotoLibrary"),
                create_date: get_str_value(dict, "createDate"),
                image_io_version: get_str_value(dict, "imageIOVersion"),
                raw_camera_bundle_version: get_str_value(dict, "rawCameraBundleVersion"),
                touched_by_aperture: get_bool_value(dict, "touchedByAperture"),
                master_count: get_int_value(dict, "masterCount"),
                version_count: get_int_value(dict, "versionCount"),
                project_version: get_int_value(dict, "projectVersion"),
                project_compat_back_to_version: get_int_value(
                    dict,
                    "projectCompatibleBackToVersion",
                ),
            }),
            _ => None,
        }
    }
}

/// Library is the Aperture library.
pub struct Library {
    /// The path to the .aplib bundle (the directory)
    path: PathBuf,

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
    /// All the volumes UUID
    volumes: HashSet<String>,

    /// The object store. The key is the UUID
    objects: HashMap<String, store::Wrapper>,
    /// Auditor for the audit mode.
    auditor: Option<Reporter>,
    /// Database connection
    database_conn: OnceCell<Option<rusqlite::Connection>>,
}

impl Library {
    /// Create a new library object from the exist path to
    /// the bundle directory.
    pub fn new<P>(p: P) -> Library
    where
        P: AsRef<std::path::Path>,
    {
        Library {
            path: p.as_ref().to_path_buf(),
            version: String::new(),

            folders: HashSet::new(),
            albums: HashSet::new(),
            //            keywords: HashSet::new(),
            masters: HashSet::new(),
            versions: HashSet::new(),
            volumes: HashSet::new(),

            objects: HashMap::new(),
            auditor: None,

            database_conn: OnceCell::new(),
        }
    }

    /// Set an auditor.
    pub fn set_auditor(&mut self, auditor: Option<Reporter>) {
        self.auditor = auditor;
    }
    /// Get the auditor
    pub fn auditor(&self) -> Option<&Reporter> {
        self.auditor.as_ref()
    }

    /// Get the main database from the library.
    pub fn database(&self) -> &Option<rusqlite::Connection> {
        self.database_conn.get_or_init(|| {
            let dbpath = self.path.join("Database/apdb/Library.apdb");
            let connection = rusqlite::Connection::open(dbpath);
            connection.ok()
        })
    }

    /// Store the wrapped object.
    /// Return true if the object was stored
    /// Return false if there already was an object with the same uuid
    /// or if the uuid in invalid.
    pub fn store(&mut self, obj: store::Wrapper) -> bool {
        if let Some(uuid_str) = obj.uuid() {
            self.objects.insert(uuid_str, obj).is_none()
        } else {
            false
        }
    }

    /// Get an object out of the store by UUID
    pub fn get(&self, uuid: &str) -> Option<&store::Wrapper> {
        self.objects.get(uuid)
    }

    /// Get the library version. Will parse the plist for that
    /// if needed.
    pub fn library_version(&mut self) -> Result<&String, SkipReason> {
        if self.version.is_empty() {
            let plist_path = self.build_path(INFO_PLIST, false);
            let plist = plutils::parse_plist(&plist_path);
            let audit = self.auditor.is_some();
            let mut report = if audit { Some(Report::new()) } else { None };

            match plist {
                Value::Dictionary(ref dict) => {
                    let version = audit_get_str_value(
                        dict,
                        "CFBundleShortVersionString",
                        &mut report.as_mut(),
                    );
                    if version.is_none() {
                        println!("FATAL no library version found");
                        return Err(SkipReason::NotFound);
                    }
                    self.version = version.unwrap();

                    let bundle_id =
                        audit_get_str_value(dict, "CFBundleIdentifier", &mut report.as_mut());
                    if let Some(id) = bundle_id {
                        if id != BUNDLE_IDENTIFIER {
                            if audit {
                                if let Some(ref mut r) = report {
                                    r.skip("CFBundleIdentifier", SkipReason::InvalidData);
                                }
                            }
                            println!("FATAL not a library");
                            return Err(SkipReason::InvalidData);
                        }
                    } else if audit {
                        if let Some(ref mut r) = report {
                            r.skip("CFBundleIdentifier", SkipReason::NotFound);
                        }
                        println!("FATAL no bundle identifier");
                        return Err(SkipReason::NotFound);
                    }

                    if audit {
                        if let Some(ref mut r) = report {
                            r.audit_ignored(dict, None);
                        }
                        self.auditor
                            .as_mut()
                            .unwrap()
                            .parsed(&plist_path.to_string_lossy(), report.unwrap());
                    }
                }
                _ => {
                    if audit {
                        self.auditor
                            .as_mut()
                            .unwrap()
                            .skip(&plist_path.to_string_lossy(), SkipReason::InvalidType);
                    }
                }
            }
        }
        Ok(&self.version)
    }

    /// Build a path from the bundle root.
    /// If database is true, will be from the Database subdirectory.
    fn build_path(&self, dir: &str, database: bool) -> PathBuf {
        let mut ppath = self.path.clone();
        if database {
            ppath.push(DATABASE_DIR);
        }
        ppath.push(dir);

        ppath
    }

    /// list items in dir with extension ext.
    /// Return a vector with full path for each.
    fn list_items(&self, dir: &str, ext: &str) -> Vec<PathBuf> {
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

    /// Return the model info block
    pub fn get_model_info(&self) -> Option<ModelInfo> {
        let ppath = self.build_path(DATAMODEL_VERSION_PLIST, true);
        let plist = plutils::parse_plist(ppath);

        ModelInfo::parse(&plist)
    }

    /// Load items from directory `dir` with extension `ext`
    /// and store the uuids into `set`
    fn load_items<T, F>(
        &mut self,
        dir: &str,
        ext: &str,
        set: &mut HashSet<String>,
        mut pg: Option<F>,
    ) where
        T: PlistLoadable + AplibObject,
        F: FnMut(u64) -> bool,
    {
        let file_list = self.list_items(dir, ext);
        let audit = self.auditor.is_some();
        for file in file_list {
            let mut report = if audit { Some(Report::new()) } else { None };
            if let Some(obj) = T::from_path(&file, report.as_mut()) {
                let mut store = false;
                if let Some(ref uuid) = *obj.uuid() {
                    set.insert(uuid.to_owned());
                    if audit {
                        self.auditor
                            .as_mut()
                            .unwrap()
                            .parsed(&file.to_string_lossy(), report.unwrap());
                    }
                    store = true;
                }
                if store {
                    self.store(T::wrap(obj));
                }
            } else {
                if audit {
                    self.auditor
                        .as_mut()
                        .unwrap()
                        .skip(&file.to_string_lossy(), SkipReason::ParseFailed);
                }
                println!("Failed to decode object from {:?}", file);
            }
            if let Some(pg) = pg.as_mut() {
                if !pg(1) {
                    println!("Cancelled");
                    return;
                }
            }
        }
    }

    /// Load albums. Once done the result it cached.
    pub fn load_albums<F: FnMut(u64) -> bool>(&mut self, pg: Option<F>) {
        if self.albums.is_empty() {
            let mut albums: HashSet<String> = HashSet::new();
            self.load_items::<Album, F>(ALBUMS_DIR, "apalbum", &mut albums, pg);
            self.albums = albums;
        }
    }

    /// Get albums uuids.
    pub fn albums(&self) -> &HashSet<String> {
        &self.albums
    }

    /// Load folders. Once done the result is cached.
    pub fn load_folders<F: FnMut(u64) -> bool>(&mut self, pg: Option<F>) {
        if self.folders.is_empty() {
            let mut folders: HashSet<String> = HashSet::new();
            self.load_items::<Folder, F>(FOLDERS_DIR, "apfolder", &mut folders, pg);
            self.folders = folders;
        }
    }

    /// Get folders uuids.
    pub fn folders(&self) -> &HashSet<String> {
        &self.folders
    }

    fn recurse_list_directory(path: &Path, level: i32) -> Vec<PathBuf> {
        let mut list: Vec<PathBuf> = Vec::new();
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            if entry.metadata().unwrap().is_dir() {
                if level == 0 {
                    list.push(entry.path());
                } else {
                    let mut sublist = Library::recurse_list_directory(&entry.path(), level - 1);
                    list.append(&mut sublist)
                }
            }
        }

        list
    }

    fn list_items_dirs(&self, dir: &str) -> Vec<PathBuf> {
        let ppath = self.build_path(dir, true);

        if !fs::metadata(&ppath).unwrap().is_dir() {
            // XXX return a Result
            return Vec::new();
        }

        Library::recurse_list_directory(&ppath, 4)
    }

    // XXX shall this a list_items() be merged?
    fn list_recursive_items(&self, dir: &str, ext: &str) -> Vec<PathBuf> {
        let list = self.list_items_dirs(dir);
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

    fn load_volumes_items<T, F>(&mut self, ext: &str, set: &mut HashSet<String>, mut pg: Option<F>)
    where
        T: PlistLoadable + SqliteLoadable + AplibObject,
        F: FnMut(u64) -> bool,
    {
        use rusqlite::params;

        let file_list = self.list_items(VOLUMES_DIR, ext);
        if file_list.is_empty() {
            // open the database and load from there.
            let mut objects = Vec::new();
            if let Some(conn) = self.database() {
                let query = format!("SELECT {} FROM {}", T::columns(), T::tables());
                if let Ok(mut stmt) = conn.prepare(&query) {
                    if let Ok(volumes) = stmt.query_and_then(params![], |row| T::from_row(row)) {
                        volumes
                            .into_iter()
                            .filter(|vol| vol.is_ok())
                            .for_each(|vol| {
                                let vol = vol.unwrap();
                                if let Some(uuid) = vol.uuid() {
                                    set.insert(uuid.clone());
                                    objects.push(vol);
                                }
                            });
                    }
                }
            }

            objects.into_iter().for_each(|vol| {
                self.store(T::wrap(vol));
            });

            return;
        }
        let audit = self.auditor.is_some();
        for file in file_list {
            let mut report = if audit { Some(Report::new()) } else { None };
            if let Some(obj) = T::from_path(&file, report.as_mut()) {
                let mut store = false;
                if let Some(ref uuid) = *obj.uuid() {
                    set.insert(uuid.to_owned());
                    store = true;
                    if audit {
                        self.auditor
                            .as_mut()
                            .unwrap()
                            .parsed(&file.to_string_lossy(), report.unwrap());
                    }
                }
                if store {
                    self.store(T::wrap(obj));
                }
            } else {
                if audit {
                    self.auditor
                        .as_mut()
                        .unwrap()
                        .skip(&file.to_string_lossy(), SkipReason::ParseFailed);
                }
                println!("Error decoding object from {:?}", file);
            }
            if let Some(pg) = pg.as_mut() {
                if !pg(1) {
                    println!("Cancelled!");
                    break;
                }
            }
        }
    }

    fn load_versions_items<T, F>(&mut self, ext: &str, set: &mut HashSet<String>, mut pg: Option<F>)
    where
        T: PlistLoadable + AplibObject,
        F: FnMut(u64) -> bool,
    {
        let file_list = self.list_recursive_items(VERSIONS_BASE_DIR, ext);
        let audit = self.auditor.is_some();
        for file in file_list {
            let mut report = if audit { Some(Report::new()) } else { None };
            if let Some(obj) = T::from_path(&file, report.as_mut()) {
                let mut store = false;
                if let Some(ref uuid) = *obj.uuid() {
                    set.insert(uuid.to_owned());
                    store = true;
                    if audit {
                        self.auditor
                            .as_mut()
                            .unwrap()
                            .parsed(&file.to_string_lossy(), report.unwrap());
                    }
                }
                if store {
                    self.store(T::wrap(obj));
                }
            } else {
                if audit {
                    self.auditor
                        .as_mut()
                        .unwrap()
                        .skip(&file.to_string_lossy(), SkipReason::ParseFailed);
                }
                println!("Error decoding object from {:?}", file);
            }
            if let Some(pg) = pg.as_mut() {
                if !pg(1) {
                    println!("Cancelled!");
                    break;
                }
            }
        }
    }

    /// Load volumess.
    pub fn load_volumes<F: FnMut(u64) -> bool>(&mut self, pg: Option<F>) {
        if self.volumes.is_empty() {
            let mut volumes: HashSet<String> = HashSet::new();
            self.load_volumes_items::<Volume, F>("apvolume", &mut volumes, pg);
            self.volumes = volumes;
        }
    }

    /// Load versions.
    pub fn load_versions<F: FnMut(u64) -> bool>(&mut self, pg: Option<F>) {
        if self.versions.is_empty() {
            let mut versions: HashSet<String> = HashSet::new();
            self.load_versions_items::<Version, F>("apversion", &mut versions, pg);
            self.versions = versions;
        }
    }

    /// Load masters.
    pub fn load_masters<F: FnMut(u64) -> bool>(&mut self, pg: Option<F>) {
        if self.masters.is_empty() {
            let mut masters: HashSet<String> = HashSet::new();
            self.load_versions_items::<Master, F>("apmaster", &mut masters, pg);
            self.masters = masters;
        }
    }

    /// Return masters uuids.
    pub fn masters(&self) -> &HashSet<String> {
        &self.masters
    }

    /// Return versions uuids.
    pub fn versions(&self) -> &HashSet<String> {
        &self.versions
    }

    /// Return volumes uuids.
    pub fn volumes(&self) -> &HashSet<String> {
        &self.volumes
    }

    /// Resolve the path of a master to it's macOS on disk location
    /// either to an existing volume or relative to the library.
    pub fn resolve_master_path(&self, uuid: &str) -> Option<String> {
        match self.get(uuid) {
            Some(crate::StoreWrapper::Master(master)) => {
                let image_path = master.image_path.as_ref().unwrap();
                if let Some(volume_uuid) = master.file_volume_uuid.as_ref() {
                    self.get(volume_uuid).and_then(|object| {
                        if let crate::StoreWrapper::Volume(volume) = object {
                            Some(format!(
                                "/Volumes/{}/{image_path}",
                                volume.volume_name.clone().unwrap_or_else(String::default)
                            ))
                        } else {
                            None
                        }
                    })
                } else {
                    Some(format!("Masters/{image_path}"))
                }
            }
            _ => None,
        }
    }

    /// List keywords.
    pub fn list_keywords(&mut self) -> Option<Vec<Keyword>> {
        let audit = self.auditor.is_some();
        let mut report = if audit { Some(Report::new()) } else { None };
        let result = parse_keywords(self.build_path(KEYWORDS_PLIST, true), &mut report.as_mut());
        if audit {
            if result.is_some() {
                self.auditor
                    .as_mut()
                    .unwrap()
                    .parsed(KEYWORDS_PLIST, report.unwrap());
            } else {
                self.auditor
                    .as_mut()
                    .unwrap()
                    .skip(KEYWORDS_PLIST, SkipReason::ParseFailed);
            }
        }
        result
    }
}
