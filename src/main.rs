use std::env;
use std::f32::consts::PI;
use std::fs::read_to_string;
use std::i16;
use std::io::Error;
use std::path::Path;
fn main() {
    let args: Vec<String> = env::args().collect();
    let target_file = &args[1];
    let mut output_file = &args[2];

    let target_file_path = std::path::Path::new(target_file);
    // let output_file_path = std::path::Path::new(output_file);

    let output_file = if !output_file.ends_with(".wav") {
        output_file.to_string() + ".wav"
    } else {
        output_file.to_string()
    };

    println!("Target file: {}, Output file: {}", target_file, output_file);

    let target_file_path = match file_exists(target_file_path) {
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
    for t in (0..44100).map(|x| x as f32 / 44100.0) {
        let sample = (t * 440.0 * 2.0 * PI).sin();
        let amplitude = i16::MAX as f32;
        writer.write_sample((sample * amplitude) as i16).unwrap();
    }
    writer.finalize().unwrap();
}

fn file_exists(target_file: &Path) -> Result<bool, Error> {
    let target_file_path = std::path::Path::new(target_file);
    if target_file_path.exists() {
        Ok(true)
    } else {
        Err(Error::new(std::io::ErrorKind::NotFound, "File not found"))
    }
}
