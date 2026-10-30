use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("../data/a/requests.log")?;
    let reader = BufReader::new(file);

    let mut line_count: u64 = 0;

    for line in reader.lines() {
        let line = line?;

        println!("{}", line);

        line_count += 1;

        if line_count == 5 {
            break;
        }
    }

    println!("Lines read: {}", line_count);

    Ok(())
}
