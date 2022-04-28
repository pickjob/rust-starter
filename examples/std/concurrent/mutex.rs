use anyhow::{Context, Result};
use flexi_logger::{
    colored_detailed_format, Age, Cleanup, Criterion, Duplicate, FileSpec, LevelFilter,
    LogSpecification, Logger, Naming,
};
use log::info;
use std::sync::{Arc, Mutex};
use std::thread;
use std::{error::Error, time::Duration};

/**
 *  Mutex: 互斥锁实现线程间并发控制
 *      1. 遇到异常 Mutex 会 Posioned, 检测后调用 into_inner 相信数据正常
 *      2. 手动调用 ste::memory::drop() 方法释放 Mutex
*/
fn main() -> Result<(), Box<dyn Error>> {
    setup_logger().context(format!("Create logger failed"))?;
    let lock = Arc::new(Mutex::new(0));
    for idx in 0..5 {
        let cloned_lock = lock.clone();
        thread::spawn(move || {
            let mut _guard = match cloned_lock.lock() {
                Ok(guard) => {
                    info!("Not posioned value: {:#?}", guard);
                    guard
                }
                Err(poisoned) => {
                    // Posioned Mutex
                    info!("Poisoned value: {:#?}", poisoned);
                    // 相信数据正常继续执行
                    poisoned.into_inner()
                }
            };
            info!("guard value: {:#?}", _guard);
            if idx == 1 {
                panic!("throw by cause");
            } else {
                *_guard += 1;
            }
            drop(_guard);
        });
    }
    thread::sleep(Duration::new(5, 0));
    info!("in the end ...");

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
