pub fn handle_help() {
    const BANNER: &str = r#"
    _______________.___._____________________ ___________   ____
    \______   \__  |   |\______   \_   _____/ \      \   \ /   /
     |     ___//   |   | |       _/|    __)_  /   |   \   Y   / 
     |    |    \____   | |    |   \|        \/    |    \     /  
     |____|    / ______| |____|_  /_______  /\____|__  /\___/   
               \/               \/        \/         \/         
                "#;
    eprintln!("{}", BANNER);
    eprintln!("pyrenv is a tool to manage multiple python versions on your system.");
    eprintln!("usage: pyrenv <command> [<args>]");

    eprintln!("\ncommands:");
    eprintln!("  help                   show this help message");
    eprintln!("  install <version>      install a python version");
    eprintln!("  list                   list installed python versions");
    eprintln!("  candidates             get a list of possible python versions to install");
    eprintln!("  use <version>          use a python version");
    eprintln!("  version                show the current version of pyrenv");
    eprintln!("\n");
    eprintln!("pyrenv is written by Mongosaurusrex <github.com/mongosaurusrex>");
    eprintln!("pyrenv is licensed under the MIT license");
    eprintln!(
        "pyrenv is open source, and you can contribute to it at <github.com/mongosaurusrex/pyrenv>"
    );
}
