use anyhow::Result;
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
