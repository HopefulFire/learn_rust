
fn main()
{
    let args: Vec<String> = std::env::args().collect();
    
    let config = learn_rust::Config::new(&args).unwrap_or_else(|err|
        {
            println!("Problem parsing arguments: {}", err);
            std::process::exit(1);
        });

    println!("Searching for {} in {}", config.query, config.filename);
    
    learn_rust::run(config).unwrap_or_else(|err|
        {
            println!("Application error: {}", err);
            std::process::exit(1);
        });
}



