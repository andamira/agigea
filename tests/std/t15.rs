#[test]
#[cfg(feature = "std")]
fn t15_path_stroke() {
    use agigea::Render;

    let (w, h) = (100, 100);

    let pixf = agigea::Pixfmt::<agigea::Rgb8>::new(w, h);

    let mut ren_base = agigea::RenderingBase::new(pixf);

    ren_base.clear(agigea::Rgba8::new(255, 255, 255, 255));

    let mut ren = agigea::RenderingScanlineAASolid::with_base(&mut ren_base);

    ren.color(agigea::Rgba8::new(255, 0, 0, 255));

    let mut ras = agigea::RasterizerScanline::new();

    ras.clip_box(40.0, 0.0, w as f64 - 40.0, h as f64);

    ras.reset();
    ras.move_to(10.0, 10.0);
    ras.line_to(50.0, 90.0);
    ras.line_to(90.0, 10.0);

    agigea::render_scanlines(&mut ras, &mut ren);

    let mut ps = agigea::Path::new();
    ps.remove_all();
    ps.move_to(10.0, 10.0);
    ps.line_to(50.0, 90.0);
    ps.line_to(90.0, 10.0);
    ps.line_to(10.0, 10.0);

    let mut pg = agigea::Stroke::new(ps);

    pg.width(2.0);
    ras.add_path(&mut pg);

    agigea::render_scanlines_aa_solid(&mut ras, &mut ren_base, agigea::Rgba8::new(0, 0, 0, 255));

    ren_base.to_file("tests/std/tmp/agg_test_15.png").unwrap();

    assert_eq!(
        agigea::ppm::img_diff("tests/std/tmp/agg_test_15.png", "tests/images/agg_test_15.png")
            .unwrap(),
        true
    );
}
