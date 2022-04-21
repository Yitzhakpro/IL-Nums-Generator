use clap::Parser;
use pad::PadStr;

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

    for pref in gen_arg.prefixes.iter()  {
        let corrected_prefix = if !pref.starts_with("0") {
            String::from("0") + pref
        } else {
            String::from(pref)
        };

        if !slient_mode {
            println!("Generating nums for: {}", corrected_prefix);
        }

        for i in 0i32..9999999 {
            let num_to_string = i.to_string();
            let padded_num = num_to_string.pad_to_width(7);

            let full_il_num = String::from(&corrected_prefix) + &padded_num;

            println!("num: {}", full_il_num);
        }
    }
}
