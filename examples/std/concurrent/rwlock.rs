use anyhow::{Context, Result};
use flexi_logger::{
    colored_detailed_format, Age, Cleanup, Criterion, Duplicate, FileSpec, LevelFilter,
    LogSpecification, Logger, Naming,
};
use log::info;
use std::sync::{Arc, RwLock};
use std::thread;
use std::{error::Error, time::Duration};

/**
 *   RwLock: 读写锁实现线程间并发控制
 *      1. 遇到异常 Mutex 会 Posioned, 检测后调用 into_inner 相信数据正常
 *      2. 手动调用 ste::memory::drop() 方法释放 RwLock
 *      3. 都共享，写独占
 */
fn main() -> Result<(), Box<dyn Error>> {
    setup_logger().context(format!("Create logger failed"))?;
    let lock = Arc::new(RwLock::new(0));
    for idx in 0..5 {
        let cloned_lock = lock.clone();
        thread::spawn(move || {
            let mut _guard = match cloned_lock.write() {
                Ok(guard) => {
                    println!("Not posioned value: {:#?}", guard);
                    guard
                }
                Err(poisoned) => {
                    // Posioned Mutex
                    println!("Poisoned value: {:#?}", poisoned);
                    // 相信数据正常继续执行
                    poisoned.into_inner()
                }
            };
            println!("guard value: {:#?}", _guard);
            if idx == 1 {
                panic!("throw by cause");
            } else {
                *_guard += 1;
            }
            drop(_guard);
        });
    }
    thread::sleep(Duration::new(5, 0));
    println!("in the end ...");

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
                .suppress_timestamp(),
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
