use dirs::home_dir;
use image::DynamicImage;
use image::GenericImageView;
use rand::seq::SliceRandom;
use std::error::Error;
use std::fs;
use std::io;
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

fn main() -> Result<(), Box<dyn Error>> {
    let home = home_dir().expect("Could not find home directory");
    let dir_path = home.join(".local/share/waifu-colorscripts/");
    let images = read_files_in_dir(dir_path.join("images").to_str().unwrap())?;
    let config = config::Config::builder()
        .add_source(config::File::with_name(
            dir_path.join("conf.toml").to_str().unwrap(),
        ))
        .build()
        .unwrap();

    let random_file = images
        .choose(&mut rand::thread_rng())
        .ok_or("No files found")?;
    let img = image::open(random_file)?;
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
