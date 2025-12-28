/*
 * Copyright (C) 2015-2025 Hubert Figuière
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use std::io::stderr;

use clap::{Parser, Subcommand};
use num_traits::ToPrimitive;
use pbr::ProgressBar;

use aplib::audit::{Report, Reporter};
use aplib::AplibObject;
use aplib::Keyword;
use aplib::Library;
use aplib::ModelInfo;
use aplib::NotesProperties;
use aplib::StoreWrapper;
use aplib::{AlbumSubclass, ToXmp, PROGRESS_NONE};

mod tree;

#[derive(Debug, Parser)]
#[command(version)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Clone, Debug, Subcommand)]
enum Command {
    /// Dump content of the library
    Dump(CommandArgs),
    /// Audit the library to find unhandled properties
    Audit(NoArgs),
    /// List all masters
    List(NoArgs),
    /// Dump XMP
    Xmp(XmpArgs),
    /// Print the library tree
    Tree(tree::TreeArgs),
}

#[derive(Clone, Debug, Parser)]
struct NoArgs {
    /// Path to the Aperture Library
    path: String,
}

#[derive(Clone, Debug, Parser)]
struct XmpArgs {
    #[arg(long)]
    /// Version uuid to dump XMP
    version: Option<String>,
    #[arg(long)]
    /// All XMP (very verbose)
    all: bool,
    /// Path to the Aperture Library
    path: String,
}

#[derive(Clone, Debug, Parser)]
struct CommandArgs {
    #[arg(long)]
    /// Dump everything
    all: bool,
    #[arg(long)]
    /// Dump the albums
    albums: bool,
    #[arg(long)]
    /// Dump the versions (of images)
    versions: bool,
    #[arg(long)]
    /// Dump the masters
    masters: bool,
    #[arg(long)]
    /// Dump the folders
    folders: bool,
    #[arg(long)]
    /// Dump the keywords
    keywords: bool,
    #[arg(long)]
    /// Dump the notes
    notes: bool,
    #[arg(long)]
    /// Dump the volumes
    volumes: bool,
    /// Path to the Aperture Library
    path: String,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Dump(_) => process_dump(&args),
        Command::Audit(_) => process_audit(&args),
        Command::List(_) => process_list(&args),
        Command::Xmp(args) => process_xmp(&args),
        Command::Tree(args) => tree::process_tree(&args),
    };
}

/// Return true if the library is ok
fn check_library(library: &mut Library) -> bool {
    let version = library.library_version();
    if version.is_err() {
        println!("Invalid library");
        return false;
    }

    true
}

fn dump_one_xmp(library: &Library, uuid: &str) {
    use exempi2::{SerialFlags, Xmp};

    if let Some(StoreWrapper::Version(version)) = library.get(uuid) {
        let mut xmp = Xmp::new();
        version.to_xmp(&mut xmp);
        println!(
            "{}",
            xmp.serialize(SerialFlags::OMITPACKETWRAPPER, 0).unwrap()
        );
    }
}

fn process_xmp(args: &XmpArgs) {
    let mut library = Library::new(&args.path);
    if !check_library(&mut library) {
        return;
    }
    if args.all || args.version.is_some() {
        library.load_versions(PROGRESS_NONE);
    }
    if args.all {
        let versions = library.versions();
        for version_uuid in versions {
            dump_one_xmp(&library, version_uuid);
        }
    }
    if let Some(version) = &args.version {
        dump_one_xmp(&library, version);
    }
}

fn process_list(args: &Args) {
    if let Command::List(args) = &args.command {
        let mut library = Library::new(&args.path);
        {
            let version = library.library_version();
            if version.is_err() {
                println!("Invalid library");
                return;
            }
        }
        library.load_volumes(PROGRESS_NONE);
        library.load_masters(PROGRESS_NONE);

        let masters = library.masters();
        for master_uuid in masters {
            if master_uuid.is_empty() {
                continue;
            }
            if let Some(master_path) = library.resolve_master_path(master_uuid) {
                println!("{master_path}");
            } else {
                eprintln!("Can't resolve master path for {master_uuid}");
            }
        }
    }
}

fn print_report(report: &Report) {
    println!("+---- Ignored {}", report.ignored_count());
    let mut ignored: Vec<&String> = report.get_ignored().iter().collect();
    ignored.sort();
    for key in ignored {
        println!("    +- {key}");
    }
    println!("+---- Skipped {}", report.skipped_count());
    let mut skipped: Vec<&String> = report.get_skipped().keys().collect();
    skipped.sort();
    for key in skipped {
        let reason = &report.get_skipped()[key];
        println!("    +- {key} ({reason:?})");
    }
}

fn process_audit(args: &Args) {
    if let Command::Audit(args) = &args.command {
        let mut library = Library::new(&args.path);

        let auditor = Reporter::new();
        library.set_auditor(Some(auditor));

        {
            let version = library.library_version();
            if version.is_err() {
                println!("Invalid library");
                return;
            }
        }
        library.load_volumes(PROGRESS_NONE);
        library.load_folders(PROGRESS_NONE);
        library.load_albums(PROGRESS_NONE);
        library.load_masters(PROGRESS_NONE);
        library.load_versions(PROGRESS_NONE);

        println!("Audit:");
        let auditor = library.auditor().unwrap();
        println!("Parsed {}", auditor.parsed_count());
        println!("+-----------------------------");
        for (key, report) in auditor.get_parsed() {
            if report.skipped_count() > 0 || report.ignored_count() > 0 {
                println!("| {key} ");
                print_report(report);
            }
        }
        println!("+-----------------------------");
        println!("Skipped {}", auditor.skipped_count());
        for key in auditor.get_skipped().keys() {
            println!("| {key} ");
        }
        println!("Ignored {}", auditor.ignored_count());
        for key in auditor.get_ignored() {
            println!("| {key} ");
        }
    } else {
        unreachable!()
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
        let parent = keyword.parent().clone().unwrap_or_default();
        println!("| {uuid:<26} | {parent:<26} | {indent}{name}");
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
    if let Command::Dump(args) = &args.command {
        let mut library = Library::new(&args.path);

        {
            if let Ok(version) = library.library_version() {
                println!("Version {version}");
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

        if args.all || args.volumes {
            dump_volumes(&mut library);
        }
        if args.all || args.folders {
            dump_folders(&mut library);
        }
        if args.all || args.albums {
            dump_albums(&mut library);
        }
        if args.all || args.keywords {
            dump_keywords(&mut library);
        }
        if args.all || args.notes {
            dump_notes(&mut library);
        }

        if args.all || args.masters {
            dump_masters(&model_info, &mut library);
        }
        if args.all || args.versions {
            dump_versions(&model_info, &mut library);
        }
    } else {
        unreachable!()
    }
}

fn dump_volumes(library: &mut Library) {
    let mut pb = ProgressBar::on(stderr(), 1);
    pb.tick_format("|/-\\");

    library.load_volumes(Some(&mut |_: u64| {
        pb.tick();
        true
    }));
    pb.finish();

    let volumes = library.volumes();
    println!("{} Volumes:", volumes.len());

    println!("| Name                   | uuid                   | Disk UUID                            | id   |");
    println!("+------------------------+------------------------+--------------------------------------+------+");
    for uuid in volumes {
        if uuid.is_empty() {
            continue;
        }
        match library.get(uuid) {
            Some(StoreWrapper::Volume(volume)) => {
                let name = volume.volume_name.as_ref().unwrap();
                let uuid = volume.uuid().as_ref().unwrap();
                let disk_uuid = volume.disk_uuid.clone().unwrap_or_default();
                let model_id = volume.model_id();
                println!("| {name:<22} | {uuid:<22} | {disk_uuid:<36} | {model_id:>4} |",)
            }
            _ => {
                println!("Folder not found.");
            }
        }
    }
}

fn dump_folders(library: &mut Library) {
    let mut pb = ProgressBar::on(stderr(), 1);
    pb.tick_format("|/-\\");

    library.load_folders(Some(&mut |_: u64| {
        pb.tick();
        true
    }));
    pb.finish();

    let folders = library.folders();
    println!("{} Folders:", folders.len());
    println!("| Name                   | uuid                   | parent                 | impl album                            | type      | model id | path");
    println!("+------------------------+------------------------+------------------------+---------------------------------------+-----------+----------+----------");
    for folder_uuid in folders {
        if folder_uuid.is_empty() {
            continue;
        }
        match library.get(folder_uuid) {
            Some(StoreWrapper::Folder(folder)) => {
                let name = folder.name.as_ref().unwrap();
                let uuid = folder.uuid().as_ref().unwrap();
                let parent_uuid = folder.parent().clone().unwrap_or_default();
                let implicit_album_uuid = folder.implicit_album_uuid.clone().unwrap_or_default();
                let path = folder.path.as_ref().unwrap();
                let folder_type = folder.folder_type.unwrap_or_default();
                let folder_type_num = folder
                    .folder_type
                    .as_ref()
                    .and_then(ToPrimitive::to_i32)
                    .unwrap_or(0);
                println!(
                    "| {:<22} | {:<22} | {:<22} | {:<37} | {:<7}{:>2} | {:>8} | {}",
                    name,
                    uuid,
                    parent_uuid,
                    implicit_album_uuid,
                    format!("{folder_type:?}"),
                    folder_type_num,
                    folder.model_id(),
                    path
                )
            }
            _ => println!("folder {folder_uuid} not found"),
        }
    }
}

fn dump_albums(library: &mut Library) {
    let mut pb = ProgressBar::on(stderr(), 1);
    pb.tick_format("|/-\\");

    library.load_albums(Some(&mut |_: u64| {
        pb.tick();
        true
    }));
    pb.finish();

    let albums = library.albums();
    println!("{} Albums:", albums.len());
    println!("| uuid                                  | parent (fldr)              | query (fldr)               | type | class      | model id | name");
    println!("+---------------------------------------+----------------------------+----------------------------+------+------------+----------+-----");
    for album_uuid in albums {
        if album_uuid.is_empty() {
            continue;
        }
        match library.get(album_uuid) {
            Some(StoreWrapper::Album(album)) => {
                let name = album.name.clone().unwrap_or_default();
                let uuid = album.uuid().as_ref().unwrap();
                let parent = album.parent().clone().unwrap_or_default();
                let query_folder_uuid = album.query_folder_uuid.clone().unwrap_or_default();
                let album_class = album.subclass.unwrap_or_default();
                let album_class_num = album
                    .subclass
                    .as_ref()
                    .and_then(AlbumSubclass::to_i32)
                    .unwrap_or(0);
                println!(
                    "| {uuid:<37} | {parent:<26} | {query_folder_uuid:<26} | {:>4} | {album_class:<8?}{album_class_num:>2} | {:>8} | {name}",
                    album.album_type.unwrap_or(0),
                    album.model_id(),
                )
            }
            _ => println!("album {album_uuid} not found"),
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

    library.load_masters(Some(&mut |inc: u64| {
        pb.add(inc);
        true
    }));
    pb.finish();

    let masters = library.masters();
    println!("{} Masters:", masters.len());
    println!("| uuid                   | project                | alternate              | mtyp | subt  | orig | path");
    println!("+------------------------+------------------------+------------------------+------+-------+-----------------------");
    for master_uuid in masters {
        if master_uuid.is_empty() {
            continue;
        }
        match library.get(master_uuid) {
            Some(StoreWrapper::Master(master)) => {
                let uuid = master.uuid().as_ref().unwrap();
                let parent = master.parent().as_ref().unwrap();
                let image_path = master.image_path.as_ref().unwrap();
                let alternate = master.alternate_master.clone().unwrap_or_default();
                let mtype = master.master_type.clone().unwrap_or_default();
                let subtype = master.subtype.clone().unwrap_or_default();
                let orig_uuid = master.original_version_uuid.clone().unwrap_or_default();
                println!(
                    "| {uuid:<22} | {parent:<22} | {alternate:<22} | {mtype:<4} | {subtype:<5} | {orig_uuid} | {image_path}",
                )
            }
            _ => println!("master {master_uuid} not found"),
        }
    }
}

fn dump_versions(model_info: &ModelInfo, library: &mut Library) {
    let count = model_info.version_count.unwrap_or(0) as u64;
    let mut pb = ProgressBar::on(stderr(), count);

    library.load_versions(Some(&mut |inc: u64| {
        pb.add(inc);
        true
    }));
    pb.finish();

    let versions = library.versions();
    println!("{} Versions:", versions.len());
    println!("| uuid                   | master                 | project                | orig  | raw   | num | name");
    println!("+------------------------+------------------------+------------------------+-------+-------+-----+------------");
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
                let rawmaster = version.raw_master_uuid == version.master_uuid;
                let num = version
                    .version_number
                    .map(|v| v.to_string())
                    .unwrap_or_default();

                println!(
                    "| {uuid:<22} | {parent:<22} | {project_uuid:<22} | {:>5} | {rawmaster:>5} | {num:>3} | {name}",
                    version.is_original.unwrap_or(false),
                )
            }
            _ => println!("version {version_uuid} not found"),
        }
    }
}

fn dump_note(t: char, note: &NotesProperties) {
    let uuid = note.uuid.as_ref().unwrap();
    let date = if let Some(date) = &note.create_date {
        date.to_string()
    } else {
        "None".to_string()
    };
    let property = note.property_key.clone().unwrap_or_default();
    println!(
        "| {t} | {uuid:<22} | {date:<33} | {:>8} | {property:<13} | {}",
        if let Some(note_data) = &note.data {
            note_data.len().to_string()
        } else {
            "None".to_string()
        },
        if let Some(note) = &note.note {
            note
        } else {
            "None"
        }
    );
}

fn dump_notes(library: &mut Library) {
    let mut pb = ProgressBar::on(stderr(), 1);
    pb.tick_format("|/-\\");

    library.load_masters(Some(&mut |inc: u64| {
        pb.add(inc);
        true
    }));
    pb.finish();

    library.load_folders(Some(&mut |_: u64| {
        pb.tick();
        true
    }));
    pb.finish();

    println!("| T | uuid                   | created                           | data len | property      | note                   ");
    println!("+---+------------------------+-----------------------------------+----------+---------------|------------------------");
    let masters = library.masters();
    for master_uuid in masters {
        if master_uuid.is_empty() {
            continue;
        }
        if let Some(StoreWrapper::Master(master)) = library.get(master_uuid) {
            if let Some(notes) = &master.notes {
                for note in notes {
                    dump_note('M', note);
                }
            }
        }
    }

    let folders = library.folders();
    for folder_uuid in folders {
        if folder_uuid.is_empty() {
            continue;
        }
        if let Some(StoreWrapper::Folder(folder)) = library.get(folder_uuid) {
            if let Some(notes) = &folder.notes {
                for note in notes {
                    dump_note('F', note);
                }
            }
        }
    }
}
