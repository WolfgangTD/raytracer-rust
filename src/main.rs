mod vector;

use vector::{Vector3, Dot, Cross};
 
fn main() {
    let point1 = Vector3{
        x: 1.0,
        y: 2.0,
        z: 3.0
    };
    let point2 = Vector3{
        x: 4.0,
        y: 5.0,
        z: 6.0
    };
    let blahblah = point1.dot(&point2);
    let result = point1.cross(&point2);
    println!("{:?}", blahblah);
    println!("{:?}", result);
}
