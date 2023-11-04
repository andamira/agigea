use agigea::Render;

#[test]
fn rounded_rect() {
    let (w, h) = (600, 400);

    let m_x = [100., 500.];
    let m_y = [100., 350.];

    let pixf = agigea::Pixfmt::<agigea::Rgb8>::new(w, h);

    let mut ren_base = agigea::RenderingBase::new(pixf);

    ren_base.clear(agigea::Rgba8::new(255, 255, 255, 255));

    let mut ren = agigea::RenderingScanlineAASolid::with_base(&mut ren_base);

    ren.color(agigea::Rgba8::new(255, 0, 0, 255));

    let mut ras = agigea::RasterizerScanline::new();

    ren.color(agigea::Rgba8::new(54, 54, 54, 255));
    for i in 0..2 {
        let e = agigea::Ellipse::new(m_x[i], m_y[i], 3., 3., 16);
        ras.add_path(&e);
        agigea::render_scanlines(&mut ras, &mut ren);
    }

    let d = 0.0f64;
    let mut r = agigea::RoundedRect::new(m_x[0] + d, m_y[0] + d, m_x[1] + d, m_y[1] + d, 36.0);
    r.normalize_radius();
    r.calc();
    let mut stroke = agigea::Stroke::new(r);
    stroke.width(7.0);
    ras.add_path(&stroke);
    ren.color(agigea::Rgba8::new(0, 0, 0, 255));
    agigea::render_scanlines(&mut ras, &mut ren);

    ren.to_file("tests/std/tmp/rounded_rect.png").unwrap();
    assert_eq!(
        agigea::ppm::img_diff("tests/std/tmp/rounded_rect.png", "images/rounded_rect.png").unwrap(),
        true
    );
}
