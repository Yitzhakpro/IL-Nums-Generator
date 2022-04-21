use clap::Parser;
use pad::PadStr;

use std::{
    thread,
    thread::JoinHandle,

    fs,
    fs::OpenOptions,
    io::{ Write, Read },

    time::{ Instant }
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct GenArgs {
    #[clap(short, long,
        help = "separated prefixes by space (050 052 054)",
        multiple_values = true,
        required = true
    )]
    prefixes: Vec<String>,

    #[clap(short, long,
        help = "whether to print progress or not",
    )]
    silent: bool,

    #[clap(short, long,
        help = "output file name",
        default_value = "il_nums.txt"
    )]
    output_file: String
}

fn main() {
    let gen_arg = GenArgs::parse();
    let slient_mode = gen_arg.silent;

    let mut thread_handles: Vec<JoinHandle<()>> = Vec::new();

    let start_time = Instant::now();

    for pref in gen_arg.prefixes.iter()  {
        let corrected_prefix = if !pref.starts_with("0") {
            String::from("0") + pref
        } else {
            String::from(pref)
        };

        if !slient_mode {
            println!("Generating nums for: {}", corrected_prefix);
        }

        let handle = thread::spawn(move || {
            let prefix_file_name = String::from(".") + &corrected_prefix;

            let mut prefix_file = match OpenOptions::new()
                .append(true)
                .create(true)
                .open(&prefix_file_name) {
                    Err(e) => panic!("Could not create {}: {}", prefix_file_name, e),
                    Ok(file) => file
                };

            for i in 0i32..9999999 {
                let num_to_string = i.to_string();
                let padded_num = num_to_string.pad(7, '0', pad::Alignment::Right, true);

                let full_il_num = String::from(&corrected_prefix) + &padded_num + "\n";

                match prefix_file.write_all(full_il_num.as_bytes()) {
                    Err(e) => panic!("Could not write num to: {}: {}", prefix_file_name, e),
                    Ok(()) => ()
                };
            }
        });

        thread_handles.push(handle);
    }

    // joining threads
    for handle in thread_handles {
        handle.join().unwrap();
    }

    let mut final_file = match OpenOptions::new()
        .write(true)
        .create(true)
        .open("final_il_nums.txt") {
            Err(e) => panic!("Could not create final il nums file, {}", e),
            Ok(file) => file
        };

    // combine temp files & delete temp files
    for pref in gen_arg.prefixes.iter() {
        let file_name = if !pref.starts_with("0") {
            ".".to_string() + "0" + &pref
        } else {
            ".".to_string() + &pref
        };

        let mut file_data = String::new();

        let mut prefix_file = match OpenOptions::new()
            .read(true)
            .open(&file_name) {
                Err(e) => panic!("Could not open prefix file: {}", e),
                Ok(file) => file
            };

        match prefix_file.read_to_string(&mut file_data) {
            Err(e) => panic!("Could not read from prefix file: {}", e),
            Ok(usize) => ()
        };

        match final_file.write_all(file_data.as_bytes()) {
            Err(e) => panic!("Could not write file data to final il nums: {}", e),
            Ok(()) => ()
        };

        // delete temp file after writing to final file
        match fs::remove_file(file_name) {
            Err(e) => panic!("Could not delete temp prefix file: {}", e),
            Ok(()) => ()
        };
    }

    let duration = start_time.elapsed();

    println!("Finished generating, enjoy!");
    println!("Time took to finish: {:#?}", duration);
}
