use dirs::home_dir;
use image::DynamicImage;
use image::GenericImageView;
use rand::seq::SliceRandom;
use std::error::Error;
use std::fs;
use std::io;
use std::path::PathBuf;
use viuer::print;
use viuer::Config;

macro_rules! either {
    ($test:expr => $true_expr:expr; $false_expr:expr) => {
        if $test {
            $true_expr
        } else {
            $false_expr
        }
    };
}

fn read_files_in_dir(dir_path: &str) -> io::Result<Vec<String>> {
    let mut file_paths = Vec::new();

    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if let Some(path_str) = path.to_str() {
            file_paths.push(path_str.to_string());
        }
    }

    Ok(file_paths)
}

fn resize(img: &DynamicImage, config: &config::Config) -> Result<(u32, u32), Box<dyn Error>> {
    let (width, height) = img.dimensions();
    Ok((
        width / config.get::<u32>("width")? * (200 / width) as u32,
        height / config.get::<u32>("height")? * (200 / height) as u32,
    ))
}

fn print_usage() {
    println!("Usage: waifu-colorscripts [OPTIONS]");
    println!();
    println!("Options:");
    println!("  --name <name>        Filter images by character name.");
    println!("  --file-name <name>   Specify a particular image file.");
    println!("  --help, -h, --usage  Display this help message.");
    println!();
    println!("If no options are provided, a random waifu image will be displayed.");
}

#[allow(unused_assignments)]
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let mut i = 1;

    let mut character_name: String = String::new();
    let mut file_name: String = String::new();

    while i < args.len() {
        match args[i].as_str() {
            "--name" => {
                either!(i + 1 < args.len() && args[i + 1].len() > 0 => {
                    character_name = String::from(&args[i + 1]);
                    i += 2;
                }; {
                    println!("--name flag is missing a value!");
                    i += 1;
                });
            }
            "--file-name" => {
                either!(i + 1 < args.len() && args[i + 1].len() > 0 => {
                    file_name = String::from(&args[i + 1]);
                    i += 2;
                }; {
                    println!("--file-name flag is missing a value!");
                    i += 1;
                });
            }
            "--help" | "-h" | "--usage" => {
                print_usage();
                return Ok(());
            }
            _ => {
                println!("Unknown argument: {}", args[i]);
                i += 1;
            }
        }
    }

    let home = home_dir().expect("Could not find home directory");
    let dir_path = home.join(".local/share/waifu-colorscripts/");
    let images = read_files_in_dir(dir_path.join("images").to_str().unwrap())?;
    let mut random_file: &str = "";

    if character_name.is_empty() && file_name.is_empty() {
        random_file = images
            .choose(&mut rand::thread_rng())
            .ok_or("No files found")?;
        print_waifu(random_file, dir_path)?;
        return Ok(());
    }

    if !file_name.is_empty() {
        random_file = &file_name;
        print_waifu(random_file, dir_path)?;
        return Ok(());
    }

    let mut matching_files: Vec<&String> = Vec::new();

    if !character_name.is_empty() {
        matching_files = images
            .iter()
            .filter(|&file| file.to_lowercase().contains(&character_name.to_lowercase()))
            .collect();

        random_file = matching_files
            .choose(&mut rand::thread_rng())
            .ok_or("No files found")?;

        print_waifu(random_file, dir_path)?;
    }

    Ok(())
}

fn print_waifu(path: &str, dir_path: PathBuf) -> Result<(), Box<dyn Error>> {
    let config = config::Config::builder()
        .add_source(config::File::with_name(
            dir_path.join("conf.toml").to_str().unwrap(),
        ))
        .build()
        .unwrap();

    let img = image::open(path)?;
    let (width, height) = resize(&img, &config)?;
    let config_height = (height as i16) * -1;
    let config = Config {
        transparent: true,
        restore_cursor: false,
        absolute_offset: false,
        x: 2,
        y: either!(config_height % 2 == 0 => config_height - 2 ; config_height - 3),
        width: Some(width),
        height: Some(height),
        ..Config::default()
    };

    let mut x = 0;
    let mut y = config.height.unwrap();
    either!(y % 2 == 0 => y += 3; y += 4);
    while x < y {
        println!();
        x += 1
    }
    print(&img, &config).unwrap();
    Ok(())
}
