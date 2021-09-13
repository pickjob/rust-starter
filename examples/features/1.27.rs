use anyhow::Result;
use std::error::Error;

/**
 *  接口实现接口:  impl TraitA for dyn TraitB
 *      由于Trait Object Unsized, 必须在堆中实现转换, 调用
 */
fn main() {
    // let c = Circle { radius: 2f64 };
    // info!("{:#?}", c.area());

    let b = Box::new(Circle { radius: 4f64 }) as Box<dyn Round>;
    println!(b.area());

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

impl Shape for dyn Round {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.get_radius() * self.get_radius()
    }
}
