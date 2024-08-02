use super::*;
use std::cell::{Cell, RefCell};

pub struct Style {
    pub padding: LogicalRect<f32>,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            padding: LogicalRect::new(5.0, 2.0, 2.0, 2.0),
        }
    }
}

pub enum Message {
    Changed(usize),
}

struct Child {
    object: Box<dyn Widget>,
    rect: Cell<Option<LogicalRect<f32>>>,
}

impl Child {
    fn new(object: impl Widget) -> Self {
        Self {
            object: Box::new(object),
            rect: Cell::new(None),
        }
    }
}

pub struct ListBox {
    id: Id,
    style: Style,
    children: Vec<Child>,
    vertical_bar: RefCell<ScrollBar>,
    first_view_element: Cell<usize>,
    selected: Option<usize>,
}

impl ListBox {
    #[inline]
    pub fn new() -> Self {
        Self {
            id: Id::new(),
            style: Default::default(),
            children: vec![],
            vertical_bar: RefCell::new(ScrollBar::new(0, 0)),
            first_view_element: Cell::new(0),
            selected: None,
        }
    }

    #[inline]
    pub fn select(&mut self, index: Option<usize>) {
        let Some(index) = index else {
            self.selected = None;
            return;
        };
        assert!(index < self.children.len());
        self.selected = Some(index);
    }
}

impl HasId for ListBox {
    fn id(&self) -> Id {
        self.id
    }
}

impl Widget for ListBox {
    fn input(&mut self, ctx: &Context, input: &Input, events: &mut Vec<Event>) {
        let mut layout = ctx.find_layout(self);
        let Some(area) = layout.find(|l| matches!(l, LayoutElement::ClippedArea(_))) else {
            return;
        };
        let Some(bar) = ctx.find_layout(&*self.vertical_bar.borrow()).next() else {
            return;
        };
        match input {
            Input::MouseInput(m) => {
                if m.button == MouseButton::Left && m.button_state == ButtonState::Pressed {
                    let mut i = self.first_view_element.get();
                    let mut height = 0.0;
                    while i < self.children.len() && height < area.rect().size().height {
                        let element = &self.children[i];
                        let element_rect = element.rect.get().map(|r| {
                            LogicalRect::new(
                                r.left,
                                r.top,
                                r.right - bar.rect().size().width,
                                r.bottom,
                            )
                        });
                        if element_rect.map_or(false, |r| r.contains(&m.mouse_state.position)) {
                            self.selected = Some(i);
                            break;
                        }
                        i += 1;
                        height += element.rect.get().map_or(0.0, |r| r.size().height);
                    }
                }
            }
            _ => {}
        }
        self.vertical_bar.borrow_mut().input(ctx, input, events);
    }

    fn apply(&mut self, funcs: &mut ApplyFuncs) {
        funcs.apply(self);
        for child in self.children.iter_mut() {
            child.object.apply(funcs);
        }
    }

    fn size(&self, ctx: &LayoutContext) -> LogicalSize<f32> {
        ctx.rect.size()
    }

    fn layout(&self, lc: LayoutContext, result: &mut LayoutConstructor) {
        let mut layout = LayoutConstructor::new();
        let current = self.vertical_bar.borrow().current() as f32;
        let padding_rect = LogicalRect::new(
            lc.rect.left + self.style.padding.left,
            lc.rect.top + self.style.padding.top,
            lc.rect.right - self.style.padding.right,
            lc.rect.bottom - self.style.padding.bottom,
        );
        let viewport = padding_rect;
        let mut rect = LogicalRect::new(padding_rect.left, padding_rect.top - current, 0.0, 0.0);
        let mut total_size = LogicalSize::new(padding_rect.size().width, 0.0);
        let mut thumb_height = 0.0;
        let mut first_view_element = None;
        for (i, child) in self.children.iter().enumerate() {
            let size = child.object.size(&lc);
            rect = LogicalRect::from_position_size(
                rect.left_top(),
                (padding_rect.size().width, size.height),
            );
            if rect.is_crossing(&viewport) {
                if self.selected.map_or(false, |selected| selected == i) {
                    layout.push_back(LayoutElement::selected_area(self, WidgetState::None, rect));
                }
                if first_view_element.is_none() {
                    first_view_element = Some(i);
                }
                child.object.layout(lc.next(rect), &mut layout);
                let d = if padding_rect.top > rect.top {
                    padding_rect.top - rect.top
                } else if padding_rect.bottom < rect.bottom {
                    rect.bottom - padding_rect.bottom
                } else {
                    0.0
                };
                thumb_height += size.height - d;
            }
            child.rect.set(Some(rect));
            rect.top += size.height;
            rect.bottom += size.height;
            total_size.height += size.height;
        }
        self.first_view_element.set(first_view_element.unwrap_or(0));
        {
            let mut bar = self.vertical_bar.borrow_mut();
            bar.thumb.len = (thumb_height.ceil()) as usize;
            bar.len = total_size.height.ceil() as usize;
        }
        result.push_back(LayoutElement::clipped_area(
            self,
            WidgetState::None,
            lc.rect,
            lc.ctx,
            layout,
        ));
        {
            let bar = self.vertical_bar.borrow();
            let size = bar.size(&lc);
            bar.layout(
                lc.next(LogicalRect::new(
                    lc.rect.right - size.width - self.style.padding.right,
                    lc.rect.top + self.style.padding.top,
                    lc.rect.right - self.style.padding.right,
                    lc.rect.bottom - self.style.padding.bottom,
                )),
                result,
            );
        }
    }
}

impl WidgetMessage for ListBox {
    type Message = Message;
}

impl HasChildren for ListBox {
    fn len(&self) -> usize {
        self.children.len()
    }

    fn push(&mut self, child: impl Widget) {
        self.children.push(Child::new(child));
    }

    fn erase(&mut self, child: impl HasId) {
        let Some(index) = self
            .children
            .iter()
            .position(|c| c.object.id() == child.id())
        else {
            return;
        };
        if self.selected.map_or(false, |selected| selected == index) {
            self.selected = None;
        }
        self.children.remove(index);
    }
}
