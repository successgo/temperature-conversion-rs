fn main() {
    println!("c2f:");

    for x in 1..10 {
        println!("{x:2},{:6.2}", c2f(x as f32))
    }

    println!();

    println!("f2c:");

    for x in 50..60 {
        println!("{x:2},{:6.2}", f2c(x as f32))
    }
}

fn f2c(f: f32) -> f32 {
    (f - 32.0) * 5.0 / 9.0
}

fn c2f(c: f32) -> f32 {
    (c * 9.0 / 5.0) + 32.0
}