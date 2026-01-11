* iterate the over the image pixels and create rects from them 
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

# Some other bullshit

```rs
    let window = app.main_window();
    let device = window.device();
    let draw = app.draw();
    
    let window_rect = window.rect();
    let pixel_rect = Rect::from_w_h(20.0, 20.0).top_left_of(window_rect);
    draw
        .rect()
        .wh(pixel_rect.wh())
        .rgb8(100, 200, 200);

    let texture = TextureBuilder::new()
        .size([5, 5])
        .usage(TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING)
        .format(wgpu::TextureFormat::Rgba16Float)
        .build(device);
    let descriptor = texture.descriptor();
    
    
    let mut encoder = device.create_command_encoder(&DESCRIPTOR);
    let mut renderer = nannou::draw::RendererBuilder::new().build_from_texture_descriptor(device, descriptor);
    renderer.render_to_texture(&device, &mut encoder, &draw, &texture);
    window.queue().submit(Some(encoder.finish()));

    return texture;  
```

# Render rects to image attempt

```rs
    let window = app.main_window();
    let device = window.device();
    let draw = app.draw();
    let texture = TextureBuilder::new()
        .size([5, 5])
        .usage(TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING)
        .build(device);

    let pixels = image::open(&args.image)
        .unwrap()
        .grayscale()
        .resize(args.size, args.size, FilterType::Gaussian)
        .to_rgb8();

    let window_rect = window.rect();
    for (x, y, pixel) in pixels.enumerate_pixels() {
        let pixel_rect = Rect::from_w_h(1.0, 1.0).top_left_of(window_rect);
        draw
            .rect()
            .x_y(x as f32, y as f32)
            .wh(pixel_rect.wh())
            .rgb8(pixel[0], pixel[1], pixel[2]);
    }

    let mut encoder = device.create_command_encoder(&DESCRIPTOR);
    let descriptor = texture.descriptor();
    let mut renderer = nannou::draw::RendererBuilder::new().build_from_texture_descriptor(device, descriptor);
    renderer.render_to_texture(&device, &mut encoder, &draw, &texture);

    return texture;
```

# Render an image to a texture

```rs
    let image = image::open(&args.image)
        .unwrap()
        .grayscale()
        .resize(args.size, args.size, FilterType::Gaussian);
    
    return Texture::from_image(app, &image);
```
# Links
* https://crates.io/crates/cargo-watch
* https://guide.nannou.cc/
* https://docs.rs/nannou/latest/nannou/
* https://docs.rs/clap/latest/clap/
* https://github.com/nannou-org/nannou/blob/91cd548f8d92cfb8ebcd7bcb2069575acba66088/examples/draw/draw_capture_hi_res.rs