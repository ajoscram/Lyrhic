* Validate args (?)

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
* https://crates.io/crates/cargo-watch
* https://guide.nannou.cc/
* https://docs.rs/nannou/latest/nannou/
