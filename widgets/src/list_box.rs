use super::*;
use std::cell::{Cell, RefCell};

#[derive(Debug)]
pub struct Style {
    pub padding: LogicalRect<f32>,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            padding: LogicalRect::new(5.0, 2.0, 5.0, 2.0),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Message {
    Selected(usize),
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct ListBox {
    id: Id,
    style: Style,
    children: Vec<Child>,
    vscroll: RefCell<VScrollBar>,
    first_view_element: Cell<usize>,
    selected: Option<usize>,
    widget_state: WidgetState,
    min_height: Cell<f32>,
}

impl ListBox {
    #[inline]
    pub fn new() -> Self {
        Self {
            id: Id::new(),
            style: Default::default(),
            children: vec![],
            vscroll: RefCell::new(VScrollBar::new(0, 0)),
            first_view_element: Cell::new(0),
            selected: None,
            widget_state: WidgetState::None,
            min_height: Cell::new(f32::MAX),
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        let mut vscroll = self.vscroll.borrow_mut();
        vscroll.len = 0;
        vscroll.thumb.len = 0;
        self.first_view_element.set(0);
        self.selected = None;
        self.children.clear();
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

    #[inline]
    pub fn child(&self, index: usize) -> Option<&dyn Widget> {
        (index < self.children.len()).then(|| self.children[index].object.as_ref())
    }

    #[inline]
    pub fn selected_child(&self) -> Option<&dyn Widget> {
        self.selected.map(|i| self.children[i].object.as_ref())
    }

    #[inline]
    pub fn erase_selected(&mut self) {
        if let Some(index) = self.selected {
            self.erase(&self.children[index].object.id())
        }
    }
}

impl HasId for ListBox {
    fn id(&self) -> Id {
        self.id
    }
}

impl Widget for ListBox {
    fn input(&mut self, ctx: &Context, input: &Input, events: &mut Events) -> ControlFlow {
        let mut layout = ctx.find_layout(self);
        let Some(area) = layout.find(|l| matches!(&**l, LayoutElement::StartClipping(_))) else {
            return ControlFlow::Continue;
        };
        let Some(bar) = ctx.find_layout(&*self.vscroll.borrow()).next() else {
            return ControlFlow::Continue;
        };
        match input {
            Input::MouseInput(m) => {
                let cond = m.button == MouseButton::Left
                    && m.button_state == ButtonState::Pressed
                    && area.rect().is_crossing(&m.mouse_state.position);
                if cond {
                    events.push(self, SetFocus);
                    let mut i = self.first_view_element.get();
                    let mut height = 0.0;
                    while i < self.children.len() && height < area.rect().size().height {
                        let child = &self.children[i];
                        let child_rect = child.rect.get().map(|r| {
                            LogicalRect::new(
                                r.left,
                                r.top,
                                r.right - bar.rect().size().width,
                                r.bottom,
                            )
                        });
                        if child_rect.map_or(false, |r| r.contains(&m.mouse_state.position)) {
                            events.push_message(self, Message::Selected(i));
                            self.selected = Some(i);
                            break;
                        }
                        i += 1;
                        height += child_rect.map_or(0.0, |r| r.size().height);
                    }
                }
            }
            Input::CursorMoved(m) => {
                if area.rect().is_crossing(&m.mouse_state.position) {
                    if m.mouse_state.buttons.contains(MouseButton::Left) {
                        self.widget_state = events.push_state_changed(
                            self,
                            WidgetState::Pressed,
                            self.widget_state,
                        );
                    } else {
                        self.widget_state =
                            events.push_state_changed(self, WidgetState::Hover, self.widget_state);
                    }
                } else {
                    self.widget_state =
                        events.push_state_changed(self, WidgetState::None, self.widget_state);
                }
            }
            Input::MouseWheel(m) => {
                if area.rect().is_crossing(&m.mouse_state.position)
                    && m.axis == MouseWheelAxis::Vertical
                {
                    let mut vbar = self.vscroll.borrow_mut();
                    vbar.advance(self.min_height.get() as isize * m.distance as isize);
                }
            }
            _ => {}
        }
        self.vscroll.borrow_mut().input(ctx, input, events)
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
        result.push(
            &lc,
            LayoutElement::start_clipping(self, lc.rect, &lc.ancestors, lc.layer),
        );
        result.push(
            &lc,
            LayoutElement::area(
                self,
                self.widget_state,
                lc.rect,
                &lc.ancestors,
                lc.layer,
                false,
            ),
        );
        let current = self.vscroll.borrow().current() as f32;
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
        self.min_height.set(f32::MAX);
        for (i, child) in self.children.iter().enumerate() {
            let size = child.object.size(&lc);
            self.min_height.set(self.min_height.get().min(size.height));
            rect = LogicalRect::from_position_size(
                rect.left_top(),
                (padding_rect.size().width, size.height),
            );
            if rect.is_crossing(&viewport) {
                let selected = self.selected.map_or(false, |selected| selected == i);
                if selected {
                    result.push(
                        &lc,
                        LayoutElement::area(
                            self,
                            WidgetState::None,
                            rect,
                            &lc.ancestors,
                            lc.layer,
                            true,
                        ),
                    );
                }
                if first_view_element.is_none() {
                    first_view_element = Some(i);
                }
                child
                    .object
                    .layout(lc.next(self, rect, lc.layer, selected), result);
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
            let mut bar = self.vscroll.borrow_mut();
            bar.thumb.len = (thumb_height.ceil()) as usize;
            bar.len = total_size.height.ceil() as usize;
        }
        {
            let vscroll = self.vscroll.borrow();
            let size = vscroll.size(&lc);
            vscroll.layout(
                lc.next(
                    self,
                    LogicalRect::new(
                        lc.rect.right - size.width - self.style.padding.right,
                        lc.rect.top + self.style.padding.top,
                        lc.rect.right - self.style.padding.right,
                        lc.rect.bottom - self.style.padding.bottom,
                    ),
                    lc.layer,
                    lc.selected,
                ),
                result,
            );
        }
        result.push(
            &lc,
            LayoutElement::end_clipping(self, lc.rect, &lc.ancestors, lc.layer),
        );
    }
}

impl WidgetMessage for ListBox {
    type Message = Message;
}

impl HasChildren for ListBox {
    #[inline]
    fn len(&self) -> usize {
        self.children.len()
    }

    #[inline]
    fn push(&mut self, child: impl Widget) {
        self.children.push(Child::new(child));
    }

    #[inline]
    fn erase(&mut self, child: &impl HasId) {
        let Some(index) = self
            .children
            .iter()
            .position(|c| c.object.id() == child.id())
        else {
            return;
        };
        self.children.remove(index);
        if let Some(selected) = self.selected {
            if self.children.is_empty() {
                self.selected = None;
            } else if selected == index {
                if index == 0 {
                    self.selected = Some(0);
                } else {
                    self.selected = Some(selected - 1);
                }
            }
        }
    }
}

impl Default for ListBox {
    fn default() -> Self {
        Self::new()
    }
}
