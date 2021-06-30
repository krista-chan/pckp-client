use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("pckp")
        .about("A package manager for SPWN")
        .author("https://github.com/krista-chan")
        .subcommands(vec![
            SubCommand::with_name("install")
                .alias("i")
                .about("Installs a package")
                .arg(
                    Arg::with_name("package")
                        .help("The package to install")
                        .required(true)
                ),

            SubCommand::with_name("uninstall")
                .aliases(&["remove", "rm", "ui"])
                .about("Remove a locally installed package by name"),

            SubCommand::with_name("clean")
                .aliases(&["cln", "c"])
                .about("Remove all packages and reinstall them again"),

            SubCommand::with_name("init")
                .alias("new")
                .about("Creates boilerplate code for a package")
        ])
        .get_matches();

    if matches.is_present("install") {
        let package_name = matches.subcommand_matches("install").unwrap().value_of("package").unwrap();
        // TODO: validate name with regex and then make request to relevant github repo, check if spwn.json is present, and pull from repo to ./spwn_modules (maybe add spwn lockfile?) 
    }
}
