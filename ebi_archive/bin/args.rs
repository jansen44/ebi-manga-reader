use clap::{Arg, ArgGroup, ArgMatches, Command};

pub fn get_args() -> ArgMatches {
    Command::new("ebi-archiver")
    .subcommand_required(true)
    .arg_required_else_help(true)
    .subcommand(Command::new("sources").about("Lists all available sources"))
    .subcommand(
        Command::new("down")
            .about("Download manga given with given options")
            .arg(
                Arg::new("target_dir")
                    .short('d')
                    .long("dir")
                    .help("Where the the chapter will be saved at. This values defaults to \"<UserDir>/ebi/<source>/<manga>/<chapter>/\".")
            )
            .arg(
                Arg::new("source")
                    .short('s')
                    .long("source")
                    .display_order(0)
                    .help("Manga source that you'll search, select and download your manga from. Possible values can be listed using \"ebi-archiver sources\".")
                    .max_occurrences(1)
                    .required(true)
                    .takes_value(true),
            )
            .arg(
                Arg::new("title")
                    .short('t')
                    .long("title")
                    .display_order(1)
                    .help("Manga title that will be used to look after it before downloading. Optional if \"identifier\" present.")
                    .max_occurrences(1)
                    .takes_value(true),
            )
            .arg(
                Arg::new("identifier")
                    .short('i')
                    .long("identifier")
                    .display_order(2)
                    .help("Select manga given it's identifier (faster than title). Optional if \"title\" present.")
                    .max_occurrences(1)
                    .takes_value(true),
            )
            .arg(
                Arg::new("chapter")
                    .short('c')
                    .long("chapter")
                    .display_order(3)
                    .help("Manga chapter number to download.")
                    .max_occurrences(1)
                    .required(true)
                    .takes_value(true),
            )
            .group(ArgGroup::new("manga").args(&["title", "identifier"]).required(true)),
    )
    .get_matches()
}
