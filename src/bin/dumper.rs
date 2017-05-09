/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

extern crate aplib;
extern crate docopt;
extern crate rustc_serialize;
extern crate pbr;

use docopt::Docopt;
use rustc_serialize::{Decodable, Decoder};
use pbr::ProgressBar;

use aplib::AplibObject;
use aplib::Library;
use aplib::Keyword;
use aplib::StoreWrapper;
use aplib::audit::Reporter;

const USAGE: &'static str = "
Usage:
  dumper <command> ([--all] | [--albums] [--versions] [--masters] [--folders] [--keywords]) <path>

Options:
    --all          Select all objects
    --albums       Select only albums
    --masters      Select only masters
    --folders      Select only folders
    --keywords     Select only keywords

Commands are:
    dump           Dump the objects
    audit          Audit mode: output what we ignored
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_command: Command,
    arg_path: String,
    flag_all: bool,
    flag_albums: bool,
    flag_versions: bool,
    flag_masters: bool,
    flag_folders: bool,
    flag_keywords: bool
}

#[derive(Debug)]
enum Command {
    Dump,
    Audit,
    Unknown(String)
}

impl Decodable for Command {
    fn decode<D: Decoder>(d: &mut D) -> Result<Command, D::Error> {
        let s = try!(d.read_str());
        Ok(match &*s {
            "dump" => Command::Dump,
            "audit" => Command::Audit,
            s => Command::Unknown(s.to_owned()),
        })
    }
}

fn main() {

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());
    {
        match args.arg_command {
            Command::Dump =>
                process_dump(&args),
            Command::Audit =>
                process_audit(&args),
            _ =>
                ()
        };
    }
}


fn process_audit(args: &Args) {

    let mut library = Library::new(&args.arg_path);

    let auditor = Reporter::new();
    library.set_auditor(Some(auditor));

    library.library_version();
    library.load_folders(&mut |_: u64| true);
    library.load_albums(&mut |_: u64| true);
    library.load_masters(&mut |_: u64| true);
    library.load_versions(&mut |_: u64| true);

    println!("Audit: {:?}", library.get_auditor().unwrap());
}

/// print the keywords with indentation for the hierarchy
fn print_keywords(keywords: &Vec<Keyword>, indent: &str) {
    for keyword in keywords {
        if !keyword.is_valid() {
            continue;
        }
        let name = keyword.name.as_ref().unwrap();
        let uuid = keyword.uuid().as_ref().unwrap();
        let parent = if let &Some(ref p) = keyword.parent() {
            p.clone()
        } else {
            "".to_owned()
        };
        println!("| {}{} | {} | {} |", indent, name,
                 uuid, parent);
        if keyword.children.is_some() {
            let new_indent;
            if indent.is_empty() {
                new_indent = "+- ".to_owned() + indent;
            } else {
                new_indent = "\t".to_owned() + indent;
            }
            print_keywords(keyword.children.as_ref().unwrap(), &new_indent);
        }
    }
}

