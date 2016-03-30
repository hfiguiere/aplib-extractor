/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

extern crate aplib;
extern crate docopt;
extern crate rustc_serialize;

use docopt::Docopt;
use rustc_serialize::{Decodable, Decoder};

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

    library.load_folders();
    library.load_albums();
    library.load_masters();
    library.load_versions();

}

/// print the keywords with indentation for the hierarchy
fn print_keywords(keywords: &Vec<Keyword>, indent: &str) {
    for keyword in keywords {
        println!("| {}{} | {} | {} |", indent, keyword.name,
                 keyword.uuid(), keyword.parent());
        if !keyword.children.is_empty() {
            let new_indent;
            if indent.is_empty() {
                new_indent = "+- ".to_owned() + indent;
            } else {
                new_indent = "\t".to_owned() + indent;
            }
            print_keywords(&keyword.children, &new_indent);
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
    println!("\tDB version: {}", model_info.db_version);
    println!("\tDB minor version: {}", model_info.db_minor_version);
    println!("\tProject version: {}", model_info.project_version);

    library.load_folders();
    library.load_albums();
    if args.flag_all || args.flag_folders {
        let folders = library.get_folders();
        println!("{} Folders:", folders.len());
        println!("| Name | uuid | impl album | type | model id | path |");
        for folder_uuid in folders {
            if folder_uuid.is_empty() {
                continue;
            }
            match library.get(folder_uuid) {
                Some(&StoreWrapper::Folder(ref folder)) =>
                    println!("| {} | {} | {} | {} | {} | {} |",
                             folder.name, folder.uuid(),
                             folder.implicit_album_uuid,
                             folder.folder_type,
                             folder.model_id(), folder.path),
                _ => println!("folder {} not found", folder_uuid)
            }
        }
    }
    if args.flag_all || args.flag_albums {
        let albums = library.get_albums();
        println!("{} Albums:", albums.len());
        println!("| name | uuid | parent (fldr) | query (fldr) | type | class | model id |");
        for album_uuid in albums {
            if album_uuid.is_empty() {
                continue;
            }
            match library.get(album_uuid) {
                Some(&StoreWrapper::Album(ref album)) =>
                    println!("| {} | {} | {} | {} | {} | {} | {} |",
                             album.name,
                             album.uuid(), album.parent(),
                             album.query_folder_uuid,
                             album.album_type,
                             album.subclass, album.model_id()),
                _ => println!("album {} not found", album_uuid)
            }
        }
    }
    if args.flag_all || args.flag_keywords {
        let keywords = library.list_keywords();
        println!("{} keywords:", keywords.len());
        println!("| uuid | parent | name |");
        print_keywords(&keywords, &"".to_owned());
    }

    library.load_masters();
    library.load_versions();
    if args.flag_all || args.flag_masters  {
        let masters = library.get_masters();
        println!("{} Masters:", masters.len());
        println!("| uuid | project | path |");
        for master_uuid in masters {
            if master_uuid.is_empty() {
                continue;
            }
            match library.get(master_uuid) {
                Some(&StoreWrapper::Master(ref master)) =>
                    println!("| {} | {} | {} |",
                             master.uuid(), master.parent(),
                             master.image_path),
                _ => println!("master {} not found", master_uuid)
            }
        }
    }
    if args.flag_all || args.flag_versions  {
        let versions = library.get_versions();
        println!("{} Versions:", versions.len());
        println!("| uuid | master | project | name | original |");
        for version_uuid in versions {
            if version_uuid.is_empty() {
                continue;
            }
            match library.get(version_uuid) {
                Some(&StoreWrapper::Version(ref version)) =>
                    println!("| {} | {} | {} | {} | {} |",
                             version.uuid(), version.parent(),
                             version.project_uuid, version.name,
                             version.is_original),
                _ => println!("version {} not found", version_uuid)
            }
        }
    }
}
