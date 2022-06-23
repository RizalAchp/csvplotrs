mod csvplot;
mod data;

use clap::*;
// use data::*;
use std::path::PathBuf;

use crate::{csvplot::*, data::get_data_generic};

const OUTPUT: &str = "./output.png";

/// simple cli app for converting csv data to plot
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// just list the data on csv
    #[clap(short, long)]
    lists: bool,

    /// input for csv file
    #[clap(short, long, parse(from_os_str))]
    input: PathBuf,

    /// output image untuk plot, hanya bisa menggunakan png
    #[clap(short, long, parse(from_os_str), default_value = OUTPUT)]
    output: PathBuf,

    #[clap(subcommand)]
    command: Option<Cmd>,
}
#[derive(Debug, Subcommand)]
enum Cmd {
    Generate {
        #[clap(short, long)]
        split: bool,

        #[clap(short, long, default_value = "Graph DynoTest")]
        name: String,

        #[clap(short, default_value = "1280")]
        lebar: u32,

        #[clap(short, default_value = "720")]
        tinggi: u32,
    },
}

fn check_file<'a>(entry: &'a PathBuf, ext: &'a str) -> bool {
    entry
        .extension()
        .map(|s| s.to_str().unwrap().contains(ext))
        .unwrap_or(false)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    println!("test: {:?}", args);

    if args.input.exists() == false {
        return Err(Box::new(FileIOError(
            format!(
                "file input: {} permission error or broken symbolic links!",
                args.input.to_str().unwrap()
            )
            .into(),
        )));
    }

    if check_file(&args.input, "csv") == false {
        return Err(Box::new(FileIOError(
            format!("file: {} is not csv file!", args.input.to_str().unwrap()).into(),
        )));
    }

    if args.lists {
        let tempdata = get_data_generic(&args.input);
        for (i, data) in tempdata.into_iter().enumerate() {
            print!("Data {}", i);
            for d in data {
                print!("[{}]", d);
            }
            print!("\n");
        }
        return Ok(());
    }

    if check_file(&args.output, "png") == false {
        return Err(Box::new(FileIOError(
            format!(
                "file: {} invalid output format! use `file.png` instead",
                args.input.to_str().unwrap()
            )
            .into(),
        )));
    }

    match args.command {
        Some(Cmd::Generate {
            split,
            name,
            lebar,
            tinggi,
        }) => {
            if split {
                match gen_split_plot(&name, &args.input, &args.output, (lebar, tinggi)) {
                    Ok(_) => {
                        println!(
                            "Done Generatng Plot Image: {}",
                            args.output.to_str().unwrap_or(OUTPUT)
                        );
                    }
                    Err(_) => {
                        return Err(Box::new(PlotCsvError(format!(
                            "got error when generating plot from data csv"
                        ))));
                    }
                }
            } else {
                match gen_plot(&name, &args.input, &args.output, (lebar, tinggi)) {
                    Ok(_) => {
                        println!(
                            "Done Generatng Plot Image: {}",
                            args.output.to_str().unwrap_or(OUTPUT)
                        );
                    }
                    Err(_) => {
                        return Err(Box::new(PlotCsvError(format!(
                            "got error when generating plot from data csv"
                        ))));
                    }
                };
            }
        }
        None => return Ok(()),
    }

    Ok(())
}
