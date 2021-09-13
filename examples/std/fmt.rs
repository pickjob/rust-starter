use anyhow::Result;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello");                   // => "Hello"
    println!("Hello, {}!", "world");     // => "Hello, world!"
    println!("The number is {}", 1);     // => "The number is 1"
    println!("{:?}", (3, 4));            // => "(3, 4)"
    println!("{value}", value=4);        // => "4"
    println!("{} {}", 1, 2);             // => "1 2"
    println!("{:04}", 42);               // => "0042" with leading zeros
    println!("{:#?}", (100, 200));       // => "(
                                         //       100,
                                         //       200,
                                         //     )"
    // Positional parameters
    println!("{1} {} {0} {}", 1, 2);      // => "2 1 1 2"
    // Named parameters
    println!("{argument}", argument = "test");   // => "test"
    println!("{name} {}", 1, name = 2);          // => "2 1"
    println!("{a} {c} {b}", a="a", b='b', c=3);  // => "a 3 b"
    // Formatting Parameters
    // Width
    // All of these print "Hello x    !"
    println!("Hello {:5}!", "x");
    println!("Hello {:1$}!", "x", 5);
    println!("Hello {1:0$}!", 5, "x");
    println!("Hello {:width$}!", "x", width = 5);
    // Fill/Alignment
    println!("Hello {:<5}!", "x");      // => "Hello x    !"
    println!("Hello {:-<5}!", "x");     // => "Hello x----!"
    println!("Hello {:^5}!", "x");      // => "Hello   x  !"
    println!("Hello {:>5}!", "x");      // => "Hello     x!"
    // Sign/#/0
    println!("Hello {:+}!", 5);         // => "Hello +5!"
    println!("{:#x}!", 27);             // => "0x1b!"
    println!("Hello {:05}!", 5);        // =>  "Hello 00005!"
    println!("Hello {:05}!", -5);       // => "Hello -0005!"
    println!("{:#010x}!", 27);          // => "0x0000001b!"
    // Precision
    // Hello {arg 0 ("x")} is {arg 1 (0.01) with precision specified inline (5)}
    println!("Hello {0} is {1:.5}", "x", 0.01);
    // Hello {arg 1 ("x")} is {arg 2 (0.01) with precision specified in arg 0 (5)}
    println!("Hello {1} is {2:.0$}", 5, "x", 0.01);
    // Hello {arg 0 ("x")} is {arg 2 (0.01) with precision specified in arg 1 (5)}
    println!("Hello {0} is {2:.1$}", "x", 5, 0.01);
    // Hello {next arg ("x")} is {second of next two args (0.01) with precision specified in first of next two args (5)}
    println!("Hello {} is {:.*}",    "x", 5, 0.01);
    // Hello {next arg ("x")} is {arg 2 (0.01) with precision specified in its predecessor (5)}
    println!("Hello {} is {2:.*}",   "x", 5, 0.01);
    // Hello {next arg ("x")} is {arg "number" (0.01) with precision specified in arg "prec" (5)}
    println!("Hello {} is {number:.prec$}", "x", prec = 5, number = 0.01);

    Ok(())
}