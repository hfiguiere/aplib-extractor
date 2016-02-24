/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

mod aplib;

use std::env;
use aplib::AplibObject;
use aplib::library::Library;
use aplib::keyword::Keyword;
use aplib::wrapper::ObjectStoreWrapper;

/// print the keywords with indentation for the hierarchy
fn print_keywords(keywords: &Vec<Keyword>, indent: &String) {
    for keyword in keywords {
        println!("| {}{} | {} | {} |", indent, keyword.name,
                 keyword.uuid(), keyword.parent());
        if !keyword.children.is_empty() {
            let new_indent;
            if indent.is_empty() {
                new_indent = "+- ".to_string() + indent;
            } else {
                new_indent = "\t".to_string() + indent;
            }
            print_keywords(&keyword.children, &new_indent);
        }
    }
}

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let mut library = Library::new(&args[1]);

        {
            let version = library.library_version();
            println!("Version {}", version);
        }

        let model_info = library.get_model_info();

        println!("model info");
        println!("\tDB version: {}", model_info.db_version);
        println!("\tDB minor version: {}", model_info.db_minor_version);
        println!("\tProject version: {}", model_info.project_version);

        library.load_folders();
        library.load_albums();
        {
            let folders = library.get_folders();
            println!("{} Folders:", folders.len());
            println!("| Name | uuid | type | model id | path |");
            for folder_uuid in folders {
                if folder_uuid.is_empty() {
                    continue;
                }
                match library.get(folder_uuid) {
                    Some(&ObjectStoreWrapper::Folder(ref folder)) =>
                        println!("| {} | {} | {} | {} | {} |",
                                 folder.name, folder.uuid(),
                                 folder.folder_type,
                                 folder.model_id(), folder.path),
                    _ => println!("folder {} not found", folder_uuid)
                }
            }
        }
        {
            let albums = library.get_albums();
            println!("{} Albums:", albums.len());
            println!("| name | uuid | folder | type | class | model id |");
            for album_uuid in albums {
                if album_uuid.is_empty() {
                    continue;
                }
                match library.get(album_uuid) {
                    Some(&ObjectStoreWrapper::Album(ref album)) =>
                        println!("| {} | {} | {} | {} | {} | {} |",
                                 album.name,
                                 album.uuid(), album.parent(),
                                 album.album_type,
                                 album.subclass, album.model_id()),
                    _ => println!("album {} not found", album_uuid)
                }
            }
        }
        let keywords = library.list_keywords();
        println!("{} keywords:", keywords.len());
        println!("| uuid | parent | name |");
        print_keywords(&keywords, &"".to_string());

        library.load_masters();
        library.load_versions();
        {
            let masters = library.get_masters();
            println!("{} Masters:", masters.len());
            println!("| uuid | project | path |");
            for master_uuid in masters {
                if master_uuid.is_empty() {
                    continue;
                }
                match library.get(master_uuid) {
                    Some(&ObjectStoreWrapper::Master(ref master)) =>
                        println!("| {} | {} | {} |",
                                 master.uuid(), master.parent(),
                                 master.image_path),
                    _ => println!("master {} not found", master_uuid)
                }
            }

        }
        {
            let versions = library.get_versions();
            println!("{} Versions:", versions.len());
            println!("| uuid | master | project | name | original |");
            for version_uuid in versions {
                if version_uuid.is_empty() {
                    continue;
                }
                match library.get(version_uuid) {
                    Some(&ObjectStoreWrapper::Version(ref version)) =>
                                println!("| {} | {} | {} | {} | {} |",
                                         version.uuid(), version.parent(),
                                         version.project_uuid, version.name,
                                         version.is_original),
                    _ => println!("version {} not found", version_uuid)
                }
            }
        }
    } else {
        println!("Argument required");
    }
}
