use log::info;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    match log4rs::init_file("log4rs.yml", Default::default()) {
        Ok(_) => info!("log4rs loading success"),
        _ => unreachable!(),
    };
    let pool = threadpool::Builder::new()
        .thread_name("my-thread-pool".to_string())
        .num_threads(8)
        .build();
    for i in 0..10 {
        pool.execute(move || {
            info!("{:#?}", i);
        })
    }
    pool.join();

    Ok(())
}
