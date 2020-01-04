extern crate image;

use image::GenericImageView;
use std::env;

/*
 * This will generate a string with which QMK and its oled_write_raw_P-method can use
 * to render a bitmap image on an oled.
 *
 * We assume that you'll create an image that has the correct dimensions and that is
 * monochrome. We read the 'luma' value of each pixel and if its above 127, we consider
 * it to be 'on'.
 *
 * There are no checks of sane input so this little thing _could_ blow up in our
 * face.
 *
 * The oled_write_raw_P-method expects a string of numbers where it will use the
 * bits and turn individual pixels on or off. Each number represent 8 vertical
 * pixels.
 *
 * So 255 would mean all pixles in a vertical line, 8 pixels tall, should be "on".
 *
 * Needless to say - this code could use some refactoring. Currently it shows my
 * _total lack of understanding_ of rust as a language. But I'm working on it, ok...?
 */
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage {:?} <path to image file>", args[0]);
        return;
    }
    let img = image::open(&args[1]).unwrap();
    let bytesize: u32 = 8;
    let mut counter: u32 = 0;
    let height_limit = img.height() - bytesize;
    let width_limit = img.width() - 1;
    for y in 0..(img.height() / bytesize) {
        for x in 0..img.width() {
            let y_offset: u32 = y * bytesize;
            let mut byte: u8 = 0;
            for i in 0..=7 {
                let realy = y_offset + i;
                let pixel_value = image::Pixel::to_luma(&img.get_pixel(x, realy))[0];
                if pixel_value > 127 {
                    byte |= i32::pow(2, i) as u8;
                }
            }
            print!("{}", byte);
            if !(y_offset == height_limit && x == width_limit) {
                print!(",");
            }
            counter += 1;
            if counter % img.width() == 0 {
                println!();
            }
        }
    }
}
