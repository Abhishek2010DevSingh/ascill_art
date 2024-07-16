use clap::Parser;
use image::{DynamicImage, GenericImageView};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct ImageValue {
    /// Path of image
    #[arg(short, long)]
    path: String,

    /// Scale for Ascii art
    #[arg(short, long, default_value_t = 3)]
    scale: u32,
}

fn get_str_ascii(intent: u8) -> &'static str {
    let index: u8 = intent / 32;
    let ascii: [&str; 8] = [" ", ".", ",", "-", "~", "+", "=", "@"];
    return ascii[index as usize];
}

fn get_image(dir: &str, scale: u32) {
    let img: DynamicImage = image::open(dir).unwrap();
    let (width, height) = img.dimensions();

    for y in 0..height {
        for x in 0..width {
            if y % (scale * 2) == 0 && x % scale == 0 {
                let pix = img.get_pixel(x, y);
                let mut intent: u8 = pix[0] / 3 + pix[1] / 3 + pix[2] / 3;
                if pix[3] == 0 {
                    intent = 0;
                }
                print!("{}", get_str_ascii(intent));
            }
        }
        if y % (scale * 2) == 0 {
            println!("");
        }
    }
}

fn main() {
    let value = ImageValue::parse();
    get_image(&value.path, value.scale)
}
