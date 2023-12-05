use std::{path::PathBuf, fs::{File, self}, io::Read};
use clap::Parser;
use image::{RgbImage, Pixel, Rgb};
use image::io::Reader as ImageReader;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: PathBuf,
}

fn main() {
    let args = Args::parse();

    println!("{:?}", args.file);

    let mut file = File::open(args.file.clone()).unwrap();

    let mut file_buffer = vec![];
    file.read_to_end(&mut file_buffer).unwrap();

    let file_name = args.file.file_name().unwrap().to_string_lossy();

    // file headers and shit
    let mut header_buffer: Vec<u8> = vec![];
    leb128::write::unsigned(&mut header_buffer, file_name.len() as u64).unwrap();
    header_buffer.extend(file_name.as_bytes());
    leb128::write::unsigned(&mut header_buffer, file_buffer.len() as u64).unwrap();

    let mut image_buffer = vec![];
    image_buffer.extend("HIIF".as_bytes());
    leb128::write::unsigned(&mut image_buffer, header_buffer.len() as u64).unwrap();
    image_buffer.extend(header_buffer);
    image_buffer.extend(file_buffer);

    fs::write("test.dump", image_buffer.clone()).unwrap();

    println!("total: {:?}", image_buffer.len());

    //mathing
    let size_required = (image_buffer.len() as f64 / 3.0).ceil() as u32;
    let image_size = (size_required as f64).sqrt().ceil() as u32;

    println!("size required in bytes: {}", size_required);
    println!("image size: {}", image_size);

    let mut image = RgbImage::new(image_size, image_size);

    let mut left = image_buffer.len() as i64;
    let mut index = 0;
    while left > 0 {
        let r = image_buffer.get(index + 0).unwrap_or(&0);
        let g = image_buffer.get(index + 1).unwrap_or(&0);
        let b = image_buffer.get(index + 2).unwrap_or(&0);

        let real_index = index / 3;

        let (x, y) = (real_index as u32 % image_size, real_index as u32 / image_size);

        //println!("{} : {}", x, y);

        image.put_pixel(x, y, Rgb([*r, *g, *b]));

        index += 3;
        left -= 3;

        //println!("{} left", left);
    }

    image.save("test.png").unwrap();

    //reader
    let image_reader = ImageReader::open("test.png").unwrap().decode().unwrap();
    let image = image_reader.as_rgb8().unwrap();

    let mut output = vec![];

    for y in 0..image.height() {
        for x in 0..image.width() {
            let rgb = image.get_pixel(x, y).to_rgb().0;
            //println!("{:#?}", rgb);
            output.extend(rgb);
        }
    }

    fs::write("test2.dump", output).unwrap();

    println!("is done?")
}
