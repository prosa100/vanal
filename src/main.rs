#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate prophet;

extern crate bincode;

extern crate rayon; use rayon::prelude::*;

extern crate hound;
extern crate stft;
extern crate libflate;
extern crate tar;
extern crate itertools;
extern crate image;






pub mod train;
pub mod spectrogram;


pub fn main(){
    train::data::dump_train_data();
}
