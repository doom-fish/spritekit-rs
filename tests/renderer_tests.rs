use apple_metal::{
    command_buffer_status, pixel_format, storage_mode, texture_usage, MetalDevice, MetalTexture,
    TextureDescriptor,
};
use spritekit::{
    CGRect, CGSize, Color, NodeExt, RenderPassDescriptor, Renderer, Scene, SpriteNode,
};

struct Fixture {
    device: MetalDevice,
    renderer: Renderer,
    texture: MetalTexture,
    _scene: Scene,
}

fn fixture() -> Fixture {
    let device = MetalDevice::system_default().expect("Metal device");
    let texture = device
        .new_texture(TextureDescriptor {
            usage: texture_usage::RENDER_TARGET | texture_usage::SHADER_READ,
            storage_mode: storage_mode::SHARED,
            ..TextureDescriptor::new_2d(16, 16, pixel_format::BGRA8UNORM)
        })
        .expect("render target");
    let scene = Scene::with_size(CGSize::new(16.0, 16.0)).expect("scene");
    let sprite = SpriteNode::with_color(Color::red(), CGSize::new(16.0, 16.0)).expect("sprite");
    scene.add_child(&sprite).expect("add sprite");
    let renderer = Renderer::new(&device).expect("renderer");
    renderer.set_scene(Some(&scene));
    renderer.update_at_time(0.0);
    Fixture {
        device,
        renderer,
        texture,
        _scene: scene,
    }
}

#[test]
fn render_returns_an_uncommitted_command_buffer() {
    let fixture = fixture();
    let queue = fixture.device.new_command_queue().expect("queue");
    let pass = RenderPassDescriptor::for_texture(&fixture.texture, Color::black()).expect("pass");
    let command_buffer = fixture
        .renderer
        .render(CGRect::new(0.0, 0.0, 16.0, 16.0), &queue, &pass)
        .expect("render");
    assert_eq!(command_buffer.status(), command_buffer_status::NOT_ENQUEUED);
    command_buffer.commit().expect("commit");
    command_buffer.wait_until_completed().expect("wait");
    assert_eq!(command_buffer.status(), command_buffer_status::COMPLETED);
}

#[test]
fn render_into_rejects_committed_command_buffers() {
    let fixture = fixture();
    let queue = fixture.device.new_command_queue().expect("queue");
    let pass = RenderPassDescriptor::for_texture(&fixture.texture, Color::black()).expect("pass");
    let viewport = CGRect::new(0.0, 0.0, 16.0, 16.0);

    let enqueued = queue.new_command_buffer().expect("command buffer");
    enqueued.enqueue().expect("enqueue");
    fixture
        .renderer
        .render_into(viewport, &enqueued, &pass)
        .expect("an enqueued command buffer still accepts work");
    enqueued.commit().expect("commit");
    enqueued.wait_until_completed().expect("wait");

    let error = fixture
        .renderer
        .render_into(viewport, &enqueued, &pass)
        .expect_err("a completed command buffer must be rejected");
    assert!(error.to_string().contains("already committed"), "{error}");

    let committed = queue.new_command_buffer().expect("command buffer");
    committed.commit().expect("commit");
    let error = fixture
        .renderer
        .render_into(viewport, &committed, &pass)
        .expect_err("a committed command buffer must be rejected");
    assert!(error.to_string().contains("already committed"), "{error}");
    committed.wait_until_completed().expect("wait");
}

#[test]
fn render_into_refuses_a_command_buffer_with_an_open_encoder() {
    let fixture = fixture();
    let queue = fixture.device.new_command_queue().expect("queue");
    let pass = RenderPassDescriptor::for_texture(&fixture.texture, Color::black()).expect("pass");
    let viewport = CGRect::new(0.0, 0.0, 16.0, 16.0);
    let command_buffer = queue.new_command_buffer().expect("command buffer");

    let encoder = command_buffer
        .new_blit_command_encoder()
        .expect("blit encoder");
    let error = fixture
        .renderer
        .render_into(viewport, &command_buffer, &pass)
        .expect_err("SKRenderer aborts while another encoder is open");
    assert!(error.to_string().contains("still open"), "{error}");
    encoder.end_encoding().expect("end encoding");

    fixture
        .renderer
        .render_into(viewport, &command_buffer, &pass)
        .expect("render after the encoder ended");
    command_buffer.commit().expect("commit");
    command_buffer.wait_until_completed().expect("wait");
}

#[test]
fn render_into_holds_off_commits_from_other_threads() {
    let fixture = fixture();
    let queue = fixture.device.new_command_queue().expect("queue");
    let pass = RenderPassDescriptor::for_texture(&fixture.texture, Color::black()).expect("pass");
    let viewport = CGRect::new(0.0, 0.0, 16.0, 16.0);
    let (mut drawn, mut refused) = (0, 0);

    for delay in (0..512).map(|iteration| iteration % 64) {
        let command_buffer = queue.new_command_buffer().expect("command buffer");
        let committer = command_buffer.clone();
        let barrier = std::sync::Barrier::new(2);
        let (render, commit) = std::thread::scope(|scope| {
            let commit = scope.spawn(|| {
                barrier.wait();
                let start = std::time::Instant::now();
                while start.elapsed() < std::time::Duration::from_micros(delay * 2) {
                    std::hint::spin_loop();
                }
                committer.commit()
            });
            barrier.wait();
            let render = fixture
                .renderer
                .render_into(viewport, &command_buffer, &pass);
            (render, commit.join().expect("commit thread"))
        });
        match (render, commit) {
            (Ok(()), Ok(())) => drawn += 1,
            (Ok(()), Err(apple_metal::CommandBufferError::ActiveEncoder)) => {
                drawn += 1;
                command_buffer.commit().expect("commit after the render");
            }
            (Err(error), Ok(())) if error.to_string().contains("already committed") => {
                refused += 1;
            }
            (render, commit) => panic!("unexpected outcome {render:?} {commit:?}"),
        }
        command_buffer.wait_until_completed().expect("wait");
    }
    assert_eq!(drawn + refused, 512);
}
