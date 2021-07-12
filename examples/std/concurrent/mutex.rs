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
    match log4rs::init_file("log4rs.yml", Default::default()) {
        Ok(_) => info!("log4rs loading success"),
        _ => unreachable!(),
    };
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
