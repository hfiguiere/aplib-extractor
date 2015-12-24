extern crate plist;

use std::fs::File;
use std::path::Path;

use self::plist::PlistEvent;

pub enum FolderType {
    INVALID = 0,
    FOLDER = 1,
    PROJECT = 2,
}

pub struct Folder {
    pub uuid: String,
    pub model_id: i64,
    pub folder_type: u64,
    db_version: i64,
    project_version: i64,
    pub path: String,
    pub name: String,
    parent_uuid: String,
}

impl Folder {

    fn parse_plist(path : &str) -> Vec<PlistEvent>
    {
        use self::plist::StreamingParser;

        let ppath = Path::new(path);
        let f = File::open(&ppath).unwrap();

        let streaming_parser = StreamingParser::new(f);

        streaming_parser.map(|e| e.unwrap()).collect()
    }

    pub fn from(plist_path: &str) -> Folder
    {
        let mut folder = Folder { uuid: "".to_string(),
                                  model_id: 0, folder_type: 0,
                                  db_version: 0, project_version: 0,
                                  path: "".to_string(), name: "".to_string(),
                                  parent_uuid: "".to_string() };

        let events = Folder::parse_plist(plist_path);

        let mut i = 0usize;
        loop {
            match events[i] {
                PlistEvent::Key(ref s) => {
                    match s.as_ref() {
                        "folderPath" =>
                            match events[i + 1] {
                                PlistEvent::StringValue(ref s) => {
                                    folder.path = s.clone();
                                },
                                _ => ()
                            },
                        "folderType" =>
                            match events[i + 1] {
                                PlistEvent::IntegerValue(n) => {
                                    folder.folder_type = n as u64;
                                }
                                _ => ()
                            },
                        "modelId" =>
                            match events[i + 1] {
                                PlistEvent::IntegerValue(n) => {
                                    folder.model_id = n;
                                }
                                _ => ()
                            },
                        "name" =>
                            match events[i + 1] {
                                PlistEvent::StringValue(ref s) => {
                                    folder.name = s.clone();
                                },
                                _ => ()
                            },
                        "parentFolderUuid" =>
                            match events[i + 1] {
                                PlistEvent::StringValue(ref s) => {
                                    folder.parent_uuid = s.clone();
                                },
                                _ => ()
                            },
                        "uuid" =>
                            match events[i + 1] {
                                PlistEvent::StringValue(ref s) => {
                                    folder.uuid = s.clone();
                                },
                                _ => ()
                            },
                        "version" =>
                            match events[i + 1] {
                                PlistEvent::IntegerValue(n) => {
                                    folder.db_version = n;
                                }
                                _ => ()
                            },
                        "projectVersion" =>
                            match events[i + 1] {
                                PlistEvent::IntegerValue(n) => {
                                    folder.project_version = n;
                                }
                                _ => ()
                            },

                        _ => i += 1
                    }
                },
                _ => ()
            }
            i += 1;
            if i >= events.len() {
                break;
            }
        }
        folder
    }

}
