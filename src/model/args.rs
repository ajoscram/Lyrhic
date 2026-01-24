use clap::Parser;
use std::path::PathBuf;
use nannou::text::{Font, font};
use crate::model::Color;

const DEFAULT_FONT: &str = "DEFAULT_FONT";

#[derive(Parser, Debug)]
#[command(author, version, about = "An image generator which uses text as pixels. Lyrics + Graphic = Lyrhic 😀")]
pub struct Args {    
    /// Path to the source image to transform. 
    #[arg(short, long, value_parser=parse_file)]
    pub image: PathBuf,

    /// Path to a text file to use as a sequence of characters for each "char pixel".
    #[arg(short, long, value_parser=parse_file)]
    pub text: PathBuf,
    
    /// Path where the generated image will be stored.
    #[arg(short, long, default_value="lyrhic_output.png")]
    pub out: PathBuf,
    
    /// Path to a .ttf file to use for each character's font.
    #[arg(short, long, default_value=DEFAULT_FONT, value_parser=parse_font)]
    pub font: Font,

    /// Determines the background color as an RGB triple, with each value separated by a comma (e.g. "1,2,3").
    #[arg(short, long, default_value="0,0,0", value_parser=parse_color)]
    pub bg: Color,
    
    /// Size in pixels for each "char pixel" in the image.
    #[arg(short='s', long, default_value_t=32)]
    pub charsize: u32,
    
    /// Number of "char pixels" to use as the width and height of the image.
    #[arg(short='r', long, default_value_t=128)]
    pub charres: u32,

    /// Number of pixels to use as an additional margin to the image.
    #[arg(short, long, default_value_t=0)]
    pub margin: u32,
}

impl Args {
    pub fn size(&self) -> u32 { (self.charres + 2) * self.charsize + (self.margin * 2) }
}

fn parse_file(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);

    if !path.is_file() {
        return Err(format!("Path '{}' is not an existing file.", s));
    }

    Ok(path)
}

fn parse_font(s: &str) -> Result<Font, String> {
    if s == DEFAULT_FONT { return Ok(font::default_notosans()); }
    
    match parse_file(s) {
        Ok(filepath) => match font::from_file(filepath) {
            Ok(font) => Ok(font),
            Err(error) => Err(error.to_string()),
        },
        Err(error) => Err(error),
    }
}

fn parse_color(s: &str) -> Result<Color, String> {
    let parts: Vec<&str> = s.split(',').collect();

    if parts.len() != 3 {
        return Err(format!("Color '{}' must have 3 components (r,g,b)", s));
    }

    let r = parse_u8(parts[0])?;
    let g = parse_u8(parts[1])?;
    let b = parse_u8(parts[2])?;

    Ok(Color::from_components((r, g, b, 255)))
}

fn parse_u8(s: &str) -> Result<u8, String> {
    match s.trim().parse::<u8>() {
        Ok(value) => Ok(value),
        Err(error) => Err(error.to_string()),
    }
}