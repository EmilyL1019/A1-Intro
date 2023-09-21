use std::{env, process::exit};
use csc411_image::{Read, GrayImage, Write};

fn brightness(image: GrayImage) -> u64{
    let mut total_brightness:u64 = 0;
    for pix in image.pixels {
        total_brightness = total_brightness + pix.value as u64;
    }
    return total_brightness / (image.height * image.width) as u64;
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2{
        panic!("Wrong number of inputs. Enter 1 image file.");
    }
    let image = csc411_image::read(env::args().nth(1).as_deref()).unwrap();
    println!("Average: {}", brightness(image));
}