fn process_dump(args: &Args) {
    let mut library = Library::new(&args.arg_path);

    {
        let version = library.library_version();
        println!("Version {}", version);
    }

    let model_info = library.get_model_info().unwrap();
    println!("model info");
    println!("\tDB version: {}", model_info.db_version.unwrap_or(0));
    println!("\tDB minor version: {}", model_info.db_minor_version.unwrap_or(0));
    println!("\tDB back compat: {}", model_info.db_minor_back_compatible_version.unwrap_or(0));
    println!("\tProject version: {}", model_info.project_version.unwrap_or(0));
    println!("\tCreation date: {}", model_info.create_date.unwrap_or("NONE".to_owned()));
    println!("\tImageIO: {} Camera RAW: {}",
             model_info.image_io_version.unwrap_or("NONE".to_owned()),
             model_info.raw_camera_bundle_version.unwrap_or("NONE".to_owned()));

    if args.flag_all || args.flag_folders {

        let mut pb = ProgressBar::new(1);
        pb.tick_format("|/-\\");

        library.load_folders(&mut |_: u64| {
            pb.tick();
            true
        });
        pb.finish();

        let folders = library.get_folders();
        println!("{} Folders:", folders.len());
        println!("| Name | uuid | impl album | type | model id | path |");
        for folder_uuid in folders {
            if folder_uuid.is_empty() {
                continue;
            }
            match library.get(folder_uuid) {
                Some(&StoreWrapper::Folder(ref folder)) => {
                    let name = folder.name.as_ref().unwrap();
                    let uuid = folder.uuid().as_ref().unwrap();
                    let implicit_album_uuid = if let Some(value) =
                        folder.implicit_album_uuid.as_ref() {
                            value
                        } else {
                            ""
                        };
                    let path = folder.path.as_ref().unwrap();
                    println!("| {} | {} | {} | {} | {} | {} |",
                             name, uuid, implicit_album_uuid,
                             folder.folder_type.unwrap_or(0),
                             folder.model_id(), path)
                },
                _ => println!("folder {} not found", folder_uuid)
            }
        }
    }
    if args.flag_all || args.flag_albums {

        let mut pb = ProgressBar::new(1);
        pb.tick_format("|/-\\");

        library.load_albums(&mut |_: u64| {
            pb.tick();
            true
        });
        pb.finish();

        let albums = library.get_albums();
        println!("{} Albums:", albums.len());
        println!("| name | uuid | parent (fldr) | query (fldr) | type | class | model id |");
        for album_uuid in albums {
            if album_uuid.is_empty() {
                continue;
            }
            match library.get(album_uuid) {
                Some(&StoreWrapper::Album(ref album)) => {
                    let name = if let Some(ref n) = album.name {
                        n.clone()
                    } else {
                        "".to_owned()
                    };
                    let uuid = album.uuid().as_ref().unwrap();
                    let parent = if let &Some(ref p) = album.parent() {
                        p.clone()
                    } else {
                        "".to_owned()
                    };
                    let query_folder_uuid = if let Some(ref qf) =
                        album.query_folder_uuid {
                            qf.clone()
                        } else {
                            "".to_owned()
                        };
                    println!("| {} | {} | {} | {} | {} | {} | {} |",
                             name, uuid, parent, query_folder_uuid,
                             album.album_type.unwrap_or(0),
                             album.subclass.unwrap_or(0), album.model_id())
                },
                _ => println!("album {} not found", album_uuid)
            }
        }
    }
    if args.flag_all || args.flag_keywords {
        if let Some(ref keywords) = library.list_keywords() {
            println!("{} keywords:", keywords.len());
            println!("| name | uuid | parent |");
            print_keywords(keywords, &"".to_owned());
        }
    }

    if args.flag_all || args.flag_masters  {

        let count = model_info.master_count.unwrap_or(0) as u64;
        let mut pb = ProgressBar::new(count);

        library.load_masters(&mut |inc: u64| {
            pb.add(inc);
            true
        });
        pb.finish();

        let masters = library.get_masters();
        println!("{} Masters:", masters.len());
        println!("| uuid | project | path |");
        for master_uuid in masters {
            if master_uuid.is_empty() {
                continue;
            }
            match library.get(master_uuid) {
                Some(&StoreWrapper::Master(ref master)) => {
                    let uuid = master.uuid().as_ref().unwrap();
                    let parent = master.parent().as_ref().unwrap();
                    let image_path = master.image_path.as_ref().unwrap();
                    println!("| {} | {} | {} |", uuid, parent, image_path)
                },
                _ => println!("master {} not found", master_uuid)
            }
        }
    }
    if args.flag_all || args.flag_versions  {

        let count = model_info.version_count.unwrap_or(0) as u64;
        let mut pb = ProgressBar::new(count);

        library.load_versions(&mut |inc: u64| {
            pb.add(inc);
            true
        });
        pb.finish();

        let versions = library.get_versions();
        println!("{} Versions:", versions.len());
        println!("| uuid | master | project | name | original |");
        for version_uuid in versions {
            if version_uuid.is_empty() {
                continue;
            }
            match library.get(version_uuid) {
                Some(&StoreWrapper::Version(ref version)) => {
                    let uuid = version.uuid().as_ref().unwrap();
                    let parent = version.parent().as_ref().unwrap();
                    let project_uuid = version.project_uuid.as_ref().unwrap();
                    let name = version.name.as_ref().unwrap();

                    println!("| {} | {} | {} | {} | {} |",
                             uuid, parent, project_uuid, name,
                             version.is_original.unwrap_or(false))
                },
                _ => println!("version {} not found", version_uuid)
            }
        }

    }
}
