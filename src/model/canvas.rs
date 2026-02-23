use std::{cell::Ref, path::PathBuf};
use nannou::{draw::{Renderer, RendererBuilder}, prelude::*, wgpu::{CommandEncoder, CommandEncoderDescriptor, Device, Texture, TextureBuilder, TextureCapturer, TextureFormat, TextureReshaper, TextureSnapshot, TextureUsages}};
use crate::model::{Args, Drawable};

const PREVIEW_SIZE: u32 = 512;

pub struct Canvas {
    window_id: WindowId,
    draw: Draw,
    texture: Texture,
    capturer: TextureCapturer,
}

impl Canvas {
    pub fn new(app: &App, args: &Args) -> Self {
        let window_id = get_window_id(app);
        let window = get_window(app, window_id);
        let texture = get_texture(args, &window);
        
        Self {
            window_id,
            texture,
            draw: Draw::new(),
            capturer: TextureCapturer::default(),
        }
    }

    pub fn draw(&self, drawable: &impl Drawable) { drawable.draw_into(&self.draw); }

    pub fn render(&mut self, app: &App) -> TextureSnapshot {
        let window = get_window(app, self.window_id);
        let device = window.device();
        let mut encoder = get_encoder(device);
        let mut renderer = get_renderer(device, &self.texture);

        renderer.render_to_texture(device, &mut encoder, &self.draw, &self.texture);
        let snapshot = self.capturer.capture(device, &mut encoder, &self.texture);
        window.queue().submit(Some(encoder.finish()));

        snapshot
    }

    pub fn show(&self, app: &App, frame: Frame) {
        let window = get_window(app, self.window_id);
        let reshaper = get_texture_reshaper(&self.texture, &window);
        reshaper.encode_render_pass(frame.texture_view(), &mut *frame.command_encoder());
    }

    pub fn save(&self, app: &App, snapshot: TextureSnapshot, output_path: &PathBuf) {
        let output_path_clone = output_path.clone();
        snapshot
            .read(|result| {
                result
                    .expect("Failed to map texture memory.")
                    .to_owned()
                    .save(output_path_clone)
                    .expect("Failed to save texture to PNG image.");
            })
            .unwrap();

        // TODO: await_active_snapshots is not working as intended.
        // This seems to do something for now, but needs to get removed eventually
        let window = get_window(app, self.window_id);
        println!("THIS SHOULDN'T BE HERE, NOR DISPLAY 0: {}", self.capturer.active_snapshots());
        self.capturer.await_active_snapshots(window.device()).unwrap();
        println!("THIS SHOULDN'T BE HERE EITHER.");
    }
}

fn get_window_id(app: &App) -> WindowId {
    app
        .new_window()
        .size(PREVIEW_SIZE, PREVIEW_SIZE)
        .build()
        .unwrap()
}

fn get_window<'a>(app: &'a App, window_id: WindowId) -> Ref<'a, Window> { app.window(window_id).unwrap() }

fn get_texture(args: &Args, window: &Window) -> Texture {
    TextureBuilder::new()
        .size([args.size(), args.size()])
        .usage(TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING)
        .sample_count(window.msaa_samples())
        .format(TextureFormat::Rgba16Float)
        .build(window.device())
}

fn get_texture_reshaper(texture: &Texture, window: &Window) -> TextureReshaper {
    TextureReshaper::new(
        window.device(),
        &texture.view().build(),
        window.msaa_samples(),
        texture.sample_type(),
        window.msaa_samples(),
        Frame::TEXTURE_FORMAT,
    )
}

fn get_encoder(device: &Device) -> CommandEncoder {
    let desc = CommandEncoderDescriptor { label: Some("Lyrhic image renderer"), };
    device.create_command_encoder(&desc)
}

fn get_renderer(device: &Device, texture: &Texture) -> Renderer {
    RendererBuilder::new()
        .build_from_texture_descriptor(
            device,
            texture.descriptor())
}