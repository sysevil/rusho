use clap::Parser;
use serde::Deserialize;
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use std::env;
use tokio::time::{sleep, Duration};
mod api;
use api::API;

#[derive(Parser)]
struct Cli {
    #[arg(short = 'd', long)]
    domain: String,
    
    #[arg(short = 'c', long)]
    config: Option<String>,
    
    #[arg(short = 'o', long)]
    output: Option<String>,

    #[arg(short = 'x', long)]
    key_file: Option<String>,
}

#[derive(Deserialize)]
struct Config {
    key: String,
}

fn load_config(path: Option<String>) -> Result<Config, Box<dyn Error>> {
    let config_path = match path {
        Some(p) => PathBuf::from(p),
        None => PathBuf::from("config.yaml"),
    };

    if !config_path.exists() {
        let home_dir = env::var("HOME")?;
        let default_path = PathBuf::from(format!("{}/.config/rusho/config.yaml", home_dir));

        if !default_path.exists() {
            fs::create_dir_all(default_path.parent().unwrap())?;
            let mut file = File::create(&default_path)?;
            writeln!(file, "key: your_default_shodan_key")?;
            println!("Created config file at {:?}", default_path);
        }

        Ok(serde_yaml::from_reader(File::open(default_path)?)?)
    } else {
        Ok(serde_yaml::from_reader(File::open(config_path)?)?)
    }
}

fn load_keys_from_file(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let keys = reader
        .lines()
        .filter_map(Result::ok)
        .collect::<Vec<String>>();

    if keys.is_empty() {
        return Err("The key file is empty or not valid".into());
    }
    Ok(keys)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let keys = if let Some(key_file_path) = args.key_file {
        load_keys_from_file(&key_file_path)?
    } else {
        vec![load_config(args.config)?.key]
    };

    let mut key_index = 0;
    let mut api = API::new(keys[key_index].clone());

    loop {
        match api.info_account(true).await {
            Ok(account_info) => {
                println!(
                    "API Plan: {}, Query Credits Available: {}",
                    account_info.plan, account_info.query_credits
                );
                if account_info.query_credits > 0 {
                    break; 
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }

        key_index = (key_index + 1) % keys.len();
        api.set_api_key(keys[key_index].clone());

        sleep(Duration::from_millis(200)).await;
        
        if key_index == 0 {
            println!("All keys have been exhausted without credits.");
            return Ok(());
        }
    }

    let subdomain_data = api.get_subdomain(&args.domain, true).await?;
    let subdomains = match subdomain_data.subdomains {
        Some(data) => data,
        None => {
            println!("No subdomains found for {}", args.domain);
            return Ok(());
        }
    };

    println!("Subdomains found for {}:", args.domain);
    for subdomain in &subdomains {
        println!("{}.{}", subdomain, args.domain);
    }

    let output_path = match args.output {
        Some(path) => path,
        None => format!("{}.rusho", args.domain),
    };

    let mut file = File::create(output_path)?;
    for subdomain in subdomains {
        writeln!(file, "{}.{}", subdomain, args.domain)?;
    }
    println!("Subdomains saved successfully.");

    Ok(())
}
