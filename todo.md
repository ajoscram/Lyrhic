* Add an optional parameter to select a margin size
* Add an optional parameter for a path to a font
* Change `fontsize` parameter to `charsize`
* Add a config to select the background color for the image
* Validate args
```rs
// Custom validation function
fn validate_existing_file(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);
    if path.is_file() {
        Ok(path)
    } else {
        Err(format!("Path '{}' is not an existing file.", s))
    }
}
```

# Links
* https://fonts.google.com/?categoryFilters=Serif:%2FSerif%2FFat+Face
* https://crates.io/crates/cargo-watch
* https://guide.nannou.cc/
* https://docs.rs/nannou/latest/nannou/
* https://docs.rs/clap/latest/clap/
* https://github.com/nannou-org/nannou/blob/91cd548f8d92cfb8ebcd7bcb2069575acba66088/examples/draw/draw_capture_hi_res.rs