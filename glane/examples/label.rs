use windows::Win32::System::Com::{
    CoInitializeEx, COINIT_APARTMENTTHREADED, COINIT_DISABLE_OLE1DDE,
};

fn main() -> anyhow::Result<()> {
    unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED | COINIT_DISABLE_OLE1DDE)?;
    }
    let mut event_rx = wiard::EventReceiver::new();
    let window = wiard::Window::builder(&event_rx)
        .title("glane label")
        .build()?;
    let size = window.inner_size().unwrap();
    let dpi = window.dpi().unwrap() as f32;
    let mut pnte_ctx = pnte::Context::new(pnte::Direct2D::new()?)?;
    pnte_ctx.set_dpi(dpi, dpi);
    let render_target = pnte_ctx.create_render_target(&window, (size.width, size.height))?;
    let white_brush = pnte::SolidColorBrush::new(&pnte_ctx, (1.0, 1.0, 1.0, 1.0))?;
    let root = glane::widgets::Label::new("hello, glane!");
    let mut scene = glane::Scene::new(root);
    let face = scene.default_font().unwrap().face.clone();
    let font_size = pnte::FontPoint(scene.default_font().unwrap().size);
    let text_format = pnte::TextFormat::new(
        &pnte_ctx,
        pnte::Font::File(&face.path, "Yu Gothic UI"),
        font_size,
        None,
        None,
    )?;
    loop {
        let Some((event, _)) = event_rx.recv() else {
            break;
        };
        match event {
            wiard::Event::Draw(_) => {
                let layout = scene.layout();
                pnte_ctx.draw(&render_target, |cmd| {
                    cmd.clear((0.0, 0.0, 0.3, 0.0));
                    for l in layout.iter() {
                        if let Some(text) = l.string.as_ref() {
                            cmd.stroke(
                                &pnte::Rect::new(
                                    l.rect.left,
                                    l.rect.top,
                                    l.rect.right,
                                    l.rect.bottom,
                                ),
                                &white_brush,
                                1.0,
                                None,
                            );
                            let text = pnte::TextLayout::new(
                                &pnte_ctx,
                                text,
                                &text_format,
                                pnte::TextAlignment::Center,
                                None
                            )
                            .unwrap();
                            cmd.draw_text(&text, (l.rect.left, l.rect.top), &white_brush)
                                .unwrap();
                        }
                    }
                })?;
            }
            _ => {}
        }
    }
    Ok(())
}
