use anyhow::Result;
use chrono::Local;
use fern::colors::{Color, ColoredLevelConfig};
use fern::Dispatch;
use log::{debug, error, info, trace, warn};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    setup_logger()?;
    trace!("Trace Message");
    debug!("Debug Message");
    info!("Info Message");
    warn!("Warn Message");
    error!("Error Message");

    Ok(())
}

fn setup_logger() -> Result<(), fern::InitError> {
    let colors = ColoredLevelConfig::new()
        // .error(Color::Red)
        // .warn(Color::BrightMagenta)
        // .info(Color::BrightGreen)
        // .debug(Color::BrightCyan)
        // .trace(Color::BrightBlue)
        ;

    Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{} [{}] {} - {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                colors.color(record.level()),
                record.target(),
                message
            ))
        })
        .chain(std::io::stdout())
        .chain(fern::log_file("logs/output.log")?)
        .apply()?;

    Ok(())
}
