mod vector;

use vector::{Colour, WriteColour};

fn main() {
    let image_width:i32 = 256;
    let image_height:i32 = 256;

    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    let mut j = image_height-1;

    while j >= 0 {
        for i in 0..image_width {

            let pixel_color = Colour {
                r: i as f32/(image_width-1) as f32,
                g: j as f32/(image_height-1) as f32,
                b: 0.25,
            };
            pixel_color.write_colour();
        }
        j-=1;
    }

    
}
