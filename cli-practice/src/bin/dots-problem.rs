#[derive(Debug, structopt::StructOpt)]
struct DotsProblemArgs {
    #[structopt(long)]
    title: String,
    #[structopt(long)]
    description: String,
    #[structopt(long)]
    complexity: u8,
}

#[paw::main]
fn main(args: DotsProblemArgs) {
    println!(
        "Буду создавать новую задачу \"{}\" сложности {}",
        args.title, args.complexity
    );
    println!("Описание: {}", args.description);

    std::fs::write(
        "Problem.xml",
        format!("<title>{}</title>", args.title).as_bytes(),
    )
    .unwrap();
}
