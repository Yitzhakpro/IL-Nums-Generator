use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct GenArgs {
    #[clap(short, long,
        help = "separated prefixes by comma (050,052,054)"
    )]
    prefixes: String,

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

    println!("{:#?}", gen_arg.prefixes);
}
