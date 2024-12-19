use glane::ToLogical;
use std::any::TypeId;

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

fn mouse_state(m: &wiard::MouseState, dpi: f32) -> glane::MouseState {
    let position = m.position;
    let position =
        wiard::PhysicalPosition::new(position.x as f32, position.y as f32).to_logical(dpi);
    glane::MouseState {
        position,
        buttons: mouse_buttons(&m.buttons),
    }
}

fn mouse_input(m: &wiard::event::MouseInput, dpi: f32) -> glane::MouseInput {
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
        mouse_state: mouse_state(&m.mouse_state, dpi),
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
        Ok(Self {
            ctx,
            render_target,
            text_format,
            white,
            button_bg,
            button_bg_hover,
            button_bg_pressed,
        })
    }

    fn draw_element<T: pnte::Backend>(&self, cmd: &pnte::DrawCommand<T>, l: &glane::LayoutElement) {
        match l {
            glane::LayoutElement::Area(area) => match l.handle().type_id() {
                t if t == TypeId::of::<glane::widgets::Button>() => {
                    let brush = match area.widget_state {
                        glane::WidgetState::None => &self.button_bg,
                        glane::WidgetState::Hover => &self.button_bg_hover,
                        glane::WidgetState::Pressed => &self.button_bg_pressed,
                    };
                    let rect = l.rect();
                    cmd.fill(
                        &pnte::Rect::new(rect.left, rect.top, rect.right, rect.bottom),
                        brush,
                    );
                }
                _ => {}
            },
            glane::LayoutElement::Text(t) => {
                let text = pnte::TextLayout::new(&self.ctx)
                    .text(&t.string)
                    .format(&self.text_format)
                    .build()
                    .unwrap();
                cmd.draw_text(&text, (l.rect().left, l.rect().top), &self.white)
                    .unwrap();
            }
            glane::LayoutElement::StartClipping(clip) => {
                let rect = pnte::Rect::new(
                    clip.rect.left,
                    clip.rect.top,
                    clip.rect.right,
                    clip.rect.bottom,
                );
                cmd.push_clip(rect);
            }
            glane::LayoutElement::EndClipping(_) => {
                cmd.pop_clip();
            }
            _ => {}
        }
    }

    fn draw(&self, layout: &glane::Layout) -> anyhow::Result<()> {
        self.ctx.draw(&self.render_target, |cmd| {
            cmd.clear((0.1, 0.1, 0.1, 0.0));
            for l in layout.iter() {
                self.draw_element(&cmd, &l);
            }
        })?;
        Ok(())
    }

    fn resize(&mut self, size: wiard::PhysicalSize<u32>) -> anyhow::Result<()> {
        self.render_target.resize((size.width, size.height))?;
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    pnte::co_initialize(pnte::CoInit::ApartmentThreaded)?;
    let mut event_rx = wiard::EventReceiver::new();
    let window = wiard::Window::builder(&event_rx)
        .title("glane counter")
        .inner_size(wiard::LogicalSize::new(640, 480))
        .build()?;
    let (mut scene, root) = { glane::Scene::new(glane::widgets::Column::new()) };
    let counter = glane::widgets::Text::new("0");
    let button = glane::widgets::Button::new("push");
    let counter = scene.push_child(&root, counter);
    let button = scene.push_child(&root, button);
    let mut events = glane::Events::new();
    let mut canvas = Canvas::new(&window, &scene)?;
    let mut count = 0;
    loop {
        let Some((window_event, _)) = event_rx.recv() else {
            break;
        };
        events.clear();
        match window_event {
            wiard::Event::MouseInput(m) => {
                let Some(dpi) = window.dpi() else {
                    continue;
                };
                let input = mouse_input(&m, dpi as f32);
                scene.input(glane::Input::MouseInput(input), &mut events);
                for event in events.iter() {
                    if event.handle() == button
                        && event.message(&button).map_or(false, |msg| {
                            *msg == glane::widgets::button::Message::Clicked
                        })
                    {
                        count += 1;
                        scene.apply(&counter, move |c| {
                            c.text = count.to_string();
                        });
                    }
                }
                window.redraw(None);
            }
            wiard::Event::CursorMoved(m) => {
                let Some(dpi) = window.dpi() else {
                    continue;
                };
                scene.input(
                    glane::Input::CursorMoved(glane::CursorMoved {
                        mouse_state: mouse_state(&m.mouse_state, dpi as f32),
                    }),
                    &mut events,
                );
                if !events.is_empty() {
                    window.redraw(None);
                }
            }
            wiard::Event::Draw(_) => {
                let layout = scene.layout();
                canvas.draw(&layout)?;
                continue;
            }
            wiard::Event::Resized(ev) => {
                let Some(dpi) = window.dpi() else {
                    continue;
                };
                canvas.resize(ev.size)?;
                let size = ev.size.to_logical(dpi);
                scene.set_viewport(glane::LogicalSize::new(
                    size.width as f32,
                    size.height as f32,
                ));
                window.redraw(None);
            }
            _ => {}
        }
    }
    Ok(())
}
