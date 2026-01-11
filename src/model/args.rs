pub use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about = "An image generator which uses text as pixels. Lyrics + Graphic = Lyrhic 😀")]
pub struct Args {    
    /// Path to the source image to transform. 
    #[arg(short, long)]
    pub image: PathBuf,

    /// Path to a text file to use as a sequence of characters for each "char pixel".
    #[arg(short, long)]
    pub text: PathBuf,

    /// Path where the generated image will be stored.
    #[arg(short, long, default_value = "lyrhic_output.png")]
    pub out: PathBuf,

    /// Font size for each "char pixel" in the image.
    #[arg(short, long, default_value_t = 10.0)]
    pub fontsize: f32,

    /// Number of "char pixels" to use as the width and height of the image.
    #[arg(short, long, default_value_t = 100)]
    pub size: u32,
}

