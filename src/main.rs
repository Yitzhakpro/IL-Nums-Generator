use clap::Parser;
use pad::PadStr;

use std::{
    thread,
    thread::JoinHandle,

    fs::OpenOptions, io::Write
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct GenArgs {
    #[clap(short, long,
        help = "separated prefixes by space (050 052 054)",
        multiple_values = true
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

    for handle in thread_handles {
        handle.join().unwrap();
    }


}
