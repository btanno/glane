use std::any::TypeId;
use wiard::ToLogical;
use windows::Win32::System::Com::{
    CoInitializeEx, COINIT_APARTMENTTHREADED, COINIT_DISABLE_OLE1DDE,
};

fn mouse_buttons(src: &wiard::MouseButtons) -> glane::MouseButtons {
    let mut dest = glane::MouseButtons::new();
    if src.contains(wiard::MouseButton::Left) {
        dest |= glane::MouseButton::Left;
    }
    if src.contains(wiard::MouseButton::Right) {
        dest |= glane::MouseButton::Right;
    }
    if src.contains(wiard::MouseButton::Middle) {
        dest |= glane::MouseButton::Middle;
    }
    for i in 0..glane::MouseButton::EX_LEN {
        if src.contains(wiard::MouseButton::Ex(i)) {
            dest |= glane::MouseButton::Ex(i);
        }
    }
    dest
}

fn mouse_input(m: &wiard::event::MouseInput, dpi: f32) -> glane::MouseInput {
    let position = m.mouse_state.position;
    let position =
        wiard::PhysicalPosition::new(position.x as f32, position.y as f32).to_logical(dpi);
    glane::MouseInput {
        button: match m.button {
            wiard::MouseButton::Left => glane::MouseButton::Left,
            wiard::MouseButton::Right => glane::MouseButton::Right,
            wiard::MouseButton::Middle => glane::MouseButton::Middle,
            wiard::MouseButton::Ex(n) => glane::MouseButton::Ex(n),
        },
        button_state: match m.button_state {
            wiard::ButtonState::Pressed => glane::ButtonState::Pressed,
            wiard::ButtonState::Released => glane::ButtonState::Released,
        },
        mouse_state: glane::MouseState {
            position,
            buttons: mouse_buttons(&m.mouse_state.buttons),
        },
    }
}

fn main() -> anyhow::Result<()> {
    unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED | COINIT_DISABLE_OLE1DDE)?;
    }
    let mut event_rx = wiard::EventReceiver::new();
    let window = wiard::Window::builder(&event_rx)
        .title("glane button")
        .build()?;
    let size = window.inner_size().unwrap();
    let dpi = window.dpi().unwrap() as f32;
    let mut pnte_ctx = pnte::Context::new(pnte::Direct2D::new()?)?;
    pnte_ctx.set_dpi(dpi, dpi);
    let render_target = pnte_ctx.create_render_target(&window, (size.width, size.height))?;
    let white_brush = pnte::SolidColorBrush::new(&pnte_ctx, (1.0, 1.0, 1.0, 1.0))?;
    let bg_brush = pnte::SolidColorBrush::new(&pnte_ctx, (0.2, 0.2, 0.2, 1.0))?;
    let bg_hover_brush = pnte::SolidColorBrush::new(&pnte_ctx, (0.3, 0.3, 0.3, 1.0))?;
    let bg_pressed_brush = pnte::SolidColorBrush::new(&pnte_ctx, (0.5, 0.5, 0.5, 1.0))?;
    let button_entity = glane::widgets::Button::new(format!("{:^4}", 0));
    let button = glane::Handle::new(&button_entity);
    let root = glane::widgets::Abs::new(button_entity, glane::LogicalPosition::new(100.0, 100.0));
    let mut scene = glane::Scene::new(root);
    let default_font = scene.default_font().unwrap();
    let face = default_font.face.clone();
    let font_size = pnte::FontPoint(default_font.size);
    let text_format = pnte::TextFormat::new(
        &pnte_ctx,
        pnte::Font::File(&face.path, "Yu Gothic UI"),
        font_size,
        None,
        None,
    )?;
    let mut count = 0;
    loop {
        let Some((event, _)) = event_rx.recv() else {
            break;
        };
        match event {
            wiard::Event::MouseInput(m) => {
                let Some(dpi) = window.dpi() else {
                    continue;
                };
                let input = mouse_input(&m, dpi as f32);
                let events = scene.input(glane::Input::MouseInput(input));
                for msg in events.iter().filter_map(|ev| ev.message(&button)) {
                    if msg == &glane::widgets::button::Message::Clicked {
                        println!("clicked");
                        count += 1;
                        scene.apply(&button, move |btn| {
                            btn.text = format!("{count:^4}");
                        });
                    }
                }
                window.redraw(None);
            }
            wiard::Event::CursorMoved(m) => {
                let Some(dpi) = window.dpi() else {
                    continue;
                };
                let position = m.mouse_state.position;
                let position = wiard::PhysicalPosition::new(position.x as f32, position.y as f32)
                    .to_logical(dpi as f32);
                let mouse_state = glane::MouseState {
                    position,
                    buttons: mouse_buttons(&m.mouse_state.buttons),
                };
                let events = scene.input(glane::Input::CursorMoved(glane::CursorMoved {
                    mouse_state,
                }));
                if glane::state_changed_exists(&events) {
                    window.redraw(None);
                }
            }
            wiard::Event::Draw(_) => {
                let layout = scene.layout();
                pnte_ctx.draw(&render_target, |cmd| {
                    cmd.clear((0.0, 0.0, 0.3, 0.0));
                    for l in layout.iter() {
                        if l.handle.type_id() == TypeId::of::<glane::widgets::Button>() {
                            if let Some(text) = l.string.as_ref() {
                                let text = pnte::TextLayout::new(
                                    &pnte_ctx,
                                    text,
                                    &text_format,
                                    pnte::TextAlignment::Center,
                                    None,
                                )
                                .unwrap();
                                cmd.draw_text(&text, (l.rect.left, l.rect.top), &white_brush)
                                    .unwrap();
                            } else {
                                let brush = match l.state {
                                    glane::WidgetState::None => &bg_brush,
                                    glane::WidgetState::Hover => &bg_hover_brush,
                                    glane::WidgetState::Pressed => &bg_pressed_brush,
                                };
                                cmd.fill(
                                    &pnte::Rect::new(
                                        l.rect.left,
                                        l.rect.top,
                                        l.rect.right,
                                        l.rect.bottom,
                                    ),
                                    brush,
                                );
                            }
                        }
                    }
                })?;
            }
            _ => {}
        }
    }
    Ok(())
}
