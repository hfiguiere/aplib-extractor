
mod aplib;

use std::env;
use aplib::library::Library;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let mut library = Library::new(args[1].clone());

        {
            let version = library.library_version();
            println!("Version {}", version);
        }

        let count = library.count_albums();
        println!("{} albums", count);

        let folder_count = library.count_folders();
        println!("{} folder", folder_count);

        let model_info = library.get_model_info();

        println!("model info");
        println!("\tDB version: {}", model_info.db_version);
        println!("\tDB minor version: {}", model_info.db_minor_version);
        println!("\tProject version: {}", model_info.project_version);
    } else {
        println!("Argument required");
    }
}
