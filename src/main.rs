use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::process;

use clap::{App, Arg, ArgMatches};

use lib::json_data::{DataRoot, generate_template, get_projects, save};

use lib::excel_writer::{ExcelTranslations};
use lib::strings_generator::{generate_strings};
use lib::excel_file::ExcelFile;
use lib::excel_reader::import_excel;
use lib::strings_generator::StringsGeneratorType::{Ios, Android};


const COMMAND_GENERATE_TEMPLATE: &str = "template";
const COMMAND_IMPORT_XLSX: &str = "import";
const COMMAND_EXPORT_XLSX: &str = "export-xlsx";
const COMMAND_EXPORT_STRINGS: &str = "export";
const ARG_FILE_NAME: &str = "file_name";
const ARG_INPUT_FILE_NAME: &str = "input_file_name";
const ARG_IMPORT_IGNORE_UNKNOWN_KEYS: &str = "ignore_unknown_keys";
const ARG_OUTPUT_FILE_NAME: &str = "export_file_name";
const ARG_PROJECT_NAME: &str = "project_name";
const ARG_EXPORT_STRINGS_TYPE: &str = "export_type";

fn main() {
    let matches = get_arguments();

    match matches.subcommand_name() {
        Some(COMMAND_GENERATE_TEMPLATE) => generate_template_command(&matches),
        Some(COMMAND_IMPORT_XLSX) => import_xlsx_command(&matches),
        Some(COMMAND_EXPORT_XLSX) => export_xlsx_command(&matches),
        Some(COMMAND_EXPORT_STRINGS) => export_strings(&matches),
        _ => {
            println!("error: No command provided");
            process::exit(1);
        }
    }
}

fn generate_template_command(matches: &ArgMatches) {
    let file_name = matches.subcommand_matches(COMMAND_GENERATE_TEMPLATE)
        .unwrap()
        .value_of(ARG_FILE_NAME)
        .unwrap();

    if let Err(e) = generate_template(file_name) {
        println!("error: {}", e);
        process::exit(1)
    };
}

fn import_xlsx_command(matches: &ArgMatches) {
    let file_name = matches.subcommand_matches(COMMAND_IMPORT_XLSX)
        .unwrap()
        .value_of(ARG_FILE_NAME)
        .unwrap();

    let xlsx_file_name = matches.subcommand_matches(COMMAND_IMPORT_XLSX)
        .unwrap()
        .value_of(ARG_INPUT_FILE_NAME)
        .unwrap();

    let project_name = matches.subcommand_matches(COMMAND_IMPORT_XLSX)
        .unwrap()
        .value_of(ARG_PROJECT_NAME)
        .unwrap();

    let ignore_unknown = matches.subcommand_matches(COMMAND_IMPORT_XLSX)
        .unwrap()
        .is_present(ARG_IMPORT_IGNORE_UNKNOWN_KEYS);

    let mut projects_data = get_data(file_name);
    let mut file = ExcelFile::new(xlsx_file_name)
        .unwrap_or_else(|err| {
            println!("error: {}", err);
            process::exit(1)
        });

    let project = projects_data.projects.iter().find(|&p| {
        p.name.eq(project_name)
    }).expect("Invalid project name");

    let result = import_excel(&mut file, &mut projects_data.translations, &project, ignore_unknown);
    if let Err(e) = save(file_name, &projects_data) {
        println!("error: {}", e);
        process::exit(1)
    };
}

fn export_xlsx_command(matches: &ArgMatches) {
    let command = matches.subcommand_matches(COMMAND_EXPORT_XLSX)
        .unwrap();

    let file_name = command
        .value_of(ARG_FILE_NAME)
        .unwrap();

    let xlsx_file_name = command
        .value_of(ARG_OUTPUT_FILE_NAME)
        .unwrap();

    let project_name = command
        .value_of(ARG_PROJECT_NAME)
        .unwrap();

    let data = get_data(file_name);

    let project = data.projects.iter().find(|&p| {
        p.name.eq(project_name)
    }).expect("Invalid project name");

    let da: BTreeMap<_, _> = data.translations.into_iter()
        .filter(|(_, t)| t.projects.contains(&project.id))
        .collect();

    for (key, value) in da {
        if let Some(asd) = value.values.get(&project.id) {
            for lang in project.langs.to_vec() {

            }
        }
    }

    let excel_translations = ExcelTranslations::new(project.langs.to_vec(), vec![]);
    excel_translations.generate(xlsx_file_name);
}

