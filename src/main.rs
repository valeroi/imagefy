#![allow(unused_variables, dead_code)]
use std::{
    fs::{self, File}, io::{BufWriter, Read, Write}, path::Path
};
use image::{
    ImageEncoder,
    ExtendedColorType,
    codecs::png::{PngEncoder, CompressionType, FilterType}
};



fn write_image(width: u32, height: u32, mut pixels: Vec<u8>, image_path: &Path) {
    let capacity: usize = (width*height*3) as usize;
    pixels.resize(capacity, 0);

    let output_file = File::create(image_path).unwrap();
    let writer = BufWriter::new(output_file);

    let encoder = PngEncoder::new_with_quality(
        writer, 
        CompressionType::Best, 
        FilterType::NoFilter
    );
    encoder.write_image(&pixels, width, height, ExtendedColorType::Rgb8).unwrap();
}


fn read_image(image_path: &Path) -> Vec<u8> {
    let image_obj = image::open(image_path).unwrap();
    
    println!("{} {}", image_obj.width(), image_obj.height());
    return Vec::from(image_obj.as_bytes())
}


fn get_file_size(file_path: &Path) -> usize {
    let _metadata = fs::metadata(file_path).unwrap();
    _metadata.len() as usize
}


fn file_to_image(width: u32, height: u32, file_path: &Path) {
    let capacity: usize = (width*height*3) as usize;
    let mut file_obj: File  = File::open(file_path).unwrap();
    let file_size:    usize = get_file_size(file_path);
    let file_name:    &[u8]   = file_path.file_name().unwrap().as_encoded_bytes();
    let file_name:    Vec<u8> = Vec::from(file_name);

    let mut first_bytes: Vec<u8> = vec![0u8; capacity - file_name.len() - 16];
    file_obj.read(&mut first_bytes).unwrap();

    first_bytes.splice(0..0, file_size.to_le_bytes());
    first_bytes.splice(0..0, file_name.clone());
    first_bytes.splice(0..0, file_name.len().to_le_bytes());

    let image_count: f64   = file_size as f64 / capacity as f64;
    let image_count: usize = image_count.ceil() as usize;

    let output_dir = Path::new("./input");
    let output_file = output_dir.join("00000.png");

    println!("{} / {} = {}", file_size, capacity, image_count);
    write_image(width, height, first_bytes, &output_file);

    for i in 1..image_count {
        let output_name = format!("{:05}.png", i);
        println!("{}", output_name);
        let output_path = output_dir.join(output_name);
        let mut pixels = vec![0u8; capacity];
        file_obj.read(&mut pixels).unwrap();
        write_image(width, height, pixels, &output_path);
    }
}


fn image_to_file(dir_path: &Path) {
    let mut dir_iterator = dir_path.read_dir().unwrap();
    let first_image = dir_iterator.next()
    .unwrap()
    .unwrap()
    .path();
    let output_dir = Path::new("output");
    let first_image = Path::new(&first_image);
    let mut file_bytes = read_image(first_image);

    let name_length: Vec<u8> = file_bytes.drain(0..8).collect();
    let name_length: [u8; 8] = name_length.try_into().unwrap();
    let name_length: usize   = usize::from_le_bytes(name_length);
    let file_name: Vec<u8> = file_bytes.drain(0..name_length).collect();
    let file_name: String  = String::from_utf8(file_name).unwrap();
    let file_size: Vec<u8> = file_bytes.drain(0..8).collect();
    let file_size: [u8; 8] = file_size.try_into().unwrap();
    let mut file_size: usize   = usize::from_le_bytes(file_size);

    let file_path = output_dir.join(&file_name);
    let file_obj = File::create(file_path).unwrap();
    let mut writer = BufWriter::new(file_obj);

    if file_bytes.len() > file_size {
        file_bytes.resize(file_size, 0);
    } else {
        file_size -= file_bytes.len();
    }
    writer.write(file_bytes.as_ref()).unwrap();

    println!("file name: {}\nfile size:{} bytes", file_name, &file_size);

    for image_path in dir_iterator {
        let image_path = image_path.unwrap().path();
        let image_path = Path::new(&image_path);
        let mut file_bytes = read_image(image_path);
        if file_bytes.len() > file_size {
            file_bytes.resize(file_size, 0);
        } else {
            file_size -= file_bytes.len();
        }

        writer.write(&file_bytes.as_ref()).unwrap();
        println!("{}", image_path.display());
    }
}



fn main() {
    let width: u32  = 1000;
    let height: u32 = 1000;
    let capacity: u32 = width * height * 3;

    let file_path = Path::new("teste.exe");
    let input_dir = Path::new("input");

    //image_to_file(input_dir);
    //file_to_image(width, height, file_path);
}