use nannou::{draw::RendererBuilder, prelude::*, wgpu::{CommandEncoder, CommandEncoderDescriptor, Texture, TextureCapturer, TextureSnapshot}};
use std::path::PathBuf;

const DESCRIPTOR: CommandEncoderDescriptor = CommandEncoderDescriptor { label: Some("Lyrhic renderer") };

pub fn image(texture: &Texture, window: &Window, output_path: PathBuf){
    let mut encoder = window.device().create_command_encoder(&DESCRIPTOR);
    render(texture, window, &mut encoder);
    take_snapshot(texture, window, encoder)
        .read(|result| {
            result
                .expect("failed to map texture memory")
                .to_owned()
                .save(output_path)
                .expect("failed to save texture to png image");
        })
        .unwrap();
}

fn render(texture: &Texture, window: &Window, encoder: &mut CommandEncoder) {
    let device = window.device();
    RendererBuilder::new()
        .build_from_texture_descriptor(device, texture.descriptor())
        .render_to_texture(device, encoder, &Draw::new(), texture);
}

fn take_snapshot(texture: &Texture, window: &Window, mut encoder: CommandEncoder) -> TextureSnapshot {
    let device = window.device();
    let texture_capturer = TextureCapturer::default();
    let snapshot = texture_capturer.capture(device, &mut encoder, &texture);

    // Submit command and wait for capture to finish
    window.queue().submit(Some(encoder.finish()));
    texture_capturer.await_active_snapshots(&device).unwrap();

    return snapshot;
}