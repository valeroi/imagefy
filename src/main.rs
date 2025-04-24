use std::{
    path::PathBuf,
    fs::create_dir
};
use anyhow::{ensure, Context, Error, Result};
use clap::Parser;
use utils::{
    file_to_image, 
    image_to_file,
    confirm,
    print_error
};

mod utils;


#[derive(Parser, Debug)]
#[command(
    about = "Imagefy 1.2.2: a tool to convert files to images
Valeroi <valerio@valerio.valerio>",
    version = "1.2.2",
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



/// Raises error if paths in `args` are incompatible.
fn check_arguments(args: &mut ArgParser) -> Result<()> {
    let mut input_paths: Vec<PathBuf> = args.input.clone();

    // Checking input path
    ensure!(!input_paths.is_empty(), "No input file detected.");

    let error_path: Option<&PathBuf> = input_paths.iter().find(|entry| !entry.exists());
    ensure!(error_path.is_none(), "Path \"{}\" doesn't exist.",
    error_path.unwrap().display());

    if !args.image {
        let error_path: Option<&PathBuf> = input_paths.iter().find(|entry| entry.is_dir());
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
        let error_path: Option<&PathBuf> = input_paths.iter().find(|entry| entry.is_file() && 
        entry.extension().unwrap_or_default() != "png");
        ensure!(error_path.is_none(), "Path \"{}\" is not an image.",
        error_path.unwrap().display());
    }
    args.input = input_paths.clone();

    // Checking output path
    let mut output_path: PathBuf = args.output.clone().unwrap_or(
    PathBuf::from("."));
    let input_name: &str = input_paths[0].file_stem().unwrap()
    .to_str().unwrap();

    if !args.image {
        if output_path.is_dir() {
            output_path = output_path.join(format!("{}_image", input_name));
        }
        ensure!(!output_path.is_dir(), "Directory \"{}\" already exists.",
        output_path.display());

        println!("Creating directory \"{}\"", output_path.display());
        confirm("Continue?", args.yes)?;
        create_dir(&output_path).context("")?;
    } 
    else {
        ensure!(!output_path.is_file(), "File \"{}\" already exists, try -o [PATH].", 
        output_path.display())
    }
    
    args.output = Some(output_path);
    Ok(())
}

/// Handles any error that function `run()` may return.
fn error_handler(error_obj: Error) {
    let error_text: String = error_obj.to_string();
    let error_text: &str = error_text.as_str();

    match error_text {
        "parser" => println!("{}", error_obj.root_cause().to_string()),
        _ => print_error(error_text)
    }
}


/// Where everything happens.
fn run() -> Result<()> {
    let mut args: ArgParser = ArgParser::try_parse().context("parser")?;
    check_arguments(&mut args)?;
    
    let width:  u32 = args.width;
    let height: u32 = args.height;

    let file_input: Vec<PathBuf> = args.input;
    let file_output: PathBuf = args.output.unwrap();

    if args.image {
        image_to_file(file_input, &file_output)?;
    } else {
        let file_input: &PathBuf = file_input.get(0).unwrap();
        file_to_image(width, height, file_input, &file_output)?;
    }
    
    Ok(())
}

fn main() {
    match run() {
        Ok(_) => println!("Done!"),
        Err(error_obj) => error_handler(error_obj)
    }
}