fn export_strings(matches: &ArgMatches) {
    let command = matches.subcommand_matches(COMMAND_EXPORT_STRINGS)
        .unwrap();

    let file_name = command
        .value_of(ARG_FILE_NAME)
        .unwrap();

    let project_name = command
        .value_of(ARG_PROJECT_NAME)
        .unwrap();

    let export_types = command
        .values_of(ARG_EXPORT_STRINGS_TYPE)
        .unwrap();

    let projects_data = get_data(file_name);

    let project = projects_data.projects.iter().find(|&p| {
        p.name.eq(project_name)
    }).expect("Invalid project name");

    for export_type in export_types {
        match export_type {
            "ios" => generate_strings(Ios, &projects_data.translations, &project),
            "and" => generate_strings(Android, &projects_data.translations, &project),
            _ => {  }
        }
    }
}

fn get_arguments() -> ArgMatches {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Simple strings generator and manager.")
        .subcommand(App::new(COMMAND_GENERATE_TEMPLATE)
            .about("Generates project template")
            .arg(Arg::new(ARG_FILE_NAME)
                .required(true)
                .takes_value(false)
                .about("Data file name")
            )
        )
        .subcommand(App::new(COMMAND_IMPORT_XLSX)
            .about("Imports data from xlsx file")
            .arg(Arg::new(ARG_FILE_NAME)
                .required(true)
                .takes_value(false)
                .about("Data file name")
            )
            .arg(Arg::new(ARG_INPUT_FILE_NAME)
                .required(true)
                .takes_value(false)
                .about("File to import")
            )
            .arg(Arg::new(ARG_PROJECT_NAME)
                .required(true)
                .takes_value(false)
                .about("Project name")
            )
            .arg(Arg::new(ARG_IMPORT_IGNORE_UNKNOWN_KEYS)
                .required(false)
                .takes_value(false)
                .about("Ignores unknown keys, will not add them to data file")
                .short('i')
                .long("ignore-unknown")
            )
        )
        .subcommand(App::new(COMMAND_EXPORT_XLSX)
            .about("Exports data to xlsx file")
            .arg(Arg::new(ARG_FILE_NAME)
                .required(true)
                .takes_value(false)
                .about("Data file name")
            )
            .arg(Arg::new(ARG_OUTPUT_FILE_NAME)
                .required(true)
                .takes_value(false)
                .about("File name to export xlsx")
            )
            .arg(Arg::new(ARG_PROJECT_NAME)
                .required(true)
                .takes_value(false)
                .about("Project to export")
            )
        )
        .subcommand(App::new(COMMAND_EXPORT_STRINGS)
            .about("Exports strings for iOS and/or Android targets")
            .arg(Arg::new(ARG_FILE_NAME)
                .required(true)
                .takes_value(false)
                .about("Data file name")
            )
            .arg(Arg::new(ARG_PROJECT_NAME)
                .required(true)
                .takes_value(false)
                .about("Project to export")
            )
            .arg(Arg::new(ARG_EXPORT_STRINGS_TYPE)
                .required(true)
                .min_values(1)
                .multiple_values(true)
                .possible_values(["and", "ios"].as_ref())
            )
        )
        .get_matches();
    matches
}

fn get_data(file_name: &str) -> DataRoot {
    let data = load_data_file(file_name);
    let projects_data = get_projects(&data)
        .unwrap_or_else(|err| {
            println!("Cannot open data file!\nerror: {}", err);
            process::exit(1)
        });

    projects_data
}

fn load_data_file(name: &str) -> String {
    let string_data = fs::read_to_string(name)
        .unwrap_or_else(|err| {
            println!("Cannot open file \"{}\"\nerror: {}",  name, err);
            process::exit(1);
        });

    string_data
}
