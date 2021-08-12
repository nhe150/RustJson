
use simdjson_rust::dom;
use std::fmt;

use flate2::read;
use flate2::write;
use flate2::Compression;
use std::error::Error;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use std::time::Instant;



/// Read normal or compressed files seamlessly
/// Uses the presence of a `.gz` extension to decide
pub fn reader(filename: &str) -> Box<dyn BufRead> {
    let path = Path::new(filename);
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why.description()),
        Ok(file) => file,
    };

    if path.extension() == Some(OsStr::new("gz")) {
        Box::new(BufReader::with_capacity(
            128 * 1024 *100 * 10,
            read::GzDecoder::new(file),
        ))
    } else {
        Box::new(BufReader::with_capacity(128 * 1024, file))
    }
}

/// Write normal or compressed files seamlessly
/// Uses the presence of a `.gz` extension to decide
pub fn writer(filename: &str) -> Box<dyn Write> {
    let path = Path::new(filename);
    let file = match File::create(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why.description()),
        Ok(file) => file,
    };

    if path.extension() == Some(OsStr::new("gz")) {
        // Error is here: Created file isn't gzip-compressed
        Box::new(BufWriter::with_capacity(
            128 * 1024,
            write::GzEncoder::new(file, Compression::default()),
        ))
    } else {
        Box::new(BufWriter::with_capacity(128 * 1024, file))
    }
}



fn main() -> Result<(), Box<dyn std::error::Error>> {


    let start = Instant::now();

    let mut parser = dom::Parser::default();

    // let filename = "oct1.json";
    // println!("writer with regular file: '{}'", filename);
    // let mut writer = writer(filename);
    // writer.write_all(b"[")?;
    //
    //
    // let filename = "mq_oct_1.csv.gz";
    // let mut count = 0;
    // println!("reader with compressed file: '{}'", filename);
    // let reader_file_gz = reader(filename);
    // for line in reader_file_gz.lines() {
    //     //println!("{}", line?);
    //     for tab  in line.unwrap().split('\t') {
    //         //let _tp1 = parser.parse(&tab);
    //         writer.write_all(tab.as_bytes());
    //         writer.write_all(b",\n");
    //         //println!("Display: {}", tp1.unwrap());
    //         break;
    //     }
    //
    //     count +=1 ;
    //     // if count >2  {
    //     //     break;
    //     // }
    //     if count % 10000  == 0 {
    //         println!("thousands {:10?}", count);
    //     }
    // }
    //
    // writer.write_all(b"{}]");
    // println!("count {:10?}", count);





    let tweets = parser.load("oct1.json")?;
    //let tweets = parser.parse(&buffer)?;
    let elapsed = start.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    println!("What a relieve!!!");


    //
    // //print!("Display: {}", tweets);
    //
    // let mut index = 0;
    // for tweet in  tweets.at_index(3).iter() {
    //     print!("Display: {} is {}",  index, tweet);
    //     index +=1;
    // }

    Ok(())
}



