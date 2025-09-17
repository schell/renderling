//! Context manual page.

#[tokio::test]
async fn context_page() {
    // ANCHOR: create
    use renderling::context::Context;

    let ctx = Context::headless(256, 256).await;
    // ANCHOR_END: create

    // ANCHOR: frame
    let frame = ctx.get_next_frame().unwrap();
    // ...do some rendering
    //
    // Then capture the frame into an image, if you like
    let _image_capture = frame.read_image().await.unwrap();
    frame.present();
    // ANCHOR_END: frame
}
