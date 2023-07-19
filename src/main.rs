mod vector;
use std::io::{stdout, Write};
use vector::{WriteColour, Vector3, Scalar, Normalize};

struct Ray {
    origin: Vector3,
    dir: Vector3,
}

trait At {
    fn at(r:Ray, t:f64) -> Vector3;
}

trait RayColour {
    fn ray_colour(self) -> Vector3;
}

impl At for Ray {
    fn at(r:Ray ,t:f64) -> Vector3{
        r.origin + r.dir.scalar(t)
    }
}

impl RayColour for Ray {
    fn ray_colour(self) -> Vector3 {
        let unit_direction = self.dir.normalize();
        let t = 0.5*unit_direction.y + 1.0;
        let colour1 = Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0
        };
        let colour2 = Vector3 {
            x: 0.5,
            y: 0.7,
            z: 1.0
        };
        colour1.scalar(1.0-t) + colour2.scalar(t)
    }
}
fn main() {
    //image
    let aspect_ratio = 16.0/9.0;
    let image_width = 400;
    let image_height = image_width / aspect_ratio as i32;

    //camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0
    };
    let horizontal = Vector3 {
        x: viewport_width,
        y: 0.0,
        z: 0.0
    };
    let vertical = Vector3 {
        x: 0.0,
        y: viewport_height,
        z: 0.0
    };
    let lower_left_corner = origin - horizontal.scalar(1.0/2.0) - Vector3 { x: (0.0), y: (0.0), z: (focal_length) };

    println!("P3\n{image_width} {image_height}\n255\n");

    let mut j = image_height-1;

    while j >= 0 {
        eprintln!("\rScanlines remaining {j}");
        stdout().flush().unwrap();
        for i in 0..image_width {
            let u = (i / image_width-1) as f64; 
            let v = (j/image_height-1) as f64;
            let r = Ray {
                origin: origin,
                dir: lower_left_corner + horizontal.scalar(u) + vertical.scalar(v) - origin,
            };
            let pixel_color = r.ray_colour();
            
            pixel_color.write_colour();     
        }
        j-=1;
    }
    eprintln!("\nDone.\n");
}
