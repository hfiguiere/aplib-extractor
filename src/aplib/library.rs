/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

extern crate plist;

use std::fs;
use std::path::PathBuf;
use std::collections::{HashMap,HashSet};

use self::plist::Plist;
use aplib::folder::Folder;
use aplib::album::Album;
use aplib::version::Version;
use aplib::master::Master;
use aplib::keyword::{parse_keywords,Keyword};
use aplib::AplibObject;
use aplib::store;
use aplib::plutils;

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

pub struct ModelInfo {
    pub is_iphoto_library: bool,
    pub db_version: i64,
    pub db_minor_version: i64,
    pub master_count: i64,
    pub version_count: i64,
    pub project_version: i64,
}

impl ModelInfo {
    fn parse(plist : &Plist) -> ModelInfo
    {
        use aplib::plutils::{get_int_value,get_bool_value};

        match *plist {
            Plist::Dictionary(ref dict) => ModelInfo {
                db_minor_version: get_int_value(dict,
                                                "DatabaseMinorVersion"),
                db_version: get_int_value(dict, "DatabaseVersion"),
                is_iphoto_library: get_bool_value(dict, "isIPhotoLibrary"),
                master_count: get_int_value(dict, "masterCount"),
                version_count: get_int_value(dict, "versionCount"),
                project_version: get_int_value(dict, "projectVersion")
            },
            _ => ModelInfo {
                is_iphoto_library: false,
                db_version: 0,
                db_minor_version: 0,
                master_count: 0,
                version_count: 0,
                project_version: 0
            }
        }
    }
}

pub struct Library {
    path: String,

    version: String,

    folders: HashSet<String>,
    albums: HashSet<String>,
//    keywords: HashSet<String>,
    masters: HashSet<String>,
    versions: HashSet<String>,

    objects: HashMap<String, store::Wrapper>,
}

impl Library {

    pub fn new(p: &String) -> Library
    {
        Library {
            path: p.to_owned(),
            version: "".to_string(),

            folders: HashSet::new(),
            albums: HashSet::new(),
//            keywords: HashSet::new(),
            masters: HashSet::new(),
            versions: HashSet::new(),

            objects: HashMap::new(),
        }
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
        return match self.objects.insert(uuid_str, obj) {
            None => true,
            _ => false,
        };
    }

    pub fn get(&self, uuid: &String) -> Option<&store::Wrapper>
    {
        self.objects.get(uuid)
    }

    pub fn library_version(&mut self) -> &String
    {
        if self.version.is_empty()
        {
            let plist = plutils::parse_plist(self.build_path(INFO_PLIST,
                                                             false).as_ref());

            match plist {
                Plist::Dictionary(ref dict) => {
                    let bundle_id = plutils::get_str_value(dict,
                                                           "CFBundleIdentifier");
                    if bundle_id != BUNDLE_IDENTIFIER {
                        println!("FATAL not a library");
                        self.version = "".to_string();
                        return &self.version;
                    }
                    self.version =
                        plutils::get_str_value(dict,
                                               "CFBundleShortVersionString");
                },
                _ => ()
            }
        }
        return &self.version;
    }

    fn build_path(&self, dir: &str, database: bool) -> PathBuf
    {
        let mut ppath = PathBuf::from(self.path.to_owned());
        if database {
            ppath.push(DATABASE_DIR);
        }
        ppath.push(dir);

        return ppath;
    }

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
        return list;
    }

    pub fn get_model_info(&self) -> ModelInfo
    {
        let ppath = self.build_path(DATAMODEL_VERSION_PLIST, true);
        let plist = plutils::parse_plist(ppath.as_ref());

        return ModelInfo::parse(&plist);
    }

    fn load_items<T: AplibObject>(&mut self, dir: &str, ext: &str,
                                  set: &mut HashSet<String>)
    {
        let file_list = self.list_items(dir, ext);
        for file in file_list {
            let obj = T::from_path(file.as_ref());
            set.insert(obj.uuid().to_owned());
            self.store(T::wrap(obj));
        }
    }

    pub fn load_albums(&mut self)
    {
        if self.albums.is_empty() {
            let mut albums : HashSet<String> = HashSet::new();
            self.load_items::<Album>(ALBUMS_DIR, "apalbum", &mut albums);
            self.albums = albums;
        }
    }

    pub fn get_albums(&self) -> &HashSet<String>
    {
        return &self.albums;
    }

    pub fn load_folders(&mut self)
    {
        if self.folders.is_empty() {
            let mut folders : HashSet<String> = HashSet::new();
            self.load_items::<Folder>(FOLDERS_DIR, "apfolder", &mut folders);
            self.folders = folders;
        }
    }

    pub fn get_folders(&self) -> &HashSet<String>
    {
        return &self.folders;
    }

    fn recurse_list_directory(path: &PathBuf, level: i32) -> Vec<PathBuf>
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
        return list;
    }

    fn list_versions_items_dirs(&self) -> Vec<PathBuf>
    {
        let ppath = self.build_path(VERSIONS_BASE_DIR, true);

        if !fs::metadata(&ppath).unwrap().is_dir() {
            // XXX return a Result
            return Vec::new();
        }

        return Library::recurse_list_directory(&ppath, 4);
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
        return items;
    }

    fn load_versions_items<T: AplibObject>(&mut self, ext: &str,
                                           set: &mut HashSet<String>)
    {
        let file_list = self.list_versions_items(ext);
        for file in file_list {
            let obj = T::from_path(file.as_ref());
            set.insert(obj.uuid().to_owned());
            self.store(T::wrap(obj));
        }
    }

    pub fn load_versions(&mut self)
    {
        if self.versions.is_empty() {
            let mut versions: HashSet<String> = HashSet::new();
            self.load_versions_items::<Version>("apversion", &mut versions);
            self.versions = versions;
        }
    }

    pub fn load_masters(&mut self)
    {
        if self.masters.is_empty() {
            let mut masters: HashSet<String> = HashSet::new();
            self.load_versions_items::<Master>("apmaster", &mut masters);
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

    pub fn list_keywords(&self) -> Vec<Keyword>
    {
        return parse_keywords(self.build_path(KEYWORDS_PLIST, true).as_ref());
    }
}

