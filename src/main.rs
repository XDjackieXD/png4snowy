extern crate png;
extern crate getopts;

use getopts::Options;
use std::env;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use png::{BitDepth, ColorType};

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] INFILE OUTFILE", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let opts = Options::new();
    let matches = match opts.parse(&args[1..]) {
        Err(why) => panic!("Error parsing options: {}", Error::description(&why)),
        Ok(m) => m
    };

    if matches.free.len() != 2 {
        print_usage(&program, opts);
        return;
    }

    let inpath = Path::new(&matches.free[0]);
    let infile = match File::open(&inpath) {
        Err(why) => panic!("Error opening file \"{}\": {}", inpath.display(), Error::description(&why)),
        Ok(file) => file
    };

    let decoder = png::Decoder::new(infile);

    let (info, mut reader) = match decoder.read_info() {
        Err(why) => panic!("Error parsing file \"{}\": {}", inpath.display(), Error::description(&why)),
        Ok(data) => data
    };

    let mut buf = vec![0; info.buffer_size()];

    match reader.next_frame(&mut buf) {
        Err(why) => panic!("Error parsing file \"{}\": {}", inpath.display(), Error::description(&why)),
        Ok(_) => ()
    };

    println!("W: {}, H: {}, Bits: {:?}, ColorType: {:?}", info.width, info.height, info.bit_depth, info.color_type);

    if info.bit_depth != BitDepth::Eight {
        panic!("Image has to have a bit-depth of 8!");
    }

    let outpath = Path::new(&matches.free[1]);
    let outfile = match File::create(&outpath) {
        Err(why) => panic!("Error creating file \"{}\": {}", outpath.display(), Error::description(&why)),
        Ok(file) => file
    };

    let mut outstream = BufWriter::new(outfile);

    let width = info.width as usize;
    let height = info.height as usize;

    match info.color_type {
        ColorType::RGB => {
            for x in 0..width {
                for y in 0..height {
                    println!("x: {}, y: {}", x, y);
                    match outstream.write(&vec![((buf[((height-y-1)*width+x)*3] >> 2) & 0b00110000) | ((buf[((height-y-1)*width+x)*3+1] >> 4) & 0b00001100) | ((buf[((height-y-1)*width+x)*3+2] >> 6) & 0b00000011)]) {
                        Err(why) => panic!("Error writing to file \"{}\": {}", outpath.display(), Error::description(&why)),
                        Ok(size) => if size != 1 {
                            panic!("Error writing to file \"{}\"!", outpath.display());
                        }
                    };
                }
            }
        },
        ColorType::RGBA => {
            for x in 0..width {
                for y in 0..height {
                    match outstream.write(&vec![(buf[((height-y-1)*width+x)*4+3] & 0b11000000) | ((buf[((height-y-1)*width+x)*4] >> 2) & 0b00110000) | ((buf[((height-y-1)*width+x)*4+1] >> 4) & 0b00001100) | ((buf[((height-y-1)*width+x)*4+2] >> 6) & 0b00000011)]) {
                        Err(why) => panic!("Error writing to file \"{}\": {}", outpath.display(), Error::description(&why)),
                        Ok(size) => if size != 1 {
                            panic!("Error writing to file \"{}\"!", outpath.display());
                        }
                    };
                }
            }
        },
        _ => panic!("Image has to be either RGB or RGBA!"),
    };
    /*match info.color_type {
        ColorType::RGB => {
            for x in 0..(info.buffer_size()/3) {
                 match outstream.write(&vec![((buf[x*3] >> 2) & 0b00110000) | ((buf[x*3+1] >> 4) & 0b00001100) | ((buf[x*3+2] >> 6) & 0b00000011)]) {
                     Err(why) => panic!("Error writing to file \"{}\": {}", outpath.display(), Error::description(&why)),
                     Ok(size) => if size != 1 {
                        panic!("Error writing to file \"{}\"!", outpath.display());
                     }
                 };
            }
        },
        ColorType::RGBA => {
            for x in 0..(info.buffer_size()/4) {
                match outstream.write(&vec![(buf[x*4+3] & 0b11000000) | ((buf[x*4] >> 2) & 0b00110000) | ((buf[x*4+1] >> 4) & 0b00001100) | ((buf[x*4+2] >> 6) & 0b00000011)]) {
                    Err(why) => panic!("Error writing to file \"{}\": {}", outpath.display(), Error::description(&why)),
                    Ok(size) => if size != 1 {
                        panic!("Error writing to file \"{}\"!", outpath.display());
                    }
                };
            }
        },
        _ => panic!("Image has to be either RGB or RGBA!"),
    };*/
}
