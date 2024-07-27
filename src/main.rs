use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;
use std::path::Path;
fn main() {
    let args: Vec<String> = env::args().collect();
    let target_file = &args[1];
    let mut output_file = &args[2];
    let operation = &args[3];

    if operation == "encode" {
        encode(target_file, output_file);
    } else if operation == "decode" {
        decode(target_file, output_file);
    } else {
        panic!("Invalid operation");
    }
}

fn file_exists(target_file: &Path) -> Result<bool, Error> {
    let target_file_path = std::path::Path::new(target_file);
    if target_file_path.exists() {
        Ok(true)
    } else {
        Err(Error::new(std::io::ErrorKind::NotFound, "File not found"))
    }
}

fn encode(target_file: &str, output_file: &str) {
    println!("Encoding file");
    println!("Target file: {}, Output file: {}", target_file, output_file);
    let target_file_path = std::path::Path::new(target_file);
    // let output_file_path = std::path::Path::new(output_file);

    let output_file = if !output_file.ends_with(".wav") {
        output_file.to_string() + ".wav"
    } else {
        output_file.to_string()
    };

    println!("Target file: {}, Output file: {}", target_file, output_file);

    let _target_file_path = match file_exists(target_file_path) {
        Ok(true) => true,
        Ok(false) => panic!("File not found"),
        Err(_) => panic!("File not found"),
    };

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(output_file, spec).unwrap();

    let mut target_file = File::open(target_file).unwrap();
    let mut data = vec![];
    target_file.read_to_end(&mut data).unwrap();

    // println!("Data: {:?}", data);

    for byte in data {
        let sample = byte as f32 / 255.0;
        let amplitude = i16::MAX as f32;
        writer.write_sample((sample * amplitude) as i16).unwrap();
    }
    writer.finalize().unwrap();
}

fn decode(target_file: &str, output_file: &str) {
    todo!()
    // println!("Decoding file");
    // println!("Target file: {}, Output file: {}", target_file, output_file);

    // let mut reader = hound::WavReader::open(target_file).unwrap();

    // let data: i16 = reader.samples::<i16>().collect().unwrap();
    // let mut output_file = File::create(output_file).unwrap();
    // output_file.write_all(&data.collect::<Vec<u8>>()).unwrap();
}
