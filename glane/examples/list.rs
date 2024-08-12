use glane::widgets::*;
use glane::ToLogical;
use std::any::TypeId;
use std::cell::Cell;
use std::rc::Rc;

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

fn mouse_wheel(m: &wiard::event::MouseWheel, dpi: f32) -> glane::MouseWheel {
    glane::MouseWheel {
        axis: match m.axis {
            wiard::MouseWheelAxis::Vertical => glane::MouseWheelAxis::Vertical,
            wiard::MouseWheelAxis::Horizontal => glane::MouseWheelAxis::Horizontal,
        },
        distance: -m.distance / wiard::WHEEL_DELTA,
        mouse_state: mouse_state(&m.mouse_state, dpi),
    }
}

struct Canvas {
    ctx: pnte::Context<pnte::Direct2D>,
    render_target: pnte::d2d1::RenderTarget,
    text_format: pnte::TextFormat,
    list_box_bg: pnte::SolidColorBrush,
    border_color: pnte::SolidColorBrush,
    text_color: pnte::SolidColorBrush,
    selected_bg: pnte::SolidColorBrush,
    button_bg: pnte::SolidColorBrush,
    button_bg_hover: pnte::SolidColorBrush,
    button_bg_pressed: pnte::SolidColorBrush,
    scroll_bar_bg: pnte::SolidColorBrush,
    scroll_bar_thumb: pnte::SolidColorBrush,
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
        let list_box_bg = pnte::SolidColorBrush::new(&ctx, (0.1, 0.1, 0.1, 0.9))?;
        let border_color = pnte::SolidColorBrush::new(&ctx, (1.0, 1.0, 1.0, 1.0))?;
        let text_color = pnte::SolidColorBrush::new(&ctx, (1.0, 1.0, 1.0, 1.0))?;
        let selected_bg = pnte::SolidColorBrush::new(&ctx, (0.0, 0.2, 0.0, 1.0))?;
        let button_bg = pnte::SolidColorBrush::new(&ctx, (0.15, 0.15, 0.15, 1.0))?;
        let button_bg_hover = pnte::SolidColorBrush::new(&ctx, (0.2, 0.2, 0.2, 1.0))?;
        let button_bg_pressed = pnte::SolidColorBrush::new(&ctx, (0.7, 0.7, 0.7, 1.0))?;
        let scroll_bar_bg = pnte::SolidColorBrush::new(&ctx, (0.2, 0.2, 0.2, 1.0))?;
        let scroll_bar_thumb = pnte::SolidColorBrush::new(&ctx, (0.9, 0.9, 0.9, 1.0))?;
        Ok(Self {
            ctx,
            render_target,
            text_format,
            list_box_bg,
            border_color,
            text_color,
            selected_bg,
            button_bg,
            button_bg_hover,
            button_bg_pressed,
            scroll_bar_bg,
            scroll_bar_thumb,
        })
    }

    fn draw_element(&self, cmd: &pnte::DrawCommand<pnte::Direct2D>, l: &glane::LayoutElement) {
        match l {
            glane::LayoutElement::Area(area) => {
                if area.selected {
                    let rect = pnte::Rect::new(
                        area.rect.left,
                        area.rect.top,
                        area.rect.right,
                        area.rect.bottom,
                    );
                    cmd.fill(&rect, &self.selected_bg);
                    return;
                }
                if l.handle().type_id() == TypeId::of::<Button>() {
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
                } else if l.handle().type_id() == TypeId::of::<ScrollBar>() {
                    let rect = l.rect();
                    cmd.fill(
                        &pnte::Rect::new(rect.left, rect.top, rect.right, rect.bottom),
                        &self.scroll_bar_bg,
                    );
                } else if l.handle().type_id() == TypeId::of::<scroll_bar::Thumb>() {
                    let rect = l.rect();
                    cmd.fill(
                        &pnte::Rect::new(rect.left, rect.top, rect.right, rect.bottom),
                        &self.scroll_bar_thumb,
                    );
                } else if l.handle().type_id() == TypeId::of::<ListBox>() {
                    let rect = area.rect;
                    cmd.fill(
                        &pnte::Rect::new(rect.left, rect.top, rect.right, rect.bottom),
                        &self.list_box_bg,
                    );
                    cmd.stroke(
                        &pnte::Rect::new(rect.left, rect.top, rect.right, rect.bottom),
                        &self.border_color,
                        2.0,
                        None,
                    );
                } else {
                    let rect = l.rect();
                    cmd.stroke(
                        &pnte::Rect::new(rect.left, rect.top, rect.right, rect.bottom),
                        &self.border_color,
                        2.0,
                        None,
                    );
                }
            }
            glane::LayoutElement::Text(t) => {
                let text = pnte::TextLayout::new(&self.ctx)
                    .text(&t.string)
                    .format(&self.text_format)
                    .build()
                    .unwrap();
                cmd.draw_text(&text, (l.rect().left, l.rect().top), &self.text_color)
                    .unwrap();
            }
            glane::LayoutElement::CompositionText(t) => {
                let text = pnte::TextLayout::new(&self.ctx)
                    .text(&t.string)
                    .format(&self.text_format)
                    .build()
                    .unwrap();
                cmd.draw_text(&text, (l.rect().left, l.rect().top), &self.text_color)
                    .unwrap();
                let width = if t.targeted { 2.0 } else { 1.0 };
                cmd.stroke(
                    &pnte::Line::new(
                        (l.rect().left + 1.0, l.rect().bottom),
                        (l.rect().right - 1.0, l.rect().bottom),
                    ),
                    &self.text_color,
                    width,
                    None,
                );
            }
            glane::LayoutElement::Cursor(_) => {
                let rect = l.rect();
                cmd.fill(
                    &pnte::Rect::new(rect.left, rect.top, rect.left + 2.0, rect.bottom),
                    &self.text_color,
                );
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
            cmd.clear((0.0, 0.0, 0.3, 0.0));
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
        .title("glane gallery")
        .build()?;
    let (mut scene, root) = {
        let mut root = Column::new();
        root.max_height = Some(200.0);
        glane::Scene::new(root)
    };
    let list = scene.push_child(&root, ListBox::new());
    let add_row = scene.push_child(&root, Row::new());
    let add_text = scene.push_child(&add_row, TextBox::new());
    let add_button = scene.push_child(&add_row, Button::new("add"));
    let erase_button = scene.push_child(&root, Button::new("erase"));
    let mut text = None;
    let mut canvas = Canvas::new(&window, &scene)?;
    let redrawing = Rc::new(Cell::new(false));
    let redraw = |window: &wiard::Window| {
        if !redrawing.get() {
            window.redraw(None);
            redrawing.set(true);
        }
    };
    let mut events = glane::Events::new();
    loop {
        let Some((event, _)) = event_rx.recv() else {
            break;
        };
        events.clear();
        match event {
            wiard::Event::MouseInput(m) => {
                let Some(dpi) = window.dpi() else {
                    continue;
                };
                let input = mouse_input(&m, dpi as f32);
                scene.input(glane::Input::MouseInput(input), &mut events);
                redraw(&window);
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
                    redraw(&window);
                }
            }
            wiard::Event::MouseWheel(m) => {
                let Some(dpi) = window.dpi() else {
                    continue;
                };
                scene.input(
                    glane::Input::MouseWheel(mouse_wheel(&m, dpi as f32)),
                    &mut events,
                );
                redraw(&window);
            }
            wiard::Event::KeyInput(ev) => {
                scene.input(
                    glane::Input::KeyInput(glane::KeyInput {
                        vkey: ev.key_code.vkey,
                        key_state: ev.key_state,
                    }),
                    &mut events,
                );
                redraw(&window);
            }
            wiard::Event::CharInput(ev) => {
                scene.input(glane::Input::CharInput(ev.c), &mut events);
                redraw(&window);
            }
            wiard::Event::ImeBeginComposition(ev) => {
                scene.input(glane::Input::ImeBeginComposition, &mut events);
                if let Some(event) = events.iter().find_map(|event| event.message(&add_text)) {
                    if let text_box::Message::PositionNotify(position) = event {
                        ev.set_position(wiard::LogicalPosition::new(
                            position.x as i32,
                            position.y as i32,
                        ));
                    }
                }
                redraw(&window);
            }
            wiard::Event::ImeUpdateComposition(ev) => {
                scene.input(
                    glane::Input::ImeUpdateComposition(glane::Composition {
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
                    }),
                    &mut events,
                );
                redraw(&window);
            }
            wiard::Event::ImeEndComposition(ev) => {
                scene.input(glane::Input::ImeEndComposition(ev.result), &mut events);
                redraw(&window);
            }
            wiard::Event::Draw(_) => {
                let layout = scene.layout();
                canvas.draw(&layout)?;
                redrawing.set(false);
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
                redraw(&window);
            }
            _ => {}
        }
        for event in events.iter() {
            if let Some(msg) = event.message(&add_button) {
                if *msg == button::Message::Clicked {
                    if let Some(text) = text.take() {
                        scene.push_child(&list, Text::new(&text));
                        scene.apply(&add_text, |t| t.clear());
                    }
                }
            } else if let Some(msg) = event.message(&erase_button) {
                if *msg == button::Message::Clicked {
                    scene.apply(&list, |l| l.erase_selected());
                }
            } else if let Some(msg) = event.message(&add_text) {
                if let text_box::Message::Changed(s) = msg {
                    text = Some(s.clone());
                }
            }
        }
    }
    Ok(())
}
