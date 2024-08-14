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
    white: pnte::SolidColorBrush,
    button_bg: pnte::SolidColorBrush,
    button_bg_hover: pnte::SolidColorBrush,
    button_bg_pressed: pnte::SolidColorBrush,
    text_box_border: pnte::SolidColorBrush,
    scroll_bar_bg: pnte::SolidColorBrush,
    scroll_bar_thumb: pnte::SolidColorBrush,
    selected_bg: pnte::SolidColorBrush,
    list_box_bg: pnte::SolidColorBrush,
    slider_bg: pnte::SolidColorBrush,
    slider_knob: pnte::SolidColorBrush,
    button_type: TypeId,
    text_box_type: TypeId,
    scroll_bar_type: TypeId,
    scroll_bar_thumb_type: TypeId,
    list_box_type: TypeId,
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
        let scroll_bar_bg = pnte::SolidColorBrush::new(&ctx, (0.3, 0.3, 0.3, 1.0))?;
        let scroll_bar_thumb = pnte::SolidColorBrush::new(&ctx, (0.8, 0.8, 0.8, 1.0))?;
        let selected_bg = pnte::SolidColorBrush::new(&ctx, (0.0, 0.3, 0.0, 1.0))?;
        let list_box_bg = pnte::SolidColorBrush::new(&ctx, (0.1, 0.1, 0.1, 0.9))?;
        let button_type = TypeId::of::<glane::widgets::Button>();
        let slider_bg = pnte::SolidColorBrush::new(&ctx, (0.3, 0.3, 0.3, 1.0))?;
        let slider_knob = pnte::SolidColorBrush::new(&ctx, (0.8, 0.8, 0.8, 1.0))?;
        let text_box_type = TypeId::of::<glane::widgets::TextBox>();
        let scroll_bar_type = TypeId::of::<glane::widgets::ScrollBar>();
        let scroll_bar_thumb_type = TypeId::of::<glane::widgets::scroll_bar::Thumb>();
        let list_box_type = TypeId::of::<glane::widgets::ListBox>();
        Ok(Self {
            ctx,
            render_target,
            text_format,
            white,
            button_bg,
            button_bg_hover,
            button_bg_pressed,
            text_box_border,
            scroll_bar_bg,
            scroll_bar_thumb,
            selected_bg,
            list_box_bg,
            slider_bg,
            slider_knob,
            button_type,
            text_box_type,
            scroll_bar_type,
            scroll_bar_thumb_type,
            list_box_type,
        })
    }

    fn draw_element<T: pnte::Backend>(&self, cmd: &pnte::DrawCommand<T>, l: &glane::LayoutElement) {
        match l {
            glane::LayoutElement::Area(area) => {
                let rect = area.rect;
                let rect = pnte::Rect::new(rect.left, rect.top, rect.right, rect.bottom);
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
                match l.handle().type_id() {
                    t if t == self.button_type => {
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
                    t if t == self.scroll_bar_type => {
                        let rect = l.rect();
                        cmd.fill(
                            &pnte::Rect::new(rect.left, rect.top, rect.right, rect.bottom),
                            &self.scroll_bar_bg,
                        );
                    }
                    t if t == self.scroll_bar_thumb_type => {
                        let rect = l.rect();
                        cmd.fill(
                            &pnte::Rect::new(rect.left, rect.top, rect.right, rect.bottom),
                            &self.scroll_bar_thumb,
                        );
                    }
                    t if t == self.list_box_type => {
                        cmd.fill(
                            &pnte::Rect::new(rect.left, rect.top, rect.right, rect.bottom),
                            &self.list_box_bg,
                        );
                        cmd.stroke(
                            &pnte::Rect::new(rect.left, rect.top, rect.right, rect.bottom),
                            &self.text_box_border,
                            2.0,
                            None,
                        );
                    }
                    t if t == self.text_box_type => {
                        let rect = l.rect();
                        cmd.stroke(
                            &pnte::Rect::new(rect.left, rect.top, rect.right, rect.bottom),
                            &self.text_box_border,
                            2.0,
                            None,
                        );
                    }
                    t if t == TypeId::of::<glane::widgets::Slider>() => {
                        let rect = l.rect();
                        let size = rect.size();
                        cmd.fill(
                            &pnte::Rect::from_point_size(
                                (rect.left, rect.top + size.height / 4.0),
                                (size.width, size.height / 2.0),
                            ),
                            &self.slider_bg,
                        );
                    }
                    t if t == TypeId::of::<glane::widgets::slider::Knob>() => {
                        let rect = l.rect();
                        let size = rect.size();
                        cmd.fill(
                            &pnte::Circle::new(
                                (rect.left + size.width / 2.0, rect.top + size.height / 2.0),
                                size.width / 2.0,
                            ),
                            &self.slider_knob,
                        );
                    }
                    _ => {
                        let rect = l.rect();
                        cmd.stroke(
                            &pnte::Rect::new(rect.left, rect.top, rect.right, rect.bottom),
                            &self.text_box_border,
                            2.0,
                            None,
                        );
                    }
                }
            }
            glane::LayoutElement::Text(t) => {
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
            glane::LayoutElement::Cursor(_) => {
                let rect = l.rect();
                cmd.fill(
                    &pnte::Rect::new(rect.left, rect.top, rect.left + 2.0, rect.bottom),
                    &self.white,
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
    let ((mut scene, _), left, right) = {
        let (root, left, right) = glane::widgets::VerticalPanes::new(
            glane::widgets::Column::new(),
            glane::widgets::Column::new(),
            0.5,
        );
        (glane::Scene::new(root), left, right)
    };
    let row_button = scene.push_child(&left, glane::widgets::Row::new());
    scene.push_child(&row_button, glane::widgets::Label::new("Button"));
    let button = scene.push_child(&row_button, glane::widgets::Button::new("Push"));
    let row_text_box = scene.push_child(&left, glane::widgets::Row::new());
    scene.push_child(&row_text_box, glane::widgets::Label::new("TextBox"));
    let text_box = scene.push_child(&row_text_box, glane::widgets::TextBox::new());
    let row_scroll_bar = scene.push_child(&left, glane::widgets::Row::new());
    scene.push_child(&row_scroll_bar, glane::widgets::Label::new("ScrollBar"));
    let scroll_bar = {
        let scroll_bar = glane::widgets::ScrollBar::new(100, 10);
        let handle = glane::Handle::new(&scroll_bar);
        scene.push_child(
            &row_scroll_bar,
            glane::widgets::MaxSize::new(None, Some(200.0), scroll_bar),
        );
        handle
    };
    let scroll_bar2 = {
        let scroll_bar = glane::widgets::ScrollBar::new(1000, 10);
        let handle = glane::Handle::new(&scroll_bar);
        scene.push_child(
            &row_scroll_bar,
            glane::widgets::MaxSize::new(None, Some(200.0), scroll_bar),
        );
        handle
    };
    let row_slider = scene.push_child(&left, glane::widgets::Row::new());
    scene.push_child(&row_slider, glane::widgets::Label::new("Slider"));
    let slider = {
        let slider = glane::widgets::Slider::new();
        let handle = glane::Handle::new(&slider);
        scene.push_child(&row_slider, glane::widgets::MaxSize::new(Some(150.0), None, slider));
        handle
    };
    let row_dropdown_box = scene.push_child(&right, glane::widgets::Row::new());
    scene.push_child(&row_dropdown_box, glane::widgets::Label::new("DropdownBox"));
    let dropdown_box = scene.push_child(&row_dropdown_box, glane::widgets::DropdownBox::new());
    for c in 'A'..='C' {
        scene.push_child(&dropdown_box, glane::widgets::Text::new(c.to_string()));
    }
    let row_list_box = scene.push_child(&right, glane::widgets::Row::new());
    scene.push_child(&row_list_box, glane::widgets::Label::new("ListBox"));
    let list_box = {
        let list_box = glane::widgets::ListBox::new();
        let handle = glane::Handle::new(&list_box);
        scene.push_child(&row_list_box, glane::widgets::MaxSize::new(None, Some(200.0), list_box));
        handle
    };
    for c in 'a'..='z' {
        scene.push_child(&list_box, glane::widgets::Text::new(c.to_string()));
    }
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
            wiard::Event::MouseWheel(ev) => {
                let Some(dpi) = window.dpi() else {
                    continue;
                };
                scene.input(
                    glane::Input::MouseWheel(mouse_wheel(&ev, dpi as f32)),
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
                if let Some(event) = events.iter().find_map(|event| event.message(&text_box)) {
                    if let glane::widgets::text_box::Message::PositionNotify(position) = event {
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
            if let Some(msg) = event.message(&button) {
                if msg == &glane::widgets::button::Message::Clicked {
                    println!("button clicked");
                }
            } else if let Some(msg) = event.message(&text_box) {
                match msg {
                    glane::widgets::text_box::Message::Changed(s) => {
                        println!("text_box changed: {s}");
                    }
                    _ => {}
                }
            } else if let Some(msg) = event.message(&scroll_bar) {
                match msg {
                    glane::widgets::scroll_bar::Message::Changed(p) => {
                        println!("scrollbar changed: {p}-{}", p + 10);
                    }
                }
            } else if let Some(msg) = event.message(&scroll_bar2) {
                match msg {
                    glane::widgets::scroll_bar::Message::Changed(p) => {
                        println!("scrollbar2 changed: {p}-{}", p + 10);
                    }
                }
            } else if let Some(msg) = event.message(&list_box) {
                match msg {
                    glane::widgets::list_box::Message::Selected(i) => {
                        println!("list_box selected: {i}");
                    }
                }
            } else if let Some(msg) = event.message(&dropdown_box) {
                match msg {
                    glane::widgets::dropdown_box::Message::Selected(i) => {
                        println!("dropdown_box selected: {i}");
                    }
                    glane::widgets::dropdown_box::Message::OpenedList => {
                        println!("dropdown_box opened list");
                    }
                    glane::widgets::dropdown_box::Message::ClosedList => {
                        println!("dropdown_box closed list");
                    }
                }
            } else if let Some(msg) = event.message(&slider) {
                match msg {
                    glane::widgets::slider::Message::Changed(v) => {
                        println!("slider changed: {v}");
                    }
                }
            }
        }
    }
    Ok(())
}
