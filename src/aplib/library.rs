/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

extern crate plist;

use std::fs;
use std::path::PathBuf;

use self::plist::Plist;
use aplib::folder::Folder;
use aplib::album::Album;
use aplib::plutils;

// This is mostly from db_version = 110

const BUNDLE_IDENTIFIER: &'static str = "com.apple.Aperture.library";
const INFO_PLIST: &'static str = "Info.plist";
const DATABASE_DIR: &'static str = "Database";

const DATAMODEL_VERSION_PLIST: &'static str = "DataModelVersion.plist";
const ALBUMS_DIR: &'static str = "Albums";
const FOLDERS_DIR: &'static str = "Folders";

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
}

impl Library {

    pub fn new(p: &String) -> Library
    {
        Library { path: p.to_owned(), version: "".to_string() }
    }

    pub fn library_version(&mut self) -> &String
    {
        if self.version.is_empty()
        {
            let plist = plutils::parse_plist(self.build_path(INFO_PLIST, false).
                                             to_str().unwrap());

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

    fn list_items(&self, dir: &str, ext: &str) -> Vec<String>
    {
        let ppath = self.build_path(dir, true);
        let mut list: Vec<String> = Vec::new();

        if !fs::metadata(&ppath).unwrap().is_dir() {
            // XXX return a Result
            return list;
        }

        for entry in fs::read_dir(&ppath).unwrap() {
            let entry = entry.unwrap();
            let p = entry.path();
            if p.extension().unwrap() == ext {
                list.push(entry.path().to_str().unwrap().to_owned());
            }
        }
        return list;
    }

    fn count_items(&self, dir: &str, ext: &str) -> u32
    {
        let ppath = self.build_path(dir, true);

        let mut count = 0u32;
        if !fs::metadata(&ppath).unwrap().is_dir() {
            return 0;
        }

        for entry in fs::read_dir(&ppath).unwrap() {
            let entry = entry.unwrap();
            let p = entry.path();
            if p.extension().unwrap() == ext {
                count += 1;
            }
        }
        return count;
    }

    pub fn get_model_info(&self) -> ModelInfo
    {
        let ppath = self.build_path(DATAMODEL_VERSION_PLIST, true);
        let plist = plutils::parse_plist(ppath.to_str().unwrap());

        return ModelInfo::parse(&plist);
    }

    pub fn count_albums(&self) -> u32
    {
        return self.count_items(ALBUMS_DIR, "apalbum");
    }

    pub fn list_albums(&self) -> Vec<Album>
    {
        let file_list = self.list_items(ALBUMS_DIR, "apalbum");
        let mut albums: Vec<Album> = Vec::new();
        for file in file_list {
            albums.push(Album::from(&file));
        }
        return albums;
    }

    pub fn count_folders(&self) -> u32
    {
        return self.count_items(FOLDERS_DIR, "apfolder");
    }

    pub fn list_folders(&self) -> Vec<Folder>
    {
        let file_list = self.list_items(FOLDERS_DIR, "apfolder");
        let mut folders: Vec<Folder> = Vec::new();
        for file in file_list {
            folders.push(Folder::from(&file));
        }
        return folders;
    }
}

