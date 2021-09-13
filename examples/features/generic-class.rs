use anyhow::Result;
use std::error::Error;

/**
 *  泛型接口实现  impl<T: TraitA> TraitB for T
 *      为每个TraitA具体实现类添加 TraitB 申明方法
 */
fn main() -> Result<(), Box<dyn Error>> {
    let c = Circle { radius: 2f64 };
    println!("{:#?}", c.area());

    // let b = Box::new(Circle {radius: 4f64}) as Box<dyn Round>;
    // println!(b.area());

    Ok(())
}

trait Shape {
    fn area(&self) -> f64;
}

trait Round {
    fn get_radius(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Round for Circle {
    fn get_radius(&self) -> f64 {
        self.radius
    }
}

impl<T: Round> Shape for T {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.get_radius() * self.get_radius()
    }
}
