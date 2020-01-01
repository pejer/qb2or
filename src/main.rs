extern crate image;

use  std::env;
use image::GenericImageView;

/*
 * This will generate a string with which QMK and its oled_write_raw_P-method can use
 * to render a bitmap image on an oled.
 *
 * We assume that you'll create an image that has the correct dimensions and that is 
 * monochrome. Currently we only read the value of the first channel, red, and any
 * pixel with a value higher then 127 is considered to be "on".
 *
 * There are no checks of sane input so this little thing _could_ blow up in our 
 * face.
 *
 *
 * The oled_write_raw_P-method expects a string of numbers where it will use the
 * bits and turn individual pixels on or off. Each number represent 8 vertical
 * pixels.
 *
 * So 255 would mean all pixles in a vercial lign, should be "on". etc.
 *
 *
 */
fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        println!("Usage {:?} <path to image file>", args[0]);
        return;
    }
    let img = image::open(&args[1]).unwrap();

    let pixel_pos_to_bit: [u8; 8] = [
        0b0000_0001,
        0b0000_0010,
        0b0000_0100,
        0b0000_1000,
        0b0001_0000,
        0b0010_0000,
        0b0100_0000,
        0b1000_0000
    ];
    let mut counter: u32 = 0;
    for y in 0..(img.height()/8) {
        for x in 0..img.width() {
            let y_offset: u32 = y * 8;
            let mut byte: u8 = 0b0000_0000;
            for i in 0..=7 {
                let realy = y_offset + i;
                let pixel_value = img.get_pixel(x,realy)[0];
                if pixel_value > 127 {
                    byte |= pixel_pos_to_bit[i as usize];
                }
            }
            print!("{},", byte);
            counter += 1;
            if counter % img.width() == 0 {
                println!("");
            }
        }
    }
}
