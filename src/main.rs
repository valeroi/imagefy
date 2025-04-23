#![allow(unused)]
use std::{
    io::{self, Write},
    path::PathBuf,
    fs::create_dir
};
use anyhow::{bail, ensure, Context, Error, Result};
use clap::{error, Parser};
use colored::Colorize;
use utils::{file_to_image, image_to_file};

mod utils;


#[derive(Parser, Debug)]
#[command(
    about = "Imagefy 1.0.6: a tool to convert files to images
Valeroi <valerio@valerio.valerio>",
    version = "1.0.6",
    override_usage = "imagefy <INPUT> [OPTIONS]...",
    after_help = "Examples:
File to image -> imagefy example.exe -o ./example_image/
Images to file -> imagefy ./example_image/ --image -o example.exe")
]
struct ArgParser {
    /// Image directory, image(s) path or file path
    input: Vec<PathBuf>,
   
    /// Output path (directory or file path) [default: None]
    #[arg(short, long, default_value = None)]
    output: Option<PathBuf>,

    /// Converts image to file if true [default: false]
    #[arg(short, long, default_value_t = false)]
    image: bool,

    /// Image width, only used if --image set to false
    #[arg(long, default_value_t = 1000)]
    width: u32,

    /// Image height, only used if --image set to false
    #[arg(long, default_value_t = 1000)]
    height: u32,

    /// Skip confirmation prompts
    #[arg(short, default_value_t = false)]
    yes: bool
}

/// Prints `message` in yellow.
fn print_warn(message: &str) {
    println!("{} {}", "[!]".yellow().bold(), message.bright_yellow());
}

/// Prints `message` in red.
fn print_error(message: &str) {
    eprintln!("{} {}", "[X]".red().bold(), message.bright_red());
}

/// Asks for user confirmation if ignore is set to false. Raises error on decline.
fn confirm(message: &str, ignore: bool) -> Result<()> {
    if ignore {
        return Ok(())
    }
    let answer = "y";
    let mut input = String::new();

    print!("{} [{}]: ", message, answer.to_uppercase());
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_lowercase();
    
    ensure!(input == answer || input.len() == 0, "Confirmation failed.");
    Ok(())
}

/// Raises error if paths in `args` are incompatible.
fn check_arguments(args: &mut ArgParser) -> Result<()> {
    let mut input_paths = args.input.clone();

    // Checking input path
    ensure!(!input_paths.is_empty(), "No input file detected.");

    let error_path = input_paths.iter().find(|entry| !entry.exists());
    ensure!(error_path.is_none(), "Path \"{}\" doesn't exist.",
    error_path.unwrap().display());

    if !args.image {
        let error_path = input_paths.iter().find(|entry| entry.is_dir());
        ensure!(input_paths.len() == 1, "Multiple input files are not allowed.");
        ensure!(error_path.is_none(), "Path \"{}\" is not a file.",
        error_path.unwrap().display());
    }

    if args.input.len() == 1 && args.input[0].is_dir() {
        input_paths = args.input[0]
        .read_dir()?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        //.filter(|entry| entry.extension().unwrap_or_default() == "png")
        .collect();
    }
    
    if args.image {
        let error_path = input_paths.iter().find(|entry| entry.is_file() && 
        entry.extension().unwrap_or_default() != "png");
        ensure!(error_path.is_none(), "Path \"{}\" is not an image.",
        error_path.unwrap().display());
    }
    args.input = input_paths.clone();

    // Checking output path
    let mut output_path = args.output.clone().unwrap_or(
    PathBuf::from("."));
    let input_name = input_paths[0].file_stem().unwrap();
    let input_name = input_name.to_str().unwrap();

    if !args.image {
        if output_path.is_dir() {
            output_path = output_path.join(format!("{}_image", input_name));
        }
        ensure!(!output_path.is_dir(), "Directory \"{}\" already exists.",
        output_path.display());

        println!("Creating directory \"{}\"", output_path.display());
        confirm("Continue?", args.yes)?;
        create_dir(&output_path).context("");
    } 
    else {
        ensure!(!output_path.is_file(), "File \"{}\" already exists.", 
        output_path.display())
    }
    
    args.output = Some(output_path);
    Ok(())
}

/// Handles every error that may be returned by the function `run()`.
fn error_handler(error_obj: Error) {
    let error_text = error_obj.to_string();
    print_error(error_text.as_str());
}


/// Where everything happens.
fn run() -> Result<()> {
    let mut args: ArgParser = ArgParser::try_parse()?;
    check_arguments(&mut args)?;
    println!("{:#?}", &args);
    let width = args.width;
    let height = args.height;

    let file_input = args.input.get(0).unwrap().parent().unwrap().into();
    let saida = PathBuf::from("./bunda.txt");
    let file_output = args.output.unwrap();

    if args.image {
        image_to_file(&file_input, &saida);
    } else {
        file_to_image(width, height, &file_input, &saida)
    }
    
    Ok(())
}

fn main() {
    match run() {
        Ok(_) => println!("Finished."),
        Err(error_obj) => error_handler(error_obj)
    }
}