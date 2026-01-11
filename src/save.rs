use nannou::{prelude::*, wgpu::{CommandEncoder, CommandEncoderDescriptor, Texture, TextureCapturer, TextureSnapshot}};
use std::path::PathBuf;
use crate::model::nannou_info::NannouInfo;

pub fn to_image(app: &App, nannou_info: &mut NannouInfo, output_path: PathBuf) {
    let window = nannou_info.window(app);
    let device = window.device();
    let descriptor = CommandEncoderDescriptor { label: Some("Lyrhic image renderer"), };
    let mut encoder = device.create_command_encoder(&descriptor);

    nannou_info.renderer.render_to_texture(device, &mut encoder, &nannou_info.draw, &nannou_info.texture);

    take_snapshot(&nannou_info.texture, &window, encoder)
        .read(|result| {
            result
                .expect("Failed to map texture memory.")
                .to_owned()
                .save(output_path)
                .expect("Failed to save texture to PNG image.");
        })
        .unwrap();
}

fn take_snapshot(texture: &Texture, window: &Window, mut encoder: CommandEncoder) -> TextureSnapshot {
    let device = window.device();
    let texture_capturer = TextureCapturer::default();
    let snapshot = texture_capturer.capture(device, &mut encoder, &texture);

    window.queue().submit(Some(encoder.finish()));
    texture_capturer.await_active_snapshots(&device).unwrap();

    return snapshot;
}