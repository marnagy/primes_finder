use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    let filename = String::from("primes.txt");

    if !Path::new(&filename).exists() {
        let mut buffer = File::create(&filename).unwrap();

        buffer.write_all(b"2\n3\n5\n7\n").unwrap();
        buffer.flush().unwrap();
    }

    let mut known_primes: Vec<i64> = Vec::new();

    print!("Loading numbers...");
    io::stdout().flush();
    let reader = File::open(&filename).unwrap();

    let mut buff_reader = io::BufReader::new(reader);

    let mut line = String::new();
    buff_reader.read_line(&mut line).unwrap();
    while !line.is_empty() {
        //println!("Line -> {0}", line);
        let num: i64 = line.trim().parse().unwrap();
        known_primes.push(num);
        line.clear();
        let _ = buff_reader.read_line(&mut line);
    }
    println!("\rLoaded.                ");

    let mut file = OpenOptions::new()
        .append(true)
        .open(&filename)
        .expect("Cannot open file");
    //let mut writer = io::BufWriter::new(file);
    let mut wanted_num = known_primes[known_primes.len() - 1];
    loop {
        wanted_num += 2;
        let root = (wanted_num as f64).sqrt().floor() as i64;
        let mut add = true;

        for number in &known_primes {
            if number > &root {
                break;
            }

            if wanted_num % number == 0 {
                add = false;
                break;
            }
        }

        if add {
            let text: String = wanted_num.to_string() + "\n";
            //writer.write_all(text.as_bytes()).unwrap();
            file.write_all(text.as_bytes()).unwrap();
            println!("Number: {0}", wanted_num);
            known_primes.push(wanted_num);
            //sleep(Duration::from_millis(50));
        }
    }
}
