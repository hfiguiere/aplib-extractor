/*
 This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

use std::collections::HashMap;

use clap::Parser;

use aplib::{AlbumSubclass, Library, PROGRESS_NONE};

type Tree = HashMap<String, Vec<String>>;

const TOP_LEVEL: &str = "LibraryFolder";

#[derive(Clone, Debug, Parser)]
pub(crate) struct TreeArgs {
    #[arg(long)]
    skip_masters: bool,
    path: String,
}

/// Print children for object with `uuid`.
fn print_children_for(uuid: &str, tree: &Tree, library: &Library, skip_masters: bool, indent: u32) {
    use aplib::StoreWrapper as Wrapper;

    if let Some(children) = tree.get(uuid) {
        let mut skipped_masters = 0;
        let mut skipped_versions = 0;
        for child in children {
            let obj = library.get(child).unwrap();
            if skip_masters {
                match obj {
                    Wrapper::Master(_) => {
                        skipped_masters += 1;
                        continue;
                    }
                    Wrapper::Version(_) => {
                        skipped_versions += 1;
                        continue;
                    }
                    _ => {}
                }
            }
            for _ in 0..indent {
                print!(" ");
            }
            let typ = match obj {
                Wrapper::Album(a) => {
                    if let Some(typ) = a.album_type {
                        if typ != 1 {
                            println!("ERROR: unknown type {typ}");
                        }
                    }
                    let subclass = match a.subclass {
                        Some(AlbumSubclass::Implicit) => "I",
                        Some(AlbumSubclass::Smart) => "S",
                        Some(AlbumSubclass::User) => "U",
                        Some(AlbumSubclass::Invalid) => "*",
                        _ => "",
                    };
                    format!("A{subclass}")
                }
                Wrapper::Folder(f) => {
                    let typ = match f.folder_type {
                        Some(aplib::FolderType::Folder) => "F",
                        Some(aplib::FolderType::Project) => "P",
                        Some(aplib::FolderType::Invalid) => "*",
                        _ => "",
                    };
                    format!("F{typ}")
                }
                Wrapper::Version(v) => {
                    let idx = v.version_number.unwrap_or(-1);
                    format!("V{idx}")
                }
                Wrapper::Master(_) => "M".to_string(),
                _ => "*".to_string(),
            };
            let name = match obj {
                Wrapper::Folder(f) => f.name.clone(),
                Wrapper::Album(a) => a.name.clone(),
                Wrapper::Version(v) => v.name.clone(),
                Wrapper::Master(m) => m.name.clone(),
                _ => None,
            }
            .unwrap_or_default();
            println!("[{typ}] {name}");
            print_children_for(child, tree, library, skip_masters, indent + 2);
        }
        if skip_masters && (skipped_masters != 0 || skipped_versions != 0) {
            for _ in 0..indent {
                print!(" ");
            }
            println!("(Skipped {skipped_masters} masters and {skipped_versions} versions.)");
        }
    }
}

/// Add an object to the tree.
fn add_object(uuid: &str, tree: &mut Tree, library: &Library) {
    if let Some(object) = library.get(uuid) {
        if let Some(parent) = object.parent_uuid() {
            if let Some(children) = tree.get_mut(&parent) {
                children.push(uuid.to_string());
            } else {
                tree.insert(parent.clone(), vec![uuid.to_string()]);
            }
        }
        if tree.get(uuid).is_none() {
            tree.insert(uuid.to_string(), vec![]);
        }
    } else {
        println!("ERROR: Object {uuid} not found");
    }
}

pub(crate) fn process_tree(args: &TreeArgs) {
    let mut library = Library::new(&args.path);
    library.load_folders(PROGRESS_NONE);
    library.load_albums(PROGRESS_NONE);
    library.load_masters(PROGRESS_NONE);
    library.load_versions(PROGRESS_NONE);

    let mut tree = Tree::new();
    let folders = library.folders();
    folders
        .iter()
        .for_each(|uuid| add_object(uuid, &mut tree, &library));
    let albums = library.albums();
    albums
        .iter()
        .for_each(|uuid| add_object(uuid, &mut tree, &library));
    let masters = library.masters();
    masters
        .iter()
        .for_each(|uuid| add_object(uuid, &mut tree, &library));
    let versions = library.versions();
    versions
        .iter()
        .for_each(|uuid| add_object(uuid, &mut tree, &library));
    println!("TOP LEVEL");
    print_children_for(TOP_LEVEL, &tree, &library, args.skip_masters, 2);
}
