use clap::Parser;
use pad::PadStr;

use std::{
    thread,
    thread::JoinHandle,

    fs,
    fs::OpenOptions,
    io::{ Write, Read },

    time::Instant
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

fn get_correct_prefix(prefix: &String) -> String {
    if !prefix.starts_with("0") {
        String::from("0") + prefix
    } else {
        String::from(prefix)
    }
}

fn main() {
    let gen_arg = GenArgs::parse();
    let slient_mode = gen_arg.silent;
    let prefixes = gen_arg.prefixes;

    let mut thread_handles: Vec<JoinHandle<()>> = Vec::new();

    let start_time = Instant::now();

    for pref in prefixes.iter()  {
        let corrected_prefix = get_correct_prefix(&pref);

        if !slient_mode {
            println!("Generating nums for: {}", corrected_prefix);
        }

        let handle = thread::spawn(move || {
            let prefix_file_name = String::from(".") + &corrected_prefix;

            let mut prefix_file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(&prefix_file_name)
                .expect("Could not create temp prefix file, make sure this program have enough permissions.");

            for i in 0i32..9999999 {
                let num_to_string = i.to_string();
                let padded_num = num_to_string.pad(7, '0', pad::Alignment::Right, true);

                let full_il_num = String::from(&corrected_prefix) + &padded_num + "\n";

                prefix_file.write_all(full_il_num.as_bytes())
                    .expect("Could not write number to temp prefix file.");
            }
        });

        thread_handles.push(handle);
    }

    // joining threads
    for handle in thread_handles {
        handle.join().unwrap();
    }

    let mut final_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("final_il_nums.txt")
        .expect("Could not create final il nums, make sure this program have enough permissions.");

    // combine temp files & delete temp files
    for pref in prefixes.iter() {
        let file_name = get_correct_prefix(&pref);

        let mut file_data = String::new();

        let mut prefix_file = OpenOptions::new()
            .read(true)
            .open(&file_name)
            .expect("Could not open temp prefix file.");

        prefix_file.read_to_string(&mut file_data)
            .expect("Could not read from temp prefix file.");

        final_file.write_all(file_data.as_bytes())
            .expect("Could not write to final il nums file.");

        // delete temp file after writing to final file, not important if could not delete
        fs::remove_file(file_name)
            .unwrap();
    }

    let duration = start_time.elapsed();

    println!("Finished generating, enjoy!");
    println!("Time took to finish: {:#?}", duration);
}
