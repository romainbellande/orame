use clap::Parser;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use tokio::spawn;
mod fixtures;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    gen_key: bool,

    #[arg(long)]
    gen_fixtures: bool,

    #[arg(short, long)]
    database_url: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if args.gen_key {
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(64)
            .map(char::from)
            .collect();
        println!("{}", rand_string);
    }

    if args.gen_fixtures {
        if args.database_url.is_empty() {
            panic!("database_url must be set");
        }

        fixtures::exec(args.database_url).await.unwrap();
    }
}
