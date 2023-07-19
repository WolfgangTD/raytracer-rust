use std::ops::{Add, Sub};
use std::io::{stdout, Write};

#[derive(Debug, Copy, Clone)]
pub struct Vector3{
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
        fn _dot(&self, other: &Vector3) -> f64 {
            self.x + other.x + self.y + other.y + self.z + other.z
        }
        fn _cross(&self, other: &Vector3) -> Vector3 {
            Vector3 {
                x: self.y * other.z - self.z * other.y,
                y: self.z * other.x - self.x * other.z,
                z: self.x * other.y - self.y * other.x,
            }
        }
        fn normalize(&self) -> Vector3 {
            let magnitude: f64 = self.x*self.x + self.y*self.y + self.z*self.z;
            Vector3 { x: self.x/magnitude.sqrt(), y: self.y/magnitude.sqrt(), z: self.z/magnitude.sqrt() }
        }
        fn scalar(&self, other:f64) -> Vector3 {
            Vector3 { x: self.x * other, y: self.y * other, z: self.z * other }
        }
        fn write_colour(&self) {
            let r = (255.999 * self.x) as i32;
            let g = (255.999 * self.y) as i32;
            let b = (255.999 * self.z) as i32;
    
            println!("{r} {g} {b}");
        }
}

impl Add for Vector3 {
    type Output = Self;
    fn add(self, other:Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }
}
impl Sub for Vector3 {
    type Output = Self;
    fn sub(self, other:Self) -> Self {
        Self {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
}
    
struct Ray {
    _origin: Vector3,
    dir: Vector3,
}

impl Ray {
    fn _at(self ,t:f64) -> Vector3{
        self._origin + self.dir.scalar(t)
    }
    fn ray_colour(self) -> Vector3 {
        if sphere_collision(Vector3 { x: (-1.0), y: (0.0), z: (0.0) }, 0.5, &self){
            return Vector3 {x:1.0, y:0.0, z:0.0};
        }
        let unit_direction = (self.dir).normalize();
        let t = 0.5*unit_direction.y + 1.0;
        return (Vector3 { x: (1.0), y: (1.0), z: (1.0) }.scalar(1.0-t)) + (Vector3 { x: (0.5), y: (0.7), z: (1.0) }.scalar(t));
    }
}

fn sphere_collision(centre: Vector3, radius: f64, ray: &Ray ) -> bool {
    let oc = ray._origin - centre;
    let a = ray.dir._dot(&ray.dir);
    let b = 2.0 * oc._dot(&ray.dir);
    let c = oc._dot(&oc) - radius * radius;
    let discriminant = b*b - 4.0*a*c;
    return discriminant > 0.0;
}

fn main() {
    //image
    let aspect_ratio = 16.0/9.0;
    let image_width = 800.0;
    let image_height = image_width / aspect_ratio;

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
    let lower_left_corner = origin - horizontal.scalar(0.5) - vertical.scalar(0.5) - Vector3 { x: (0.0), y: (0.0), z: (focal_length) };

    println!("P3\n{image_width} {image_height}\n255\n");
    let mut pixel_color;
    let mut u;
    let mut v;
    let mut r;
    let mut j = image_height-1.0;

    while j as i32 >= 0 {
        eprintln!("\rScanlines remaining {j}");
        stdout().flush().unwrap();
        for i in 0..image_width as i32 {
            u = i as f64 / (image_width-1.0) as f64; 
            v = j as f64 /(image_height-1.0) as f64;
            r = Ray {
                _origin: origin,
                dir: lower_left_corner + horizontal.scalar(u) + vertical.scalar(v) - origin,
            };
            pixel_color = r.ray_colour();
            
            pixel_color.write_colour();     
        }
        j -= 1.0;
    }
    eprintln!("\nDone.\n");
}
