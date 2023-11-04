type PixRgb8 = agigea::Pixfmt<agigea::Rgb8>;
use agigea::Gray8;
use agigea::PixfmtAlphaBlend;

#[test]
fn component_rendering_000() {
    let alpha = 0;
    let (w, h) = (320, 320);

    let pixf = agigea::Pixfmt::<agigea::Rgb8>::new(w, h);
    let mut ren_base = agigea::RenderingBase::new(pixf);
    ren_base.clear(agigea::Rgba8::new(255, 255, 255, 255));
    let g8 = Gray8::new_with_alpha(0, alpha);

    let w2 = (w / 2) as f64;
    let h2 = (h / 2) as f64;
    let er = agigea::Ellipse::new(w2 - 0.87 * 50.0, h2 - 0.5 * 50., 100., 100., 100);
    let eg = agigea::Ellipse::new(w2 + 0.87 * 50.0, h2 - 0.5 * 50., 100., 100., 100);
    let eb = agigea::Ellipse::new(w2, h2 + 50., 100., 100., 100);

    let mut ras = agigea::RasterizerScanline::new();

    {
        let pfr = PixfmtAlphaBlend::<PixRgb8, Gray8>::new(&mut ren_base, 0);
        let mut rbr = agigea::RenderingBase::new(pfr);
        ras.add_path(&er);
        agigea::render_scanlines_aa_solid(&mut ras, &mut rbr, g8);
    }
    {
        let pfg = PixfmtAlphaBlend::<PixRgb8, Gray8>::new(&mut ren_base, 1);
        let mut rbg = agigea::RenderingBase::new(pfg);
        ras.add_path(&eg);
        agigea::render_scanlines_aa_solid(&mut ras, &mut rbg, g8);
    }
    {
        let pfb = PixfmtAlphaBlend::<PixRgb8, Gray8>::new(&mut ren_base, 2);
        let mut rbb = agigea::RenderingBase::new(pfb);
        ras.add_path(&eb);
        agigea::render_scanlines_aa_solid(&mut ras, &mut rbb, g8);
    }

    ren_base.to_file("tests/std/tmp/component_rendering_000.png").unwrap();
    assert_eq!(
        agigea::ppm::img_diff(
            "tests/std/tmp/component_rendering_000.png",
            "tests/images/component_rendering_000.png"
        )
        .unwrap(),
        true
    )
}
