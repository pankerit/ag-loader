#![allow(non_snake_case)]

use std::collections::HashMap;
use std::env;
use std::ffi::OsStr;
use std::fs::{read_dir, read_to_string, soft_link, write, File};
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;
use String;

const CHEAT_NAME: &str = "osu-master";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let processName = getCurrentProcessName();
    let cheat_data = cheatData(
        "https://europe-west3-assist-games.cloudfunctions.net/DOWNLOADER",
        &processName,
    );
    if checkVersion(&cheat_data.version) {
        println!("Same version");
    } else {
        println!("Downloading, please wait up to 3 min");
        downloadAndUnzip(&cheat_data.link, pathToFolderCheat());
    }
    updateVersion(&cheat_data.version);
    println!("Running");
    runCheat();
    Ok(())
}

fn getCurrentProcessName() -> String {
    // let name = env::args()
    //     .next()
    //     .as_ref()
    //     .map(Path::new)
    //     .and_then(Path::file_name)
    //     .and_then(OsStr::to_str)
    //     .map(String::from)
    //     .unwrap();
    // let mut parse = String::from(name);
    // let mut index = parse.find(" ").unwrap_or(0);
    // if index == 0 {
    //     index = parse.find(".").unwrap();
    // }
    // parse.replace_range(index..parse.len(), "");
    // parse
    String::from(CHEAT_NAME)
}

fn pathToFolderCheat() -> String {
    let processName = getCurrentProcessName();
    let a = std::env::temp_dir();
    let path = Path::new(&a);
    format!("{}{}", path.to_str().unwrap(), processName)
}

fn downloadAndUnzip(link: &str, to: String) {
    let mut tmpfile = tempfile::tempfile().unwrap();
    reqwest::blocking::get(link)
        .unwrap()
        .copy_to(&mut tmpfile)
        .unwrap();
    unzip(tmpfile, to);
}

fn unzip(file: std::fs::File, to: std::string::String) {
    let mut archive = zip::ZipArchive::new(file).unwrap();
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        let outpath = Path::new(&to).join(outpath);
        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }
        if (&*file.name()).ends_with('/') {
            std::fs::create_dir_all(&outpath).unwrap();
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = std::fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
    }
}

struct CheatData {
    link: String,
    version: String,
    name: String,
}

fn cheatData(link: &str, cheat: &str) -> CheatData {
    let data = reqwest::blocking::get(link)
        .unwrap()
        .json::<Vec<HashMap<String, String>>>()
        .unwrap();
    let mut find: CheatData = CheatData {
        link: "".to_string(),
        version: "".to_string(),
        name: "".to_string(),
    };
    for cheatData in data.iter() {
        let _cheat = cheatData.get("cheat").unwrap();
        if cheat == _cheat {
            find.link = cheatData.get("link").unwrap().to_string();
            find.version = cheatData.get("version").unwrap().to_string();
            find.name = cheatData.get("cheat").unwrap().to_string();
        }
    }
    find
}

fn checkVersion(version: &str) -> bool {
    let filePath = pathToFolderCheat() + "\\version.txt";
    let currentVersion = read_to_string(filePath).unwrap_or("".to_string());
    version == currentVersion
}

fn updateVersion(version: &str) {
    let filePath = pathToFolderCheat() + "\\version.txt";
    write(filePath, version).unwrap();
}

fn runCheat() {
    let mut path = pathToFolderCheat();
    path.push_str("\\cheat\\Discord.exe");
    Command::new(&path).spawn().expect("Error");
}
