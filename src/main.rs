
mod aplib;

use std::env;
use aplib::library::Library;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let mut library = Library::new(args[1].clone());

        let version = library.library_version();
        println!("Version {}", version);

        let count = library.count_albums();
        println!("{} albums", count);

        let folder_count = library.count_folders();
        println!("{} folder", folder_count);

    } else {
        println!("Argument required");
    }
}
