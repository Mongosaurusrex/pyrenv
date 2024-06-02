mod handlers;

fn main() {
    let command = std::env::args().nth(1).expect("no pattern given");
    match command.as_str() {
        "help" | "-h" => {
            handlers::help::handle_help();
        }
        "install" => {
            handlers::install::handle_install();
        }
        "list" => {
            handlers::list::handle_list();
        }
        "candidates" => {
            handlers::candidates::handle_candidates();
        }
        "version" | "-v" => {
            handlers::version::handle_version();
        }
        _ => {
            eprintln!("unknown command: {}", command);
            eprintln!("try 'pyrenv help' for more information");
            std::process::exit(1);
        }
    }

    std::process::exit(0);
}
