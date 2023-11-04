#[test]
#[cfg(feature = "std")]
fn t12_clip_box() {
    use agigea::Render;

    let (w, h) = (100, 100);

    let pixf = agigea::Pixfmt::<agigea::Rgb8>::new(w, h);

    let mut ren_base = agigea::RenderingBase::new(pixf);

    ren_base.clear(agigea::Rgba8::new(255, 255, 255, 255));

    let mut ren = agigea::RenderingScanlineAASolid::with_base(&mut ren_base);

    ren.color(agigea::Rgba8::new(255, 0, 0, 255));

    let mut ras = agigea::RasterizerScanline::new();

    ras.clip_box(40.0, 0.0, w as f64 - 40.0, h as f64);

    ras.move_to(10.0, 10.0);
    ras.line_to(50.0, 90.0);
    ras.line_to(90.0, 10.0);

    agigea::render_scanlines(&mut ras, &mut ren);

    ren.to_file("tests/std/tmp/agg_test_12.png").unwrap();

    assert_eq!(
        agigea::ppm::img_diff("tests/std/tmp/agg_test_12.png", "images/agg_test_12.png").unwrap(),
        true
    );
}
