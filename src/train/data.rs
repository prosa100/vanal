use std::vec::Vec;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::i16;
use libflate::gzip::{Decoder,Encoder};
use tar::Archive;
use hound::WavReader;

const README_NAME: &str = "README";
const AUDIO_EXTENSION: &str = "wav";
pub const SAMPLE_RATE: usize = 16000;
const NUM_CHANNELS: usize = 1;

fn read_wav<R>(reader: R) -> Vec<f32>
where
    R: Read,
{
    let reader = WavReader::new(reader).unwrap();

    let (sample_rate, _bit_depth, num_channels) = {
        let spec = reader.spec();
        (
            spec.sample_rate as usize,
            spec.bits_per_sample as usize,
            spec.channels as usize,
        )
    };
    assert_eq!(num_channels, NUM_CHANNELS);
    assert_eq!(sample_rate, SAMPLE_RATE);
    let samples = reader.into_samples::<i16>();
    let samples = samples.map(|x| x.expect("parse fail") as f32 / i16::MAX as f32);
    samples.collect()
    //samples.chunks(num_channels).into_iter().map(|s|[s]).collect()
}

type GenderBinary = bool;
const FEMALE_VAL_BOOL:bool = true;
const MALE_VAL_BOOL:bool = false;

type Gender = Option<GenderBinary>;



pub fn load_voxforge<R: io::Read>(file: R) -> Vec<(Vec<f32>, GenderBinary)> {
    let file = Decoder::new(file).unwrap();
    let mut archive = Archive::new(file);

    // going to assume non-gender-conforming.
    let mut gender = None;
    let samples = archive
        .entries()
        .expect("archive is bad")
        .filter_map(|file| {
            let mut file = file.unwrap();
            let path = file.header().path().unwrap().into_owned();
            let is_readme = path.file_name().map_or(false, |x| x.eq(README_NAME));
            let is_soundfile = path.extension().map_or(false, |x| x.eq(AUDIO_EXTENSION));
            if is_readme {
                let mut s = String::new();
                file.read_to_string(&mut s).expect("failed to read file in acrhive");
                let config = s.split('\n')
                    .filter_map(|line| {
                        line.find(':').map(|i| {
                            let (key, value) = line.split_at(i);
                            let value = value[1..].trim();
                            (key, value)
                        })
                    })
                    .collect::<HashMap<_, _>>();
                gender = match *config.get("Gender").unwrap_or(&"") {
                    "Male" => Some(MALE_VAL_BOOL),
                    "Female" => Some(FEMALE_VAL_BOOL),
                    _ => None,
                };
                return None;
            }
            if is_soundfile {
                return Some(read_wav(file));
            }
            return None;
        })
        .collect::<Vec<_>>();
    
    if gender.is_none() {
        vec![]
    }else{
        let gender=gender.unwrap();
        samples.into_iter().map(|samples|(samples, gender)).collect()
    }
}

// #[test]
// fn train_data_load_voxforge() {
//     let file = fs::File::open("assets/1028-20100710-hne.tgz").expect("Test data missing!");
//     let data = load_voxforge(file);
//     println!("{}", data.len())
// }

use std::fs;
use prophet::prelude::*;
use spectrogram::FFTProcessor;
use std::iter;
use rayon::prelude::*;
//const VOICE_DATA_DIR:&str = "/data/voice/voxforge";
const VOICE_DATA_DIR:&str = "./assets";

fn gender_as_vec(gender:GenderBinary)->Vec<f32>{
   vec![gender as i16 as f32]
}
pub fn  load_train_data() -> Box<Iterator<Item=TrainExample>>
{
        Box::new(fs::read_dir(VOICE_DATA_DIR).unwrap()
        .flat_map(|path| {
            let path = path.unwrap().path(); // maybe i should use glob?
            println!("{:?}", path);
            let file = fs::File::open(path).expect("Failed to load file.");
            load_voxforge(file)
        })
        .flat_map(|(samples, gender)|{
            let gender = iter::repeat(gender_as_vec(gender));
            let mut stft = FFTProcessor::new();
            stft.process(samples.as_slice()).into_iter()
            .zip(gender).map(TrainExample::from)
        }))
}

use bincode::{serialize_into, deserialize, Infinite};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct TrainExample {
    pub features: Vec<f32>,
    pub label: Vec<f32>
}

impl TrainExample{
    pub fn from<A1, A2>(x:(A1,A2))->TrainExample
        where A1: Into<Vec<f32>>,
            A2: Into<Vec<f32>>
    {
        TrainExample::new(x.0.into(),x.1.into())
    }
    pub fn new(features:Vec<f32>,label:Vec<f32>)->TrainExample{
        TrainExample{
            features:features,
            label:label
        }
    }
}

pub fn dump_train_data() {
     use std::time::Instant;    let now = Instant::now();

    let mut file = fs::File::create("train.bin").unwrap();
    let mut file = Encoder::new(file).expect("Somehow creating a compressor failed.");
    for first in load_train_data() {
        serialize_into(&mut file, &first, Infinite).expect("Failed to write");
    }
    println!("{}s", now.elapsed().as_secs());
}


#[test]
fn train_data_load_all() {
    use std::time::Instant;
    let now = Instant::now();
    //load_train_data();
    //println!("{}s", now.elapsed().as_secs());
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
