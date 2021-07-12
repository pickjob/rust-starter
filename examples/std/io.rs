use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{prelude::*, BufReader, BufWriter};
use std::time::SystemTime;

fn main() -> Result<(), Box<dyn Error>> {
    let file_name = "test.txt";
    {
        let mut writer = BufWriter::new(File::create(file_name)?);
        let greetings = "Hello World\n";
        let now = SystemTime::now();
        // write a byte to the buffer
        writer.write_all(greetings.as_bytes())?;
        writer.write_all(format!("{:#?}", now).as_bytes())?;
    } // the buffer is flushed once writer goes out of scope
    let reader = BufReader::new(File::open(file_name)?);
    for line in reader.lines() {
        println!("{}", line?);
    }
    fs::remove_file(file_name)?;

    Ok(())
}
