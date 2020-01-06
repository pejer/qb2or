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

extern crate image;
use image::GenericImageView;
use std::fmt;

// the struct for QBBitmap.
pub struct QBBitmap {
    bitmap: image::DynamicImage,
    bytesize: u32,
    height: u32,
    width: u32,
    height_limit: u32,
    width_limit: u32,
}

fn recalculate_limits(bytesize: u32, width: u32, height: u32) -> [u32; 2] {
    [width - 1, height - bytesize]
}

impl QBBitmap {
    // setters
    // we allow dead code since these setters aren't necessarily used all the time.

    #[allow(dead_code)]
    pub fn set_bytesize(&mut self, bs: u32) {
        self.bytesize = bs;
        self.recalculate_limits();
    }

    #[allow(dead_code)]
    pub fn set_height(&mut self, hl: u32) {
        self.height = hl;
        self.recalculate_limits();
    }

    #[allow(dead_code)]
    pub fn set_width(&mut self, wl: u32) {
        self.width = wl;
        self.recalculate_limits();
    }

    fn recalculate_limits(&mut self) {
        let new_limits = recalculate_limits(self.bytesize, self.width, self.height);
        self.width_limit = new_limits[0];
        self.height_limit = new_limits[1];
    }

    // Parses & prints the strings of bytes QMK wants.
    pub fn parse(&self) {
        let mut counter: u16 = 0;
        for y in 0..(self.height / self.bytesize) {
            let y = y as u32;
            for x in 0..self.width {
                let x = x as u32;
                let y_offset: u32 = y * self.bytesize;
                let mut byte: u8 = 0;
                for i in 0..self.bytesize {
                    let realy = y_offset + i;
                    let pixel_value = image::Pixel::to_luma(&self.bitmap.get_pixel(x, realy))[0];
                    if pixel_value > 127 {
                        byte |= i32::pow(2, i) as u8;
                    }
                }
                print!("{}", byte);
                if !(y_offset == self.height_limit && x == self.width_limit) {
                    print!(",");
                }
                counter += 1;
                if counter % self.width as u16 == 0 {
                    println!();
                    counter = 0;
                }
            }
        }
    }
}
impl fmt::Debug for QBBitmap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Image  {{ dimensions: {}x{}, limit: {}x{}, bytesize: {} }}",
            self.width, self.height, self.width_limit, self.height_limit, self.bytesize
        )
    }
}

pub fn new(bitmap_file: &str) -> QBBitmap {
    let bitmap_file = image::open(bitmap_file).unwrap();
    let bytesize = 8;
    let height = bitmap_file.height();
    let width = bitmap_file.width();
    let limits = recalculate_limits(bytesize, width, height);
    QBBitmap {
        bitmap: bitmap_file,
        bytesize,
        height,
        width,
        height_limit: limits[1],
        width_limit: limits[0],
    }
}
