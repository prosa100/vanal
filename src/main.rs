#![allow(dead_code)]

#[macro_use]
extern crate prophet;
extern crate rayon;
extern crate hound;
extern crate stft;
extern crate libflate;
extern crate tar;
extern crate itertools;
extern crate image;
extern crate byteorder;
pub mod train;
pub mod spectrogram;


pub fn main(){
    train::data::dump_train_data();
    //train::model::_load_train_data();
}
