use super::data::Data;
use super::parse;
use anyhow::Error;
use std::f32::consts::PI;

const SAMPLE_RATE: u32 = 44_100;
const ONE_BEEP_FREQ: f32 = 1000.0;
const ZERO_BEEP_FREQ: f32 = 500.0;
const BEEP_DURATION: f32 = 0.03;
const AMPLITUDE: f32 = 1.;

fn generate_beep_samples(frequency: f32, duration: f32) -> Vec<f32> {
    let total_samples = (duration * SAMPLE_RATE as f32) as usize;
    let mut samples = Vec::with_capacity(total_samples);

    for i in 0..total_samples {
        let sample = AMPLITUDE * (2. * PI * frequency * i as f32 / SAMPLE_RATE as f32).sin();
        samples.push(sample)
    }

    samples
}

fn data_to_wav(data: &Data, filename: &str) -> Result<(), Error> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };

    let mut writer = hound::WavWriter::create(filename, spec)?; // TODO: cringe

    for (index, bit) in data.binary.iter().enumerate() {
        if index % 1000 == 0 {
            println!("{}% done", (index as f32 / data.binary.len() as f32) * 100.);
        }
        let beep_samples = if *bit {
            generate_beep_samples(ONE_BEEP_FREQ, BEEP_DURATION)
        } else {
            generate_beep_samples(ZERO_BEEP_FREQ, BEEP_DURATION)
        };

        for sample in beep_samples {
            writer.write_sample(sample)?;
        }
    }

    writer.finalize()?;

    Ok(())
}

pub fn file_to_sound() {
    let bytes = parse::get_bytes_from_file(None);
    let data: Data = Data::new(&bytes);
    let _ = data_to_wav(&data, "encoded.wav");
}
