use super::*;
use unicode_normalization::UnicodeNormalization;

#[derive(Debug)]
pub struct Style {
    pub font: Option<Font>,
    pub width: Option<std::num::NonZeroU32>,
    pub padding: LogicalRect<f32>,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            font: None,
            width: None,
            padding: LogicalRect::new(5.0, 3.0, 5.0, 3.0),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    Changed(String),
    PositionNotify(LogicalPosition<f32>),
}

pub struct TextBox {
    id: Id,
    widget_state: WidgetState,
    style: Style,
    front_text: Vec<char>,
    back_text: Vec<char>,
    composition: Option<Composition>,
}

impl TextBox {
    #[inline]
    pub fn new() -> Self {
        Self {
            id: Id::new(),
            widget_state: WidgetState::None,
            style: Style::default(),
            front_text: vec![],
            back_text: vec![],
            composition: None,
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.front_text.clear();
        self.back_text.clear();
    }
}

impl HasId for TextBox {
    fn id(&self) -> Id {
        self.id
    }
}

impl Widget for TextBox {
    fn input(&mut self, ctx: &Context, input: &Input, events: &mut Events) -> ControlFlow {
        match input {
            Input::MouseInput(m) => {
                if let Some(l) = ctx.layout.iter().find(|l| l.handle().is(self)) {
                    if l.rect().contains(&m.mouse_state.position) {
                        events.push(self, SetFocus);
                    }
                }
            }
            Input::KeyInput(k) => {
                if ctx.has_focus(self) && k.key_state == KeyState::Pressed {
                    match k.vkey {
                        VirtualKey::Left => {
                            if let Some(c) = self.front_text.pop() {
                                self.back_text.push(c);
                            }
                        }
                        VirtualKey::Right => {
                            if let Some(c) = self.back_text.pop() {
                                self.front_text.push(c);
                            }
                        }
                        _ => {}
                    }
                }
            }
            Input::CharInput(c) => {
                if ctx.has_focus(self) {
                    match c {
                        '\x08' => {
                            self.front_text.pop();
                        }
                        _ if c.is_control() => {
                            return ControlFlow::Continue;
                        }
                        _ => {
                            self.front_text.extend(c.nfc());
                        }
                    }
                    let s = self
                        .front_text
                        .iter()
                        .chain(self.back_text.iter())
                        .collect::<String>();
                    events.push(self, Message::Changed(s));
                }
            }
            Input::ImeBeginComposition => {
                let cursor = ctx
                    .find_layout(self)
                    .find(|layout| matches!(&**layout, LayoutElement::Cursor(_)));
                if let Some(l) = cursor {
                    events.push(self, Message::PositionNotify(l.rect().left_bottom()));
                }
            }
            Input::ImeUpdateComposition(composition) => {
                self.composition = Some(composition.clone());
            }
            Input::ImeEndComposition(result) => {
                if ctx.has_focus(self) {
                    if let Some(result) = result {
                        self.front_text
                            .append(&mut result.chars().collect::<Vec<_>>());
                        let s = self
                            .front_text
                            .iter()
                            .chain(self.back_text.iter())
                            .collect::<String>();
                        events.push(self, Message::Changed(s));
                    }
                }
                self.composition = None;
            }
            _ => {}
        }
        ControlFlow::Continue
    }

    fn apply(&mut self, funcs: &mut ApplyFuncs) {
        funcs.apply(self);
    }

    fn size(&self, ctx: &LayoutContext) -> LogicalSize<f32> {
        let font = self
            .style
            .font
            .as_ref()
            .unwrap_or_else(|| ctx.ctx.default_font.as_ref().unwrap());
        let text = self
            .front_text
            .iter()
            .chain(
                self.composition
                    .iter()
                    .map(|comp| comp.chars.iter())
                    .flatten(),
            )
            .chain(self.back_text.iter())
            .collect::<String>();
        let rect = bounding_box_with_str(&font, &text);
        let width = if let Some(ref width) = self.style.width {
            width.get()
        } else {
            5
        };
        let size = {
            let size = font.global_bounding_size();
            let size = LogicalSize::new(size.width * width as f32, size.height);
            LogicalSize::new(
                if rect.right <= size.width {
                    size.width
                } else {
                    rect.right
                },
                rect.bottom,
            )
        };
        LogicalSize::new(
            size.width + self.style.padding.left + self.style.padding.right,
            size.height + self.style.padding.top + self.style.padding.bottom,
        )
    }

