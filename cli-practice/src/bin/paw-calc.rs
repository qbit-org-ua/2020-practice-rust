#[derive(Debug, structopt::StructOpt)]
struct MyArgs {
    /// Первое число
    #[structopt(short, long)]
    first_number: i64,
    /// Второе число
    #[structopt(short, long)]
    second_number: i64,
    /// Третье число
    #[structopt(short, long, default_value = "3")]
    third_number: i64,
}

#[paw::main]
fn main(args: MyArgs) {
    println!(
        "{} + {} + {} = {}",
        args.first_number,
        args.second_number,
        args.third_number,
        args.first_number + args.second_number + args.third_number
    );
}
