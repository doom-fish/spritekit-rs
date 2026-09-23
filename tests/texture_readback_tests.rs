use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{mpsc, Arc};
use std::time::{Duration, Instant};

use apple_metal::{
    pixel_format, storage_mode, texture_usage, MetalDevice, MetalTexture, TextureDescriptor,
};
use spritekit::{
    read_texture_bytes, CGPoint, CGRect, CGSize, Color, MutableTexture, NodeExt,
    RenderPassDescriptor, Renderer, Scene, SpriteNode,
};

mod common;

use common::autoreleasepool;

const TIMEOUT: Duration = Duration::from_secs(5);

fn device() -> MetalDevice {
    MetalDevice::system_default().expect("Metal device")
}

fn upload(texture: &MetalTexture, bytes: &[u8], bytes_per_row: usize) {
    unsafe {
        texture
            .replace_region_2d(
                bytes,
                bytes_per_row,
                (0, 0),
                (texture.width(), texture.height()),
                0,
            )
            .expect("upload texture bytes");
    }
}

fn wait_until(condition: impl Fn() -> bool) -> bool {
    let deadline = Instant::now() + TIMEOUT;
    while Instant::now() < deadline {
        if condition() {
            return true;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    condition()
}

#[test]
fn rgba16_float_textures_are_read_with_eight_bytes_per_pixel() {
    let device = device();
    let (width, height) = (5, 3);
    let texture = device
        .new_texture(TextureDescriptor {
            usage: texture_usage::SHADER_READ | texture_usage::RENDER_TARGET,
            storage_mode: storage_mode::SHARED,
            ..TextureDescriptor::new_2d(width, height, pixel_format::RGBA16FLOAT)
        })
        .expect("RGBA16Float texture");
    let pattern: Vec<u8> = (0..width * height * 8)
        .map(|index| u8::try_from(index % 251).expect("byte"))
        .collect();
    upload(&texture, &pattern, width * 8);

    let bytes = read_texture_bytes(&texture).expect("read RGBA16Float");
    assert_eq!(bytes.len(), width * height * 8);
    assert_eq!(bytes, pattern);
}

#[test]
fn bgra10_xr_textures_are_read_with_eight_bytes_per_pixel() {
    let device = device();
    let Some(texture) = device.new_texture(TextureDescriptor {
        usage: texture_usage::SHADER_READ,
        storage_mode: storage_mode::SHARED,
        ..TextureDescriptor::new_2d(4, 4, pixel_format::BGRA10_XR)
    }) else {
        eprintln!("skipping: this GPU has no BGRA10_XR textures");
        return;
    };
    let pattern: Vec<u8> = (0_usize..4 * 4 * 8)
        .map(|index| u8::try_from(index * 7 % 256).expect("byte"))
        .collect();
    upload(&texture, &pattern, 4 * 8);
    let bytes = read_texture_bytes(&texture).expect("read BGRA10_XR");
    assert_eq!(bytes, pattern);
}

#[test]
fn private_textures_are_rejected() {
    let texture = device()
        .new_texture(TextureDescriptor {
            usage: texture_usage::SHADER_READ | texture_usage::RENDER_TARGET,
            storage_mode: storage_mode::PRIVATE,
            ..TextureDescriptor::new_2d(8, 8, pixel_format::RGBA16FLOAT)
        })
        .expect("private texture");
    let error = read_texture_bytes(&texture).expect_err("private storage must be rejected");
    assert!(error.to_string().contains("storage mode"), "{error}");
}

#[test]
fn compressed_textures_are_rejected() {
    let device = device();
    let texture = [pixel_format::BC1_RGBA, pixel_format::ASTC_4X4_LDR]
        .into_iter()
        .find_map(|format| {
            device.new_texture(TextureDescriptor {
                usage: texture_usage::SHADER_READ,
                storage_mode: storage_mode::SHARED,
                ..TextureDescriptor::new_2d(16, 16, format)
            })
        });
    let Some(texture) = texture else {
        eprintln!("skipping: this GPU supports neither BC nor ASTC textures");
        return;
    };
    let error = read_texture_bytes(&texture).expect_err("compressed formats must be rejected");
    assert!(error.to_string().contains("compressed"), "{error}");
}

#[test]
fn depth_textures_are_rejected() {
    let Some(texture) = device().new_texture(TextureDescriptor {
        usage: texture_usage::SHADER_READ,
        storage_mode: storage_mode::SHARED,
        ..TextureDescriptor::new_2d(8, 8, pixel_format::DEPTH32FLOAT)
    }) else {
        eprintln!("skipping: shared depth textures are unavailable");
        return;
    };
    let error = read_texture_bytes(&texture).expect_err("depth formats must be rejected");
    assert!(error.to_string().contains("depth"), "{error}");
}

#[test]
fn rendered_scene_pixels_are_read_back() {
    let device = device();
    let queue = device.new_command_queue().expect("command queue");
    let texture = device
        .new_texture(TextureDescriptor {
            usage: texture_usage::RENDER_TARGET | texture_usage::SHADER_READ,
            storage_mode: storage_mode::SHARED,
            ..TextureDescriptor::new_2d(32, 32, pixel_format::BGRA8UNORM)
        })
        .expect("render target");
    let scene = Scene::with_size(CGSize::new(32.0, 32.0)).expect("scene");
    scene.set_background_color(Color::black());
    scene.set_anchor_point(CGPoint::new(0.5, 0.5));
    let sprite = SpriteNode::with_color(Color::red(), CGSize::new(32.0, 32.0)).expect("sprite");
    scene.add_child(&sprite).expect("add sprite");
    let renderer = Renderer::new(&device).expect("renderer");
    renderer.set_scene(Some(&scene));
    let pass = RenderPassDescriptor::for_texture(&texture, Color::black()).expect("pass");
    let command_buffer = queue.new_command_buffer().expect("command buffer");
    renderer.update_at_time(0.0);
    renderer.render(CGRect::new(0.0, 0.0, 32.0, 32.0), &command_buffer, &pass);
    command_buffer.commit().expect("commit");
    command_buffer.wait_until_completed().expect("complete");

    let pixels = read_texture_bytes(&texture).expect("read BGRA8");
    assert_eq!(pixels.len(), 32 * 32 * 4);
    let center = (16 * 32 + 16) * 4;
    assert_eq!(&pixels[center..center + 4], &[0, 0, 255, 255]);
}

#[test]
fn modify_pixel_data_runs_once_and_releases_its_closure() {
    let texture = MutableTexture::with_size(CGSize::new(2.0, 2.0)).expect("mutable texture");
    let runs = Arc::new(AtomicUsize::new(0));
    let counter = Arc::clone(&runs);
    let (tx, rx) = mpsc::channel();
    texture.modify_pixel_data(move |pixels| {
        pixels.fill(0x7f);
        counter.fetch_add(1, Ordering::SeqCst);
        let _ = tx.send(pixels.len());
    });

    assert_eq!(rx.recv_timeout(TIMEOUT).expect("pixel callback"), 16);
    assert!(wait_until(|| Arc::strong_count(&runs) == 1));
    assert_eq!(runs.load(Ordering::SeqCst), 1);
}

#[test]
fn modify_pixel_data_releases_its_closure_when_the_texture_goes_away() {
    let runs = Arc::new(AtomicUsize::new(0));
    autoreleasepool(|| {
        let texture = MutableTexture::with_size(CGSize::new(2.0, 2.0)).expect("mutable texture");
        for _ in 0..4 {
            let counter = Arc::clone(&runs);
            texture.modify_pixel_data(move |_| {
                counter.fetch_add(1, Ordering::SeqCst);
            });
        }
        drop(texture);
    });

    assert!(wait_until(|| Arc::strong_count(&runs) == 1));
    assert!(runs.load(Ordering::SeqCst) <= 4);
}
