use std::{env::args, fs, path::Path};

use config::ExtendedConfig;

pub mod config;

#[derive(Debug)]
enum ReturnError {
    ArgError,
    FileError,
    UnhandledError,
}

fn main() -> Result<(), ReturnError> {
    let args = args().collect::<Vec<String>>();

    if args.len() != 2 {
        println!("Error: wrong args, you need to pass a path to a .cfgx file");
        // TODO : print usage
        return Err(ReturnError::ArgError);
    }

    let config_file_path = Path::new(args.get(1).ok_or(ReturnError::UnhandledError)?);

    if config_file_path.extension().map(|ext| ext.to_str()) != Some(Some("cfgx")) {
        println!("Error: wrong args, you need to pass a path to a .cfgx file");
        // TODO : print usage
        return Err(ReturnError::ArgError);
    }

    let file_content_buffer = fs::read(config_file_path).map_err(|_| ReturnError::FileError)?;

    let file_content_string = String::from_utf8_lossy(&file_content_buffer);

    let mut config = ExtendedConfig::new(file_content_string.to_string());

    config.parse();

    let output_file_path = config_file_path.with_extension("cfg");

    let output_file_content = config.export();

    if let Err(e) = fs::write(&output_file_path, output_file_content) {
        println!(
            "Failed writing to file {} : {}",
            output_file_path.to_str().ok_or(ReturnError::FileError)?,
            e
        );
    }

    Ok(())
}
