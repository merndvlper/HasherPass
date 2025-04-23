use clap::{Parser, Subcommand};
mod crypto;
mod storage;
// mod web;

#[derive(Parser)]
#[command(name = "HasherPass")]
#[command(about = "Easy and securely a password mamanger.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Add { name: String },
    Get { name: String },
    List,
    Delete { name: String },
    //Server,
}

#[tokio::main]
async fn main() {


    let cli = Cli::parse();
    let key = crypto::get_master_key();

    match cli.command {
        Some(Commands::Add { name }) => storage::add_entry(&name, &key),
        Some(Commands::Get { name }) => storage::get_entry(&name, &key),
        Some(Commands::List) => storage::list_entries(),
        Some(Commands::Delete { name }) => storage::delete_entry(&name),
        //Some(Commands::Server) => web::start_web_server().await,
        None => println!("Invalid Command"),
    }
}
