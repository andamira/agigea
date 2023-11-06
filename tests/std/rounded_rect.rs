use agrega::Render;

#[test]
fn rounded_rect() {
    let (w, h) = (600, 400);

    let m_x = [100., 500.];
    let m_y = [100., 350.];

    let pixf = agrega::Pixfmt::<agrega::Rgb8>::new(w, h);

    let mut ren_base = agrega::RenderingBase::new(pixf);

    ren_base.clear(agrega::Rgba8::new(255, 255, 255, 255));

    let mut ren = agrega::RenderingScanlineAASolid::with_base(&mut ren_base);

    ren.color(agrega::Rgba8::new(255, 0, 0, 255));

    let mut ras = agrega::RasterizerScanline::new();

    ren.color(agrega::Rgba8::new(54, 54, 54, 255));
    for i in 0..2 {
        let e = agrega::Ellipse::new(m_x[i], m_y[i], 3., 3., 16);
        ras.add_path(&e);
        agrega::render_scanlines(&mut ras, &mut ren);
    }

    let d = 0.0f64;
    let mut r = agrega::RoundedRect::new(m_x[0] + d, m_y[0] + d, m_x[1] + d, m_y[1] + d, 36.0);
    r.normalize_radius();
    r.calc();
    let mut stroke = agrega::Stroke::new(r);
    stroke.width(7.0);
    ras.add_path(&stroke);
    ren.color(agrega::Rgba8::new(0, 0, 0, 255));
    agrega::render_scanlines(&mut ras, &mut ren);

    ren.to_file("tests/std/tmp/rounded_rect.png").unwrap();
    assert_eq!(
        agrega::ppm::img_diff("tests/std/tmp/rounded_rect.png", "tests/images/rounded_rect.png")
            .unwrap(),
        true
    );
}
