use clap::{App, Arg, SubCommand};
use regex::Regex;
use reqwest::blocking::Client;
use serde::*;
use std::{
    fs::{create_dir, write},
    path::PathBuf,
};

const BOILERPLATE_JSON: &'static str = include_str!("boilerplate.json");
const BOILERPLATE_SPWN: &'static str = include_str!("boilerplate.spwn");

fn main() {
    let matches = App::new("pckp")
        .about("A package manager for SPWN")
        .author("https://github.com/krista-chan")
        .subcommands(vec![
            SubCommand::with_name("install")
                .alias("i")
                .about("Installs a package [UNFINISHED]")
                .arg(
                    Arg::with_name("package")
                        .help("The package to install")
                        .required(true),
                ),
            SubCommand::with_name("uninstall")
                .aliases(&["remove", "rm", "ui"])
                .about("Remove a locally installed package by name [UNFINISHED]"),
            SubCommand::with_name("clean")
                .aliases(&["cln", "c"])
                .about("Remove all packages and reinstall them again [UNFINISHED]"),
            SubCommand::with_name("init")
                .alias("new")
                .about("Creates boilerplate code for a package in a given path (use . as a path argument to place it in the current directory)")
                .args(&[
                    Arg::with_name("dir")
                        .help("The directory to be initialised with pckp boilerplate")
                        .required(true),
                    Arg::with_name("name")
                        .short("n")
                        .help("Override the name of this package")
                        .takes_value(true)
                        .required(true),
                    Arg::with_name("author")
                        .short("a")
                        .help("Set the author of this package")
                        .takes_value(true)
                        .required(true),
                ]),
        ])
        .get_matches();

    if matches.is_present("init") {
        let dir = matches
            .subcommand_matches("init")
            .unwrap()
            .value_of("dir")
            .unwrap();

        let name = matches
            .subcommand_matches("init")
            .unwrap()
            .value_of("name")
            .unwrap();

        let author = matches
            .subcommand_matches("init")
            .unwrap()
            .value_of("author")
            .unwrap();

        init_dir(dir, name, author).unwrap();
    }

    if matches.is_present("install") {
        let package_name = matches
            .subcommand_matches("install")
            .unwrap()
            .value_of("package")
            .unwrap();
        // let regex = Regex::new(r"(https://github\.com/)?[\w-]+/[\w-]+").unwrap();
        let regex = Regex::new(r"[\w-]").unwrap();

        if regex.is_match(package_name) {
            //*! This is for github */
            // let mut reverse_pkname = package_name.split('/').rev();

            // let name = reverse_pkname.next().unwrap();
            // let username = reverse_pkname.next().unwrap();

            // let api_info_for_user = format!("https://api.github.com/repos/{}/{}", username, name);

            // let client = blocking::Client::new();
            // let res = client
            //     .get(api_info_for_user)
            //     .header(USER_AGENT, "pckp-package-manager")
            //     .send()
            //     .unwrap();

            // println!("{:?}", res)

            let client = Client::new();
            let res = client
                .get(format!("https://pckp.xyz/api/package/{}", package_name))
                .send()
                .unwrap();
            let res_json = res.json::<PckpResponse>();
        }
    }
}

#[derive(Deserialize, Serialize)]
struct PckpResponse {
    name: String,
    desc: String,
    long_desc: String,
    homepage: String,
    versions: Vec<String>,
}

fn init_dir(dir: &str, name: &str, author: &str) -> std::io::Result<()> {
    let init_json = BOILERPLATE_JSON
        .replace("PACKAGE_NAME", name)
        .replace("PCKP_VERSION", "TODO")
        .replace("PACKAGE_AUTHOR", author);

    let init_spwn = BOILERPLATE_SPWN
        .replace("PACKAGE_NAME", name);

    let paths = (PathBuf::from(dir), PathBuf::from("pckp.json"));
    let src_dir = &paths.0.join("src");

    create_dir(&paths.0)?;
    create_dir(&src_dir)?;

    write(paths.0.join(paths.1), init_json)?;
    write(src_dir.join("lib.spwn"), init_spwn)?;

    Ok(())
}
