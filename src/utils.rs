use std::{
    fs::{self, File, Metadata}, 
    io::{self, BufWriter, Read, Write},
    path::PathBuf
};
use image::{
    ImageEncoder,
    DynamicImage,
    ExtendedColorType::Rgb8,
    codecs::png::{PngEncoder, CompressionType, FilterType}
};
use colored::Colorize;
use anyhow::{ensure, Context, Result};


pub fn write_image(
    width: u32, 
    height: u32, 
    mut pixels: Vec<u8>, 
    image_path: &PathBuf,
    compress: bool) -> Result<()> 
    {
    ensure!(!image_path.is_file(), "Image \"{}\" already exists.", 
    image_path.display());
    let capacity: usize = (width*height*3) as usize;
    pixels.resize(capacity, 0);

    let compression = if compress {
        CompressionType::Best
    } else {
        CompressionType::Fast
    };
    
    let output_file: File = File::create(image_path)
    .context(format!("Error creating image \"{}\".", image_path.display()))?;
    
    let writer: BufWriter<File> = BufWriter::new(output_file);

    let encoder: PngEncoder<BufWriter<File>> = PngEncoder::new_with_quality(
        writer,
        compression,
        FilterType::NoFilter
    );
    encoder.write_image(&pixels, width, height, Rgb8)?;

    Ok(())
}

/// Prints `message` in yellow.
pub fn print_info(symbol: &str, message: &str) {
    println!("[{}] {}", symbol.green().bold(), message.bright_green());
}

/// Prints `message` in red.
pub fn print_error(message: &str) {
    eprintln!("{} {}", "[X]".red().bold(), message.bright_red());
}

/// Asks for user confirmation if ignore is set to false. Raises error on decline.
pub fn confirm(message: &str, ignore: bool) -> Result<()> {
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

fn read_image(image_path: &PathBuf) -> Result<Vec<u8>> {
    let image_obj: DynamicImage = image::open(image_path)?;
    Ok(Vec::from(image_obj.as_bytes()))
}


fn get_file_size(file_path: &PathBuf) -> Result<usize> {
    let _metadata: Metadata = fs::metadata(file_path)?;
    Ok(_metadata.len() as usize)
}


pub fn file_to_image(
    width: u32, 
    height: u32, 
    file_path: &PathBuf, 
    output_path: &PathBuf) -> Result<()>
    {
    let capacity: usize = (width*height*3) as usize;
    let mut file_obj: File  = File::open(file_path)?;
    let file_size:    usize = get_file_size(file_path)?;
    let file_name:    &[u8]   = file_path.file_name().unwrap().as_encoded_bytes();
    let file_name:    Vec<u8> = Vec::from(file_name);

    let mut first_bytes: Vec<u8> = vec![0u8; capacity - file_name.len() - 16];
    file_obj.read(&mut first_bytes)?;

    first_bytes.splice(0..0, file_size.to_le_bytes());
    first_bytes.splice(0..0, file_name.clone());
    first_bytes.splice(0..0, file_name.len().to_le_bytes());

    let image_count: f64   = file_size as f64 / capacity as f64;
    let image_count: usize = image_count.ceil() as usize;
    let compress:    bool  = if image_count == 1 {true} else {false};

    let output_image: PathBuf = output_path.join("00000.png");
    print_info("-", &format!("{} images to write.", &image_count));
    write_image(width, height, first_bytes, &output_image, compress)?;

    for i in 1..image_count {
        let output_name:  String  = format!("{:05}.png", i);
        let output_image: PathBuf = output_path.join(output_name);
        let mut pixels:   Vec<u8> = vec![0u8; capacity];
        let compress:     bool    = if i == image_count-1 {true} else {false};

        file_obj.read(&mut pixels)?;
        write_image(width, height, pixels, &output_image, compress)?;
    }

    print_info("=", &format!("result path: \"{}\"", output_path.display()));
    Ok(())
}


pub fn image_to_file(
    input_paths: Vec<PathBuf>,
    output_path: &PathBuf) -> Result<()>
    {
    let mut path_iterator = input_paths.iter();
    let first_image: &PathBuf = path_iterator.next().unwrap();
    let mut file_bytes: Vec<u8> = read_image(&first_image)?;

    let name_length: Vec<u8> = file_bytes.drain(0..8).collect();
    let name_length: [u8; 8] = name_length.try_into().unwrap();
    let name_length: usize   = usize::from_le_bytes(name_length);

    let file_name: Vec<u8> = file_bytes.drain(0..name_length).collect();
    let file_name: String  = String::from_utf8(file_name)?;
    let file_size: Vec<u8> = file_bytes.drain(0..8).collect();
    let file_size: [u8; 8] = file_size.try_into().unwrap();
    let mut file_size: usize   = usize::from_le_bytes(file_size);
    
    let file_path: PathBuf = if output_path.is_dir() {
        output_path.join(&file_name)
    } else {
        output_path.clone()
    };
    ensure!(!file_path.exists(), "File \"{}\" already exists, try -o [PATH].", 
    file_path.display());

    let file_obj: File = File::create(&file_path)?;
    let mut writer: BufWriter<File> = BufWriter::new(file_obj);

    if file_bytes.len() > file_size {
        file_bytes.resize(file_size, 0);
    } else {
        file_size -= file_bytes.len();
    }
    writer.write(file_bytes.as_ref())?;

    print_info("+", &format!("file name: {}", file_name));
    print_info("+", &format!("file size: {} bytes", &file_size));

    for image_path in path_iterator {
        let mut file_bytes: Vec<u8> = read_image(&image_path)?;
        if file_bytes.len() > file_size {
            file_bytes.resize(file_size, 0);
        } else {
            file_size -= file_bytes.len();
        }

        writer.write(&file_bytes.as_ref())?;
    }

    print_info("=", &format!("result path: \"{}\"", file_path.display()));
    Ok(())
}
