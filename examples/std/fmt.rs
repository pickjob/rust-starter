use anyhow::{Result, Context};
use flexi_logger::{
    colored_detailed_format, Age, Cleanup, Criterion, Duplicate, FileSpec, LevelFilter,
    LogSpecification, Logger, Naming,
};
use log::info;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    setup_logger().context(format!("Create logger failed"))?;
    info!("Hello");                   // => "Hello"
    info!("Hello, {}!", "world");     // => "Hello, world!"
    info!("The number is {}", 1);     // => "The number is 1"
    info!("{:?}", (3, 4));            // => "(3, 4)"
    info!("{value}", value=4);        // => "4"
    info!("{} {}", 1, 2);             // => "1 2"
    info!("{:04}", 42);               // => "0042" with leading zeros
    info!("{:#?}", (100, 200));       // => "(
                                         //       100,
                                         //       200,
                                         //     )"
    // Positional parameters
    info!("{1} {} {0} {}", 1, 2);      // => "2 1 1 2"
    // Named parameters
    info!("{argument}", argument = "test");   // => "test"
    info!("{name} {}", 1, name = 2);          // => "2 1"
    info!("{a} {c} {b}", a="a", b='b', c=3);  // => "a 3 b"
    // Formatting Parameters
    // Width
    // All of these print "Hello x    !"
    info!("Hello {:5}!", "x");
    info!("Hello {:1$}!", "x", 5);
    info!("Hello {1:0$}!", 5, "x");
    info!("Hello {:width$}!", "x", width = 5);
    // Fill/Alignment
    info!("Hello {:<5}!", "x");      // => "Hello x    !"
    info!("Hello {:-<5}!", "x");     // => "Hello x----!"
    info!("Hello {:^5}!", "x");      // => "Hello   x  !"
    info!("Hello {:>5}!", "x");      // => "Hello     x!"
    // Sign/#/0
    info!("Hello {:+}!", 5);         // => "Hello +5!"
    info!("{:#x}!", 27);             // => "0x1b!"
    info!("Hello {:05}!", 5);        // =>  "Hello 00005!"
    info!("Hello {:05}!", -5);       // => "Hello -0005!"
    info!("{:#010x}!", 27);          // => "0x0000001b!"
    // Precision
    // Hello {arg 0 ("x")} is {arg 1 (0.01) with precision specified inline (5)}
    info!("Hello {0} is {1:.5}", "x", 0.01);
    // Hello {arg 1 ("x")} is {arg 2 (0.01) with precision specified in arg 0 (5)}
    info!("Hello {1} is {2:.0$}", 5, "x", 0.01);
    // Hello {arg 0 ("x")} is {arg 2 (0.01) with precision specified in arg 1 (5)}
    info!("Hello {0} is {2:.1$}", "x", 5, 0.01);
    // Hello {next arg ("x")} is {second of next two args (0.01) with precision specified in first of next two args (5)}
    info!("Hello {} is {:.*}",    "x", 5, 0.01);
    // Hello {next arg ("x")} is {arg 2 (0.01) with precision specified in its predecessor (5)}
    info!("Hello {} is {2:.*}",   "x", 5, 0.01);
    // Hello {next arg ("x")} is {arg "number" (0.01) with precision specified in arg "prec" (5)}
    info!("Hello {} is {number:.prec$}", "x", prec = 5, number = 0.01);

    Ok(())
}

fn setup_logger() -> Result<()> {
    let mut builder = LogSpecification::builder();
    builder.default(LevelFilter::Trace);
    let logger = Logger::with(builder.build());
    logger
        .format(colored_detailed_format)
        .duplicate_to_stdout(Duplicate::Info)
        .log_to_file(
            FileSpec::default()
                .directory("logs")
                .basename("app")
                .suffix("log")
                .suppress_timestamp()
        )
        .append()
        .rotate(
            Criterion::Age(Age::Day),
            Naming::Timestamps,
            Cleanup::KeepCompressedFiles(7),
        )
        .print_message()
        .start()?;

    Ok(())
}
