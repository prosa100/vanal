use std::path::Path;
use stft::{WindowType, STFT};

use image;
const WINDOW_SIZE: usize = 1024;
const STEP_SIZE: usize = 512;
const SPECTROGRAM_WIDTH: usize = WINDOW_SIZE / 2;

type TSample = f64;

pub struct FFTProcessor {
    stft: STFT<TSample>
}
use std::fmt;
impl fmt::Debug for FFTProcessor{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "FFTProcessor")
    }
}

impl FFTProcessor {
    pub fn process(&mut self, data: &[TSample]) -> Vec<TSample>{
        assert_eq!(data.len(), STEP_SIZE);
        self.stft.append_samples(data);
        let mut out = vec![0.0; SPECTROGRAM_WIDTH];
        self.stft.compute_column(&mut out);
        out
    }
    pub fn new() -> FFTProcessor {
        {
            FFTProcessor {
                stft: STFT::<TSample>::new(WindowType::Hanning, WINDOW_SIZE, STEP_SIZE),
                //specdata_buffer: Box::new([0.0; SPECTROGRAM_WIDTH]),
            }
        }
    }
}

pub fn colorize_sample(s:TSample) -> u8{
    (256.0 * s).min(255.0) as u8
}

pub fn draw(samples: &[TSample], _sample_rate: usize) {
    let mut stft = FFTProcessor::new();
    let image_data = samples.chunks(STEP_SIZE).map(|chunk|stft.process(chunk)).flat_map(|spectra| {
         spectra.into_iter().map(colorize_sample)
    }).collect::<Vec<_>>();
    let image_lines = samples.len() / STEP_SIZE;
    image::save_buffer(
        &Path::new("output/spec.png"),
        &image_data[..],
        SPECTROGRAM_WIDTH as u32,
        image_lines as u32,
        image::ColorType::Gray(8),
    ).expect("File save failed");
}


// for some_samples in samples.chunks(WINDOW_SIZE) {
//      stft.append_samples(some_samples); // add t
//      while stft.contains_enough_to_compute() // seems to be off by one
//      {
//          let mut spectrogram_column = vec![0.0; SPECTROGRAM_WIDTH];
//          stft.compute_column(&mut spectrogram_column);
//          spec_data.push(spectrogram_column.box);
//          for sample in spectrogram_column.iter() {
//              output_image.push((256.0 * sample).min(255.0)  as u8);
//          }
//          stft.move_to_next_column();
//          image_lines+=1;
//      }
// }

pub fn plot_spectrogram() {}
