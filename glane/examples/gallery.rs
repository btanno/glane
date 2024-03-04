use glane::ToLogical;
use std::any::TypeId;
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

struct Canvas {
    ctx: pnte::Context<pnte::Direct2D>,
    render_target: pnte::d2d1::RenderTarget,
    text_format: pnte::TextFormat,
    white: pnte::SolidColorBrush,
    button_bg: pnte::SolidColorBrush,
    button_bg_hover: pnte::SolidColorBrush,
    button_bg_pressed: pnte::SolidColorBrush,
    text_box_border: pnte::SolidColorBrush,
    button_type: TypeId,
    text_box_type: TypeId,
}

impl Canvas {
    fn new(window: &wiard::Window, scene: &glane::Scene) -> anyhow::Result<Self> {
        let mut ctx = pnte::Context::new(pnte::Direct2D::new()?)?;
        let dpi = window.dpi().unwrap() as f32;
        let size = window.inner_size().unwrap();
        let render_target = ctx.create_render_target(window, (size.width, size.height))?;
        ctx.set_dpi(dpi, dpi);
        let default_font = scene.default_font().unwrap();
        let font_face = default_font.face.clone();
        let font_size = pnte::FontPoint(default_font.size);
        let text_format = pnte::TextFormat::new(&ctx)
            .font(pnte::Font::File(
                font_face.font_file().path(),
                font_face.font_family_name(),
            ))
            .size(font_size)
            .build()?;
        let white = pnte::SolidColorBrush::new(&ctx, (1.0, 1.0, 1.0, 1.0))?;
        let button_bg = pnte::SolidColorBrush::new(&ctx, (0.3, 0.3, 0.3, 1.0))?;
        let button_bg_hover = pnte::SolidColorBrush::new(&ctx, (0.4, 0.4, 0.4, 1.0))?;
        let button_bg_pressed = pnte::SolidColorBrush::new(&ctx, (0.6, 0.6, 0.6, 1.0))?;
        let text_box_border = pnte::SolidColorBrush::new(&ctx, (0.3, 0.3, 0.3, 1.0))?;
        let button_type = TypeId::of::<glane::widgets::Button>();
        let text_box_type = TypeId::of::<glane::widgets::TextBox>();
        Ok(Self {
            ctx,
            render_target,
            text_format,
            white,
            button_bg,
            button_bg_hover,
            button_bg_pressed,
            text_box_border,
            button_type,
            text_box_type,
        })
    }

