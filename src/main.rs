use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::process;

use clap::{App, Arg, ArgMatches};

use lib::json_data::{DataRoot, generate_template, get_projects, Project, save, Translation};

use lib::excel_reader::import_excel;
use lib::excel_writer::{ExcelTranslation, ExcelTranslations};
use lib::ios_generator::{TranslationOut, TranslationsIOS};
use lib::strings_generator::Generator;


const COMMAND_GENERATE_TEMPLATE: &str = "generate";
const COMMAND_IMPORT_XLSX: &str = "import";
const COMMAND_EXPORT_XLSX: &str = "export-xlsx";
const ARG_FILE_NAME: &str = "file_name";
const ARG_INPUT_FILE_NAME: &str = "input_file_name";
const ARG_IMPORT_IGNORE_UNKNOWN_KEYS: &str = "ignore_unknown_keys";
const ARG_OUTPUT_FILE_NAME: &str = "export_file_name";
const ARG_PROJECT_NAME: &str = "project_name";

fn main() {
    let matches = get_arguments();

    match matches.subcommand_name() {
        Some(COMMAND_GENERATE_TEMPLATE) => generate_template_command(&matches),
        Some(COMMAND_IMPORT_XLSX) => import_xlsx_command(&matches),
        Some(COMMAND_EXPORT_XLSX) => export_xlsx_command(&matches),
        _ => {
            println!("error: No command provided");
            process::exit(1);
        }
    }

    /*let data_file_name = matches.value_of(DATA_FILE_ARG)
        .expect("No file provided!");

    let data = load_data_file(&String::from(data_file_name));
    let project_data = json_data::get_projects(&data)
        .unwrap_or_else(|err| {
            println!("Cannot read project test_data!\nerror: {}", err);
            process::exit(1);
        });

    let project_name = matches.value_of(PROJECT_ARG)
        .expect("No project name provided!");

    let project: &Project = project_data.projects.iter()
        .filter(|&project| {
            project.name.eq(project_name)
        })
        .collect::<Vec<&Project>>()
        .first()
        .expect("Not valid project name!");

    let translations: Vec<&Translation> = project_data.translations.iter()
        .filter(|&translation| {
            translation.projects.contains(&project.id)
        })
        .collect();

    if let Some(input_excel_file) = matches.value_of(IMPORT_XLSX_ARG) {
        read_excel(input_excel_file);
    }

    if let Some(output_list) = matches.value_of(OUTPUT_ARG) {
        let output_types: Vec<&str> = output_list.split(",").collect();
        for out_type in output_types {
            match out_type {
                "and" => println!("and"),
                "ios" => generate_ios_strings(project, &translations),
                _ => {
                    println!("error: Unknown output value: {}", out_type);
                    process::exit(1);
                }
            }
        }
    }

    if let Some(excel_file_name) = matches.value_of(EXPORT_XLSX_ARG) {
        export_xlsx(project, translations, excel_file_name);
    }*/
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

    let ignore_unknown = matches.subcommand_matches(COMMAND_IMPORT_XLSX)
        .unwrap()
        .is_present(ARG_IMPORT_IGNORE_UNKNOWN_KEYS);

    let mut projects_data = get_data(file_name);

    import_excel(xlsx_file_name, &mut projects_data, 1, ignore_unknown);
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

/*fn export_xlsx(project: &Project, translations: Vec<&Translation>, file_name: &str) {
    let mut out_translations: Vec<ExcelTranslation> = vec![];

    for translation in translations {
        let translation_values: Vec<&TranslationValue> = translation.values.iter()
            .filter(|&translation| {
                translation.project.eq(&project.id)
            })
            .collect();

        let mut lang_values: Vec<String> = vec![];

        for lang in &project.langs {
            let value = translation_values.iter()
                .find(|&&item| item.lang.eq(lang));

            match value {
                Some(&val) => {
                    lang_values.push(val.value.to_string());
                },
                _ => {
                    lang_values.push("".to_string());
                }
            }
        }

        // out_translations.push(ExcelTranslation::new(translation.key.to_string(), lang_values));
    }

    let excel_translations = ExcelTranslations::new(project.langs.clone(), out_translations);
    excel_translations.generate(file_name);
}
*/
/*fn generate_ios_strings(project: &Project, translations: &Vec<&Translation>) {
    for lang in &project.langs.clone() {
        let mut out_translations: Vec<TranslationOut> = vec![];

        for translation in translations {
            let value = translation.values.iter()
                .filter(|&values| {
                    values.lang.eq(lang)
                })
                .collect::<Vec<&TranslationValue>>();

            let first_value = value.first();

            let out_value: &str;

            if let Some(v) = first_value {
                out_value = v.value.as_str()
            } else {
                out_value = translation.key.as_str();
            }

            let out_translation = TranslationOut { key: translation.key.clone(), value: out_value.to_string() };
            out_translations.push(out_translation);
        }

        let trans_ios = TranslationsIOS { lang: lang.to_string(), translations: out_translations };
        trans_ios.generate();
    }
}*/

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
        // .arg(Arg::new(DATA_FILE_ARG)
        //     .required(true)
        //     .takes_value(false)
        // )
        // .arg(Arg::new(PROJECT_ARG)
        //     .short('p')
        //     .long("project")
        //     .about("Selects project to work on")
        //     .takes_value(true)
        //     .required(true)
        // )
        // .arg(Arg::new(OUTPUT_ARG)
        //     .short('o')
        //     .long("out")
        //     .about("Comma separated strings output for iOS and/or Android targets.")
        //     .possible_values(["and", "ios", "and,ios"].as_ref())
        //     .takes_value(true)
        // )
        // .arg(Arg::new(VERBOSE_ARG)
        //     .short('v')
        //     .long("verbose")
        //     .about("Verbose mode")
        //     .takes_value(false)
        // )
        // .arg(Arg::new(OMIT_TRANSLATED_XLSX_ARG)
        //     .short('g')
        //     .long("ignore-translated")
        //     .about("Omit translated strings when exporting to Excel.")
        //     .takes_value(false)
        // )
        // .arg(Arg::new(EXPORT_XLSX_ARG)
        //     .short('e')
        //     .long("export-xlsx")
        //     .about("Exports strings to Excel (xlsx) file with given file name.")
        //     .takes_value(true)
        // )
        // .arg(Arg::new(IMPORT_XLSX_ARG)
        //     .short('i')
        //     .long("import-xlsx")
        //     .about("Import Excel (xlsx) file with given name.")
        //     .takes_value(true)
        // )
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

    return string_data;
}
