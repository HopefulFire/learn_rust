pub struct Config
{
    pub query:String,
    pub filename:String,
}

impl Config
{
    pub fn new(args:&Vec<String>) -> Result<Config, &'static str>
    {
        if args.len() < 3
        {
            return Err("Not enough arguments!");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        return Ok(Config{query, filename});
    }
}

pub fn run(config:Config) -> Result<(), Box<dyn std::error::Error>>
{
    let contents = std::fs::read_to_string(config.filename)?;

    println!("{}", contents);
    return Ok(());
}