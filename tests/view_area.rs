use spritekit::{CGRect, CGSize, Scene, View, ViewDelegate};

#[test]
fn view_with_frame_is_rejected_off_the_main_thread() {
    let error = std::thread::spawn(|| {
        View::with_frame(CGRect::new(0.0, 0.0, 160.0, 90.0))
            .map(drop)
            .expect_err("SKView must not be created off the main thread")
    })
    .join()
    .expect("worker thread");
    assert!(error.to_string().contains("main thread"), "{error}");
}

#[test]
fn scenes_outside_a_view_report_no_view() {
    let scene = Scene::with_size(CGSize::new(160.0, 90.0)).expect("scene");
    assert!(scene.view().is_none());
    assert!(ViewDelegate::new().is_some());
}
