extern crate clap;
extern crate mongodb;
extern crate tokio;

use clap::{App, Arg};
use mongodb::{Client};
use std::time::{Instant};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("mongodb-perf")
        .about("measure MongoDB read and write performance")
        .arg(
            Arg::with_name("mongodb_uri")
                .short("u")
                .long("uri")
                .value_name("URI")
                .help("MongoDB connection URI")
                .required(true),
        )
        .arg(
            Arg::with_name("read_iterations")
                .short("r")
                .long("read-iterations")
                .value_name("READ_ITERATIONS")
                .help("Number of read iterations")
                .default_value("1000"),
        )
        .arg(
            Arg::with_name("write_iterations")
                .short("w")
                .long("write-iterations")
                .value_name("WRITE_ITERATIONS")
                .help("Number of write iterations")
                .default_value("1000"),
        )
        .get_matches();

    let mongodb_uri = matches.value_of("mongodb_uri").unwrap();
    let read_iterations = matches
        .value_of("read_iterations")
        .unwrap()
        .parse::<u64>()?;
    let write_iterations = matches
        .value_of("write_iterations")
        .unwrap()
        .parse::<u64>()?;

    let client = Client::with_uri_str(mongodb_uri).await?;
    let db = client.database("test");
    let collection = db.collection::<mongodb::bson::Document>("test");

    // Warm-up phase
    for _ in 0..write_iterations {
        let document = mongodb::bson::doc! { "key": "value" };
        collection.insert_one(document, None).await?;
    }

    let start_time = Instant::now();

    // Read phase
    for _ in 0..read_iterations {
        let filter = mongodb::bson::doc! { "key": "value" };
        collection.find(filter, None).await?;
    }

    let read_duration = start_time.elapsed();
    let start_time = Instant::now();

    // Write phase
    for _ in 0..write_iterations {
        let document = mongodb::bson::doc! { "key": "value" };
        collection.insert_one(document, None).await?;
    }

    let write_duration = start_time.elapsed();

    println!(
        "Read Performance: {} reads in {:?}",
        read_iterations,
        read_duration
    );
    
    println!(
        "Write Performance: {} writes in {:?}",
        write_iterations,
        write_duration
    );

    Ok(())
}
