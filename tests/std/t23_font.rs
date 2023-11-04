#[test]
fn t23_font() {
    let lib = agigea::ft::Library::init().unwrap();
    let font = lib.new_face("/System/Library/Fonts/Helvetica.ttc", 0).unwrap();
    font.set_char_size(13 * 64, 0, 72, 0).unwrap();

    let pix = agigea::Pixfmt::<agigea::Rgb8>::new(100, 100);
    let mut ren_base = agigea::RenderingBase::new(pix);
    ren_base.clear(agigea::Rgba8::new(255, 255, 255, 255));

    agigea::draw_text("Hello World!!!", 50, 45, &font, &mut ren_base);

    let mut label = agigea::Label::new("Hello World!!!", 50., 57., 13.0, &font)
        .unwrap()
        .xalign(agigea::XAlign::Center)
        .yalign(agigea::YAlign::Center);
    label.draw(&mut ren_base);

    ren_base.blend_hline(50, 57, 50, agigea::Rgba8::new(255, 0, 0, 255), 255);

    ren_base.to_file("tests/std/tmp/font.png").unwrap();
    assert!(agigea::ppm::img_diff("tests/std/tmp/font.png", "tests/images/font.png").unwrap());
}
