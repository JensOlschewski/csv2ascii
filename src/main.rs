use std::{error::Error, fs, path::Path, process};

// for command line argument parsing
use clap::Parser;

/// RGB Triplet
#[derive(Debug, Clone)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Parser, Debug)]
#[command(
    name = "csv2ascii",
    about = "Small CLI to convert CSV files to ASCII or ASCII_RGB",
    version = "0.1.0",
    author = "Jens Olschewski"
)]
struct Args {
    /// Input file path
    #[arg()]
    path: String,

    /// Ascii file type
    #[arg(short = 'f', long = "ftype", default_value = "ascii")]
    ftype: String,

    /// Rgb tripple
    #[arg(long = "rgb", num_args = 3, value_parser = clap::value_parser!(u8))]
    rgb: Option<Vec<u8>>,
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let file_path = Path::new(&args.path);

    // Check condition: if ftype == ascii_rgb => rgb must be Some
    let rgb = if args.ftype == "ascii_rgb" {
        match args.rgb {
            Some(vals) if vals.len() == 3 => Some(Rgb {
                r: vals[0],
                g: vals[1],
                b: vals[2],
            }),
            _ => {
                return Err("For ftype=ascii_rgb you must provide --rgb <r> <g> <b>".into());
            }
        }
    } else {
        None
    };

    let content = fs::read_to_string(&file_path)?;
    let ascii_content = content.replace(",", " ");
    let out_path: String;

    match args.ftype.as_str() {
        "ascii" => {
            out_path = format!("{}.ascii", file_path.with_extension("").to_string_lossy());
        }
        "ascii_rgb" => {
            out_path = format!(
                "{}.ascii_rgb",
                file_path.with_extension("").to_string_lossy()
            );
        }
        _ => {
            return Err(From::from(
                "Unsupported file type. Use 'ascii' or 'ascii_rgb'.",
            ));
        }
    }


    // Add rgb values if provided
    let final_content = if let Some(rgb) = rgb {
        let prefix = format!("{} {} {} ", rgb.r, rgb.g, rgb.b);
        ascii_content
            .lines()
            .map(|line| format!("{}{}{}", line, " ", prefix))
            .collect::<Vec<String>>()
            .join("\n")
    } else {
        ascii_content
    };

    fs::write(&out_path, final_content)?;
    println!("File converted: {}", out_path);
    Ok(())
}
