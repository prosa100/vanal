use std::vec::Vec;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::i16;
use libflate::gzip::Decoder;
use tar::Archive;
use hound::WavReader;

const README_NAME: &str = "README";
const AUDIO_EXTENSION: &str = "wav";
pub const SAMPLE_RATE:usize = 16000;
const NUM_CHANNELS: usize = 1;

fn read_wav<R>(reader:R) -> Vec<f32>
where R: Read
{
    let reader = WavReader::new(reader).unwrap();

    let (sample_rate, _bit_depth, num_channels) = {
        let spec = reader.spec();
        (spec.sample_rate as usize, spec.bits_per_sample as usize, spec.channels as usize)
    };
    assert_eq!(num_channels, NUM_CHANNELS);
    assert_eq!(sample_rate, SAMPLE_RATE);
    let samples = reader.into_samples::<i16>();
    let samples = samples.map(|x| x.unwrap() as f32 / i16::MAX as f32);
    samples.collect()
    //samples.chunks(num_channels).into_iter().map(|s|[s]).collect()
}

#[derive(Debug)]
pub enum Gender {
    Male,
    Female,
    Other
}

pub fn load_voxforge<R: io::Read>(file : R) -> Option<(Gender, Vec<Vec<f32>>)>{
    let file = Decoder::new(file).unwrap();
    let mut archive = Archive::new(file);

    // going to assume non-gender-conforming.
    let mut gender: Gender = Gender::Other;
    let samples = archive.entries().unwrap().filter_map(|file|{
        let mut file =  file.unwrap();
        let path = file.header().path().unwrap().into_owned();
        let is_readme = path.file_name().map_or(false,|x|x.eq(README_NAME));
        let is_soundfile = path.extension().map_or(false,|x|x.eq(AUDIO_EXTENSION));
        if is_readme {
            let mut s = String::new();
            file.read_to_string(&mut s).unwrap();

            let config = s.split('\n').filter_map(
                |line|line.find(':').map(
                    |i|{let (key, value) = line.split_at(i);
                        let value = value[1..].trim();
                        (key, value)})
                ).collect::<HashMap<_,_>>();
                gender = match *config.get("Gender").unwrap() {
                    "Male" => Gender::Male,
                    "Female" => Gender::Female,
                    _ => Gender::Other,
                };
                return None;
        }
        if is_soundfile {
            return Some(read_wav(file));
        }
        return None;
    }).collect();
    Some((gender, samples))
}

#[test]
fn train_data_load_voxforge() {
    use std::fs::File;
    let file = File::open("assets/1028-20100710-hne.tgz").expect("Test data missing!");
    load_voxforge(file).expect("Failed to parse");
}


// #[derive(Debug)]
// struct TrainData {
//     gender: Gender,
//     sample: Vec<f32>
// }
// impl TrainData {
//     fn new(gender:Gender, sample:Vec<f32>)->TrainData{
//         TrainData{ gender:gender, sample : sample }
//     }
// }
