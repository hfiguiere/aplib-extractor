/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

mod aplib;

use std::env;
use aplib::library::Library;
//use aplib::folder::Folder;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let mut library = Library::new(args[1].clone());

        {
            let version = library.library_version();
            println!("Version {}", version);
        }

        let model_info = library.get_model_info();

        println!("model info");
        println!("\tDB version: {}", model_info.db_version);
        println!("\tDB minor version: {}", model_info.db_minor_version);
        println!("\tProject version: {}", model_info.project_version);

        let count = library.count_albums();
        println!("{} albums", count);

        let folder_count = library.count_folders();
        println!("{} folder", folder_count);

        let folders = library.list_folders();
        println!("Folders:");
        println!("| Name | uuid | type | model id | path |");
        for folder in folders {
            println!("| {} | {} | {} | {} | {} |",
                     folder.name, folder.uuid, folder.folder_type,
                     folder.model_id, folder.path);
        }
    } else {
        println!("Argument required");
    }
}
