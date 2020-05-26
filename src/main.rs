extern "C" {
    fn sqrt(x: f64) -> f64;
}

#[link(name = "m")]
fn main() {

    let x: f64 = 2.;
    let result: f64 = unsafe { sqrt(x) };

    println!("The square root of {} is {}", x, result);
}

// use libm::sqrt;
//
// fn main() {
//
//     let x: f64 = 2.;
//     let result: f64 = sqrt(x);
//
//     println!("The square root of {} is {}", x, result);
// }
//

// fn main() {
//
//     let x: f64 = 2.;
//     let result: f64 = x.sqrt();
//
//     println!("The square root of {} is {}", x, result);
// }
//
