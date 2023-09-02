/*
 This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

extern crate aplib;
extern crate docopt;
extern crate pbr;
extern crate serde;

use std::io::stderr;

use docopt::Docopt;
use pbr::ProgressBar;
use serde::Deserialize;

use aplib::audit::{Report, Reporter};
use aplib::AlbumSubclass;
use aplib::AplibObject;
use aplib::FolderType;
use aplib::Keyword;
use aplib::Library;
use aplib::ModelInfo;
use aplib::StoreWrapper;

const USAGE: &str = "
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

#[derive(Debug, Deserialize)]
struct Args {
    arg_command: Command,
    arg_path: String,
    flag_all: bool,
    flag_albums: bool,
    flag_versions: bool,
    flag_masters: bool,
    flag_folders: bool,
    flag_keywords: bool,
}

#[derive(Debug, Deserialize)]
enum Command {
    Dump,
    Audit,
    Unknown(String),
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.argv(std::env::args()).deserialize())
        .unwrap_or_else(|e| e.exit());
    {
        match args.arg_command {
            Command::Dump => process_dump(&args),
            Command::Audit => process_audit(&args),
            _ => (),
        };
    }
}

fn print_report(report: &Report) {
    println!("+---- Ignored {}", report.ignored_count());
    let mut ignored: Vec<&String> = report.get_ignored().iter().collect();
    ignored.sort();
    for key in ignored {
        println!("    +- {}", key);
    }
    println!("+---- Skipped {}", report.skipped_count());
    let mut skipped: Vec<&String> = report.get_skipped().keys().collect();
    skipped.sort();
    for key in skipped {
        let reason = &report.get_skipped()[key];
        println!("    +- {} ({:?})", key, reason);
    }
}

fn process_audit(args: &Args) {
    let mut library = Library::new(&args.arg_path);

    let auditor = Reporter::new();
    library.set_auditor(Some(auditor));

    {
        let version = library.library_version();
        if version.is_err() {
            println!("Invalid library");
            return;
        }
    }
    library.load_folders(&mut |_: u64| true);
    library.load_albums(&mut |_: u64| true);
    library.load_masters(&mut |_: u64| true);
    library.load_versions(&mut |_: u64| true);

    println!("Audit:");
    let auditor = library.auditor().unwrap();
    println!("Parsed {}", auditor.parsed_count());
    println!("+-----------------------------");
    for (key, report) in auditor.get_parsed() {
        if report.skipped_count() > 0 || report.ignored_count() > 0 {
            println!("| {} ", key);
            print_report(report);
        }
    }
    println!("+-----------------------------");
    println!("Skipped {}", auditor.skipped_count());
    for key in auditor.get_skipped().keys() {
        println!("| {} ", key);
    }
    println!("Ignored {}", auditor.ignored_count());
    for key in auditor.get_ignored() {
        println!("| {} ", key);
    }
}

/// print the keywords with indentation for the hierarchy
fn print_keywords(keywords: &[Keyword], indent: &str) {
    for keyword in keywords {
        if !keyword.is_valid() {
            continue;
        }
        let name = keyword.name.as_ref().unwrap();
        let uuid = keyword.uuid().as_ref().unwrap();
        let parent = if let Some(ref p) = *keyword.parent() {
            p.clone()
        } else {
            String::new()
        };
        println!("| {:<26} | {:<26} | {}{}", uuid, parent, indent, name);
        if keyword.children.is_some() {
            let new_indent = if indent.is_empty() {
                String::from("+- ") + indent
            } else {
                String::from("\t") + indent
            };
            print_keywords(keyword.children.as_ref().unwrap(), &new_indent);
        }
    }
}

fn process_dump(args: &Args) {
    let mut library = Library::new(&args.arg_path);

    {
        if let Ok(version) = library.library_version() {
            println!("Version {}", version);
        } else {
            println!("Version not found.");
            return;
        }
    }

    let model_info = library.get_model_info().unwrap();
    println!("model info");
    println!("\tDB version: {}", model_info.db_version.unwrap_or(0));
    println!(
        "\tDB minor version: {}",
        model_info.db_minor_version.unwrap_or(0)
    );
    println!(
        "\tDB back compat: {}",
        model_info.db_minor_back_compatible_version.unwrap_or(0)
    );
    println!(
        "\tProject version: {}",
        model_info.project_version.unwrap_or(0)
    );
    println!(
        "\tCreation date: {}",
        model_info
            .create_date
            .as_ref()
            .unwrap_or(&String::from("NONE"))
    );
    println!(
        "\tImageIO: {} Camera RAW: {}",
        model_info
            .image_io_version
            .as_ref()
            .unwrap_or(&String::from("NONE")),
        model_info
            .raw_camera_bundle_version
            .as_ref()
            .unwrap_or(&String::from("NONE"))
    );

    if args.flag_all || args.flag_folders {
        dump_folders(&mut library);
    }
    if args.flag_all || args.flag_albums {
        dump_albums(&mut library);
    }
    if args.flag_all || args.flag_keywords {
        dump_keywords(&mut library);
    }

    if args.flag_all || args.flag_masters {
        dump_masters(&model_info, &mut library);
    }
    if args.flag_all || args.flag_versions {
        dump_versions(&model_info, &mut library);
    }
}

fn dump_folders(library: &mut Library) {
    let mut pb = ProgressBar::on(stderr(), 1);
    pb.tick_format("|/-\\");

    library.load_folders(&mut |_: u64| {
        pb.tick();
        true
    });
    pb.finish();

    let folders = library.folders();
    println!("{} Folders:", folders.len());
    println!("| Name                       | uuid                       | impl album                 | type | model id | path");
    println!("+----------------------------+----------------------------+----------------------------+------+----------+----------");
    for folder_uuid in folders {
        if folder_uuid.is_empty() {
            continue;
        }
        match library.get(folder_uuid) {
            Some(StoreWrapper::Folder(folder)) => {
                let name = folder.name.as_ref().unwrap();
                let uuid = folder.uuid().as_ref().unwrap();
                let implicit_album_uuid = if let Some(value) = folder.implicit_album_uuid.as_ref() {
                    value
                } else {
                    ""
                };
                let path = folder.path.as_ref().unwrap();
                let folder_type = if let Some(ref ft) = folder.folder_type {
                    FolderType::to_int(ft)
                } else {
                    0
                };
                println!(
                    "| {:<26} | {:<26} | {:<26} | {:>4} | {:>8} | {}",
                    name,
                    uuid,
                    implicit_album_uuid,
                    folder_type,
                    folder.model_id(),
                    path
                )
            }
            _ => println!("folder {} not found", folder_uuid),
        }
    }
}

fn dump_albums(library: &mut Library) {
    let mut pb = ProgressBar::on(stderr(), 1);
    pb.tick_format("|/-\\");

    library.load_albums(&mut |_: u64| {
        pb.tick();
        true
    });
    pb.finish();

    let albums = library.albums();
    println!("{} Albums:", albums.len());
    println!("| uuid                       | parent (fldr)              | query (fldr)               | type | class | model id | name");
    println!("+----------------------------+----------------------------+----------------------------+------+-------+----------+-----");
    for album_uuid in albums {
        if album_uuid.is_empty() {
            continue;
        }
        match library.get(album_uuid) {
            Some(StoreWrapper::Album(album)) => {
                let name = if let Some(ref n) = album.name {
                    n.clone()
                } else {
                    String::new()
                };
                let uuid = album.uuid().as_ref().unwrap();
                let parent = if let Some(ref p) = *album.parent() {
                    p.clone()
                } else {
                    String::new()
                };
                let query_folder_uuid = if let Some(ref qf) = album.query_folder_uuid {
                    qf.clone()
                } else {
                    String::new()
                };
                let album_class = if let Some(ref c) = album.subclass {
                    AlbumSubclass::to_int(c)
                } else {
                    0
                };
                println!(
                    "| {:<26} | {:<26} | {:<26} | {:>4} | {:>5} | {:>8} | {}",
                    uuid,
                    parent,
                    query_folder_uuid,
                    album.album_type.unwrap_or(0),
                    album_class,
                    album.model_id(),
                    name
                )
            }
            _ => println!("album {} not found", album_uuid),
        }
    }
}

fn dump_keywords(library: &mut Library) {
    if let Some(ref keywords) = library.list_keywords() {
        println!("{} keywords:", keywords.len());
        println!("| uuid                       | parent                     | name");
        println!("+----------------------------+----------------------------+-----------");
        print_keywords(keywords, "");
    }
}

fn dump_masters(model_info: &ModelInfo, library: &mut Library) {
    let count = model_info.master_count.unwrap_or(0) as u64;
    let mut pb = ProgressBar::on(stderr(), count);

    library.load_masters(&mut |inc: u64| {
        pb.add(inc);
        true
    });
    pb.finish();

    let masters = library.masters();
    println!("{} Masters:", masters.len());
    println!("| uuid                       | project                    | path");
    println!("+----------------------------+----------------------------+-----------------------");
    for master_uuid in masters {
        if master_uuid.is_empty() {
            continue;
        }
        match library.get(master_uuid) {
            Some(StoreWrapper::Master(master)) => {
                let uuid = master.uuid().as_ref().unwrap();
                let parent = master.parent().as_ref().unwrap();
                let image_path = master.image_path.as_ref().unwrap();
                println!("| {:<26} | {:<26} | {}", uuid, parent, image_path)
            }
            _ => println!("master {} not found", master_uuid),
        }
    }
}

fn dump_versions(model_info: &ModelInfo, library: &mut Library) {
    let count = model_info.version_count.unwrap_or(0) as u64;
    let mut pb = ProgressBar::on(stderr(), count);

    library.load_versions(&mut |inc: u64| {
        pb.add(inc);
        true
    });
    pb.finish();

    let versions = library.versions();
    println!("{} Versions:", versions.len());
    println!("| uuid                       | master                     | project                    | original | name");
    println!("+----------------------------+----------------------------+----------------------------+----------+------------");
    for version_uuid in versions {
        if version_uuid.is_empty() {
            continue;
        }
        match library.get(version_uuid) {
            Some(StoreWrapper::Version(version)) => {
                let uuid = version.uuid().as_ref().unwrap();
                let parent = version.parent().as_ref().unwrap();
                let project_uuid = version.project_uuid.as_ref().unwrap();
                let name = version.name.as_ref().unwrap();

                println!(
                    "| {:<26} | {:<26} | {:<26} | {:>8} | {}",
                    uuid,
                    parent,
                    project_uuid,
                    version.is_original.unwrap_or(false),
                    name
                )
            }
            _ => println!("version {} not found", version_uuid),
        }
    }
}