    fn draw(&self, layout: &glane::Layout) -> anyhow::Result<()> {
        self.ctx.draw(&self.render_target, |cmd| {
            cmd.clear((0.0, 0.0, 0.3, 0.0));
            for l in layout.iter() {
                match l {
                    glane::LayoutElement::Area(_) => {
                        println!("Area: {:?}", &l.rect());
                        if l.handle().type_id() == self.button_type {
                            let brush = match l.widget_state() {
                                glane::WidgetState::None => &self.button_bg,
                                glane::WidgetState::Hover => &self.button_bg_hover,
                                glane::WidgetState::Pressed => &self.button_bg_pressed,
                            };
                            let rect = l.rect();
                            cmd.fill(
                                &pnte::Rect::new(rect.left, rect.top, rect.right, rect.bottom),
                                brush,
                            );
                        } else if l.handle().type_id() == self.text_box_type {
                            let rect = l.rect();
                            cmd.stroke(
                                &pnte::Rect::new(rect.left, rect.top, rect.right, rect.bottom),
                                &self.text_box_border,
                                2.0,
                                None,
                            );
                        }
                    }
                    glane::LayoutElement::Cursor(_) => {
                        println!("Cursor: {:?}", &l.rect());
                        let rect = l.rect();
                        cmd.fill(
                            &pnte::Rect::new(rect.left, rect.top, rect.right, rect.bottom),
                            &self.white,
                        );
                    }
                    glane::LayoutElement::Text(t) => {
                        println!("{}: {:?}", &t.string, &l.rect());
                        let text = pnte::TextLayout::new(&self.ctx)
                            .text(&t.string)
                            .format(&self.text_format)
                            .build()
                            .unwrap();
                        cmd.draw_text(&text, (l.rect().left, l.rect().top), &self.white)
                            .unwrap();
                    }
                    glane::LayoutElement::CompositionText(t) => {
                        let text = pnte::TextLayout::new(&self.ctx)
                            .text(&t.string)
                            .format(&self.text_format)
                            .build()
                            .unwrap();
                        cmd.draw_text(&text, (l.rect().left, l.rect().top), &self.white)
                            .unwrap();
                        let width = if t.targeted { 2.0 } else { 1.0 };
                        cmd.stroke(
                            &pnte::Line::new(
                                (l.rect().left + 1.0, l.rect().bottom),
                                (l.rect().right - 1.0, l.rect().bottom),
                            ),
                            &self.white,
                            width,
                            None,
                        );
                    }
                }
            }
        })?;
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED | COINIT_DISABLE_OLE1DDE).ok()?;
    }
    let mut event_rx = wiard::EventReceiver::new();
    let window = wiard::Window::builder(&event_rx)
        .title("glane gallery")
        .build()?;
    let (mut scene, root) = {
        let root = glane::widgets::Column::new();
        let handle = glane::Handle::new(&root);
        let scene = glane::Scene::new(root);
        (scene, handle)
    };
    let row = glane::push_child(&mut scene, &root, glane::widgets::Row::new());
    glane::push_child(&mut scene, &row, glane::widgets::Label::new("Button"));
    let button = glane::push_child(&mut scene, &row, glane::widgets::Button::new("Push"));
    let row1 = glane::push_child(&mut scene, &root, glane::widgets::Row::new());
    glane::push_child(&mut scene, &row1, glane::widgets::Label::new("TextBox"));
    let text_box = glane::push_child(&mut scene, &row1, glane::widgets::TextBox::new());
    let canvas = Canvas::new(&window, &scene)?;
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
                for event in events.iter() {
                    if let Some(msg) = event.message(&button) {
                        if msg == &glane::widgets::button::Message::Clicked {
                            println!("button clicked");
                        }
                    } else if let Some(msg) = event.message(&text_box) {
                        match msg {
                            glane::widgets::text_box::Message::Changed => {
                                println!("text_box changed");
                            }
                            _ => {}
                        }
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
            wiard::Event::KeyInput(ev) => {
                scene.input(glane::Input::KeyInput(glane::KeyInput {
                    vkey: ev.key_code.vkey,
                    key_state: ev.key_state,
                }));
                window.redraw(None);
            }
            wiard::Event::CharInput(ev) => {
                scene.input(glane::Input::CharInput(ev.c));
                window.redraw(None);
            }
            wiard::Event::ImeBeginComposition(ev) => {
                let events = scene.input(glane::Input::ImeBeginComposition);
                if let Some(event) = events.iter().find_map(|event| event.message(&text_box)) {
                    if let glane::widgets::text_box::Message::PositionNotify(position) = event {
                        ev.set_position(wiard::LogicalPosition::new(
                            position.x as i32,
                            position.y as i32,
                        ));
                    }
                }
                window.redraw(None);
            }
            wiard::Event::ImeUpdateComposition(ev) => {
                scene.input(glane::Input::ImeUpdateComposition(glane::Composition {
                    chars: ev.chars,
                    clauses: ev
                        .clauses
                        .into_iter()
                        .map(|clause| glane::Clause {
                            range: clause.range,
                            targeted: clause.targeted,
                        })
                        .collect(),
                    cursor_position: ev.cursor_position,
                }));
                window.redraw(None);
            }
            wiard::Event::ImeEndComposition(ev) => {
                scene.input(glane::Input::ImeEndComposition(ev.result));
                window.redraw(None);
            }
            wiard::Event::Draw(_) => {
                let layout = scene.layout();
                canvas.draw(&layout)?;
            }
            _ => {}
        }
    }
    Ok(())
}
