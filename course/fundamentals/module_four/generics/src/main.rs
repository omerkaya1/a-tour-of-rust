#![allow(dead_code, unused)]

// use std::fmt::Debug;

// // the below notation is similar to:
// // fn print_stuff<>(x: T)
// // where T: ToString,
// // {
// // ...
// fn print_stuff<T: ToString + Debug>(x: T) {
//     println!("{}", x.to_string())
// }

struct Degrees(f32);
struct Radians(f32);

impl From<Radians> for Degrees {
    fn from(rad: Radians) -> Self {
        Degrees(rad.0 * 180.0 / std::f32::consts::PI)
    }
}

impl From<Degrees> for Radians {
    fn from(deg: Degrees) -> Self {
        Radians(deg.0 * std::f32::consts::PI / 180.0)
    }
}

fn sin(angle: impl Into<Radians>) -> f32 {
    let angle: Radians = angle.into();
    angle.0.sin()
}

fn main() {
    let behind_you = Degrees(180.0);
    let behind_you_radians = Radians::from(behind_you);
    let behind_you_radians2: Radians = Degrees(180.0).into();

    println!("{}", sin(behind_you_radians2));
}
