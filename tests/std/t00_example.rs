#[test]
fn t00_example() {
    use agigea::Render;

    // Create a blank image 10x10 pixels
    let pix = agigea::Pixfmt::<agigea::Rgb8>::new(100, 100);
    let mut ren_base = agigea::RenderingBase::new(pix);
    ren_base.clear(agigea::Rgba8::white());

    // Draw a polygon from (10,10) - (50,90) - (90,10)
    let mut ras = agigea::RasterizerScanline::new();
    ras.move_to(10.0, 10.0);
    ras.line_to(50.0, 90.0);
    ras.line_to(90.0, 10.0);

    // Render the line to the image
    let mut ren = agigea::RenderingScanlineAASolid::with_base(&mut ren_base);
    ren.color(agigea::Rgba8::black());
    agigea::render_scanlines(&mut ras, &mut ren);

    // Save the image to a file
    ren_base.to_file("tests/std/tmp/little_black_triangle.png").unwrap();
    assert!(agigea::ppm::img_diff(
        "tests/std/tmp/little_black_triangle.png",
        "images/little_black_triangle.png"
    )
    .unwrap());
}
