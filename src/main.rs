mod zalgo;

use clap::Parser;
use crate::zalgo::MarkConfig;

#[derive(Parser)]
#[command(about, version, long_about = None)]
struct AppArgs {
    text: String,

    /// mark num
    #[arg(short,long, default_value_t = 6)]
    num: usize,

    /// use extend marks
    #[arg(long)]
    extend: bool,

    /// use supplement marks
    #[arg(long)]
    supplement: bool,

    /// use symbol marks
    #[arg(long)]
    symbol: bool,

    /// use half marks
    #[arg(long)]
    half: bool,
}

fn main() {
    let args = AppArgs::parse();

    if !args.text.is_empty() {
        let mut mark_config = MarkConfig::new();
        mark_config
            .set_mark_num(args.num)
            .set_extend(args.extend)
            .set_supplement(args.supplement)
            .set_symbol(args.symbol)
            .set_half(args.half);
        println!("{}", zalgo::zalgo_text(args.text, &mark_config));
    } else {
        println!("Should input a non-empty string!");
    }
}
