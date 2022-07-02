mod csvplot;
mod data;

use clap::*;
// use data::*;
use std::path::PathBuf;

use crate::{csvplot::*, data::get_data_generic};

/// simple cli app for converting csv data to plot
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// just list the data on csv
    #[clap(short, long)]
    lists: bool,

    /// input for csv file
    #[clap(value_parser = clap::value_parser!(PathBuf))]
    input: PathBuf,

    /// output image untuk plot, hanya bisa menggunakan png
    #[clap(short, long, parse(from_os_str))]
    output: Option<PathBuf>,

    #[clap(subcommand)]
    command: Option<Cmd>,
}
#[derive(Debug, Subcommand)]
enum Cmd {
    /// generate plot image from given csv file, -h for more configuration
    Gen {
        #[clap(short, long)]
        split: bool,

        #[clap(short, long, default_value = "Plot From CSV")]
        name: String,

        #[clap(short, default_value = "1280")]
        lebar: u32,

        #[clap(short, default_value = "720")]
        tinggi: u32,
    },
}

fn check_file<P, S>(entry: P, ext: S) -> bool
where
    P: AsRef<std::path::Path>,
    S: AsRef<str>,
{
    entry
        .as_ref()
        .extension()
        .map(|s| s.to_str().unwrap().contains(ext.as_ref()))
        .unwrap_or(false)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    #[cfg(debug_assertions)]
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
        return Ok(get_data_generic(&args.input)?);
    }

    let mut outputfile = String::new();
    match args.output {
        Some(p) => {
            if check_file(&p, "png") == false {
                return Err(Box::new(FileIOError(
                    format!(
                        "file: {} invalid output format! use `file.png` instead",
                        args.input.to_str().unwrap()
                    )
                    .into(),
                )));
            } else {
                outputfile.push_str(p.to_str().unwrap());
            }
        }
        None => {
            let mut temp = PathBuf::from(&args.input);
            temp.set_extension("png");
            outputfile.push_str(temp.to_str().unwrap());
        }
    }
    println!("file output: {}", outputfile);

    match args.command {
        Some(Cmd::Gen {
            split,
            name,
            lebar,
            tinggi,
        }) => {
            if split {
                match gen_split_plot(&name, &args.input, &outputfile, (lebar, tinggi)) {
                    Ok(_) => println!("Done Generatng Plot Image: {}", outputfile),
                    Err(_) => {
                        return Err(Box::new(PlotCsvError(format!(
                            "got error when generating plot from data csv"
                        ))));
                    }
                }
            } else {
                match gen_plot(&name, &args.input, &outputfile, (lebar, tinggi)) {
                    Ok(_) => println!("Done Generatng Plot Image: {}", outputfile),
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
