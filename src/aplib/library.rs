
extern crate plist;

use std::fs::File;
use std::fs;
use std::path::{Path,PathBuf};

use self::plist::Plist;
use aplib::folder::Folder;
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

    pub fn new(p: String) -> Library
    {
        Library { path: p, version: "".to_string() }
    }

    fn parse_plist(&self, path : &Path) -> Plist
    {
        let mut ppath = PathBuf::from(self.path.to_owned());
        ppath.push(path);

        let f = File::open(&ppath).unwrap();
        Plist::read(f).unwrap()
    }

    pub fn library_version(&mut self) -> &String
    {
        if self.version.is_empty()
        {
            let plist = self.parse_plist(Path::new(INFO_PLIST));

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
        &self.version
    }

    fn list_items(&self, dir: &str, ext: &str) -> Vec<String>
    {
        let mut ppath = PathBuf::from(self.path.to_owned());
        ppath.push(DATABASE_DIR);
        ppath.push(dir);

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
        list
    }

    fn count_items(&self, dir: &str, ext: &str) -> u32
    {
        let mut ppath = PathBuf::from(self.path.to_owned());
        ppath.push(DATABASE_DIR);
        ppath.push(dir);

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
        count

    }

    pub fn get_model_info(&self) -> ModelInfo
    {
        let mut ppath = PathBuf::from(DATABASE_DIR);
        ppath.push(DATAMODEL_VERSION_PLIST);

        let plist = self.parse_plist(ppath.as_path());

        ModelInfo::parse(&plist)
    }

    pub fn count_albums(&self) -> u32
    {
        self.count_items(ALBUMS_DIR, "apalbum")
    }

    pub fn count_folders(&self) -> u32
    {
        self.count_items(FOLDERS_DIR, "apfolder")
    }


    pub fn list_folders(&self) -> Vec<Folder>
    {
        let file_list = self.list_items(FOLDERS_DIR, "apfolder");
        let mut folders: Vec<Folder> = Vec::new();
        for file in file_list {
            folders.push(Folder::from(&file));
        }
        folders
    }
}

