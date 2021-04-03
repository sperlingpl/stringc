use std::env;
use std::fs;
use std::process;
use clap::{App, Arg, ArgMatches};
use crate::ios_generator::{TranslationsIOS, TranslationOut};
use crate::strings_generator::Generator;
use crate::translation::{Project, Translation, Translations};

mod translation;
mod ios_generator;
mod strings_generator;
mod excel_writer;

const DATA_FILE_ARG: &str = "data";
const PROJECT_ARG: &str = "project";
const OUTPUT_ARG: &str = "output";
const VERBOSE_ARG: &str = "verbose";

fn main() {
    let matches = get_arguments();
    let data_file_name = matches.value_of(DATA_FILE_ARG)
        .expect("No file provided!");

    let data = load_data_file(&String::from(data_file_name));
    let project_data = translation::get_projects(&data)
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

    let translations: Vec<&Translations> = project_data.translations.iter()
        .filter(|&translation| {
            translation.projects.contains(&project.id)
        })
        .collect();

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
}

fn generate_ios_strings(project: &Project, translations: &Vec<&Translations>) {
    for lang in &project.langs.clone() {
        let mut out_translations: Vec<TranslationOut> = vec![];

        for translation in translations {
            let value = translation.values.iter()
                .filter(|&values| {
                    values.lang.eq(lang)
                })
                .collect::<Vec<&Translation>>();

            let first_value = value.first();

            let mut out_value = "";

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
}

fn get_arguments() -> ArgMatches {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Simple strings generator and manager.")
        .arg(Arg::new(DATA_FILE_ARG)
            .required(true)
            .takes_value(false)
        )
        .arg(Arg::new(PROJECT_ARG)
            .short('p')
            .long("project")
            .about("Selects project to work on")
            .takes_value(true)
            .required(true)
        )
        .arg(Arg::new(OUTPUT_ARG)
            .short('o')
            .long("out")
            .about("Comma separated strings output for iOS and/or Android targets. ex. \"-o and,ios\"")
            .takes_value(true)
        )
        .arg(Arg::new(VERBOSE_ARG)
            .short('v')
            .long("verbose")
            .about("Verbose mode")
            .takes_value(false)
        ).get_matches();
    matches
}

fn load_data_file(name: &String) -> String {
    let string_data = fs::read_to_string(name)
        .unwrap_or_else(|err| {
            println!("Cannot open file \"{}\"\nerror: {}",  name, err);
            process::exit(1);
        });

    return string_data;
}
