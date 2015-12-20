
extern crate plist;

use std::io;
use std::fs::File;
use std::fs;
use std::path::PathBuf;


const INFO_PLIST: &'static str = "Info.plist";
const DATAMODEL_VERSION_PLIST: &'static str =
    "Aperture.aplib/DataModelVersion.plist";
const DATABASE_DIR: &'static str = "Database";
const ALBUMS_DIR: &'static str = "Albums";
const FOLDERS_DIR: &'static str = "Folders";

pub struct Library {
    path: String,

    version: String,
}

impl Library {

    pub fn new(p: String) -> Library
    {
        Library { path: p, version: "".to_string() }
    }

    pub fn library_version(&mut self) -> String
    {
        if self.version.is_empty()
        {
            use self::plist::PlistEvent;
            use self::plist::StreamingParser;

            let mut ppath = PathBuf::from(self.path.to_owned());
            ppath.push(INFO_PLIST);
            let f = File::open(&ppath).unwrap();

            let streaming_parser = StreamingParser::new(f);
	    let events: Vec<PlistEvent> =
                streaming_parser.map(|e| e.unwrap()).collect();

            let mut i =0usize;
            loop {
                match events[i] {
                    PlistEvent::Key(ref s) => {
                        if s == "CFBundleShortVersionString" {
                            match events[i+1] {
                                PlistEvent::StringValue(ref s) => {
                                    self.version = s.clone();
                                    break;
                                }
                                _ => ()
                            }
                        }
                    },
                    _ => ()
                }
                i+=1;
            }
        }
        self.version.clone()
    }

    fn count_items(&self, dir: &str, ext: &str) -> u32
    {
        let mut ppath = PathBuf::from(self.path.to_owned());
        ppath.push(DATABASE_DIR);
        ppath.push(dir);

        let mut count = 0u32;
        if !fs::metadata(ppath.to_owned()).unwrap().is_dir() {
            return 0;
        }

        for entry in fs::read_dir(ppath).unwrap() {
            let entry = entry.unwrap();
            let p = entry.path();
            if p.extension().unwrap() == ext {
                count += 1;
            }
        }
        count

    }

    pub fn count_albums(&mut self) -> u32
    {
        self.count_items(ALBUMS_DIR, "apalbum")
    }

    pub fn count_folders(&mut self) -> u32
    {
        self.count_items(FOLDERS_DIR, "apfolder")
    }

}