    fn layout(&self, lc: LayoutContext, result: &mut LayoutConstructor) {
        let size = self.size(&lc);
        let font = self
            .style
            .font
            .as_ref()
            .unwrap_or_else(|| lc.ctx.default_font.as_ref().unwrap());
        let mut rect = LogicalRect::from_position_size(lc.rect.left_top(), size);
        result.push(&lc, LayoutElement::area(self, self.widget_state, rect, false));
        rect.left += self.style.padding.left;
        rect.top += self.style.padding.top;
        rect.right -= self.style.padding.right;
        rect.bottom -= self.style.padding.bottom;
        let front_text = self.front_text.iter().collect::<String>();
        let front_rect = bounding_box_with_str(&font, &front_text);
        rect = LogicalRect::from_position_size(
            LogicalPosition::new(rect.left, rect.top),
            LogicalSize::new(
                front_rect.right - front_rect.left,
                front_rect.bottom - front_rect.top,
            ),
        );
        if !self.front_text.is_empty() {
            result.push(&lc, LayoutElement::text(
                self,
                self.widget_state,
                rect,
                front_text,
                false,
            ));
        }
        if lc.ctx.has_focus(self) {
            if let Some(ref composition) = self.composition {
                for clause in composition.clauses.iter() {
                    let text = composition.chars[clause.range.start..clause.range.end]
                        .iter()
                        .collect::<String>();
                    let text_rect = bounding_box_with_str(&font, &text);
                    rect = LogicalRect::from_position_size(
                        LogicalPosition::new(rect.right, rect.top),
                        LogicalSize::new(
                            text_rect.right - text_rect.left,
                            text_rect.bottom - text_rect.top,
                        ),
                    );
                    result.push(&lc, LayoutElement::composition_text(
                        self,
                        self.widget_state,
                        rect,
                        text,
                        clause.targeted,
                    ));
                }
                let text_rect = if let Some(clause) =
                    composition.clauses.iter().find(|clause| clause.targeted)
                {
                    let text = composition.chars[..clause.range.end]
                        .iter()
                        .collect::<String>();
                    bounding_box_with_str(&font, &text)
                } else if composition.cursor_position == 0 {
                    LogicalRect::new(0.0, rect.top, 0.0, rect.bottom)
                } else {
                    let text = composition.chars[..composition.cursor_position]
                        .iter()
                        .collect::<String>();
                    bounding_box_with_str(&font, &text)
                };
                let cursor_rect = LogicalRect::from_position_size(
                    LogicalPosition::new(
                        lc.rect.left + self.style.padding.left + front_rect.right + text_rect.right,
                        rect.top,
                    ),
                    LogicalSize::new(2.0, rect.bottom - rect.top),
                );
                result.push(&lc, LayoutElement::cursor(self, self.widget_state, cursor_rect));
            } else {
                let cursor_rect = LogicalRect::from_position_size(
                    LogicalPosition::new(rect.right, rect.top),
                    LogicalSize::new(2.0, rect.bottom - rect.top),
                );
                result.push(&lc, LayoutElement::cursor(self, self.widget_state, cursor_rect));
            }
        }
        if !self.back_text.is_empty() {
            let back_text = self.back_text.iter().collect::<String>();
            let back_rect = bounding_box_with_str(&font, &back_text);
            rect = LogicalRect::from_position_size(
                LogicalPosition::new(rect.right, rect.top),
                LogicalSize::new(
                    back_rect.right - back_rect.left,
                    back_rect.bottom - back_rect.top,
                ),
            );
            result.push(&lc, LayoutElement::text(
                self,
                self.widget_state,
                rect,
                back_text,
                false,
            ));
        }
    }
}

impl WidgetMessage for TextBox {
    type Message = Message;
}
