use std::path::PathBuf;
use std::fs::create_dir;
use clap::Parser;
use utils::{file_to_image, image_to_file};
mod utils;



#[derive(Parser)]
#[command(version, about)]
struct ArgParser {
    /// Input path (image directory or file path)
    #[arg(value_name = "INPUT")]
    input: PathBuf,
   
    /// Output path (Directory or file path) [default: None]
    #[arg(short, long, default_value = None)]
    output: Option<PathBuf>,

    /// Converts image to file if true [default: false]
    #[arg(short, long, default_value_t = false)]
    image: bool,

    /// [Not implemented] Zip the output if --image set to false, 
    /// unzip the input otherwise
    #[arg(short, long, default_value_t = false)]
    zip: bool,

    /// Image width, only used if --image set to false
    #[arg(short='x', long, default_value_t = 1000)]
    width: u32,

    /// Image height, only used if --image set to false
    #[arg(short='y', long, default_value_t = 1000)]
    height: u32
}


fn main() {
    let arguments: ArgParser = ArgParser::parse();
    let input_path: PathBuf = arguments.input;
    if !input_path.exists() {
        println!("Input path \"{}\" doesn't exist.", input_path.display());
        return
    }

    let mut output_path: PathBuf = arguments.output.unwrap_or(PathBuf::from("."));
    
    if arguments.image {
        image_to_file(&input_path, &output_path);
    } else {
        if output_path == PathBuf::from(".") {
            let dir_name: String = format!("{}_image", input_path.file_stem()
            .unwrap()
            .to_str()
            .unwrap());

            output_path = output_path.join(dir_name);
        }
        if !output_path.exists() {
            create_dir(&output_path).expect(
                &format!("Couldn't create directory \"{}\"", output_path.display())
            );
        }
        
        file_to_image(
            arguments.width,
            arguments.height, 
            &input_path,
            &output_path
        );
    }
}