use super::*;
use std::any::Any;

#[derive(Clone, Copy, Debug)]
pub struct StateChanged {
    pub current: WidgetState,
    pub prev: WidgetState,
}

impl StateChanged {
    #[inline]
    pub fn new(current: WidgetState, prev: WidgetState) -> Self {
        Self { current, prev }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SetFocus;

pub struct Event {
    handle: AnyHandle,
    object: Box<dyn Any>,
}

impl Event {
    #[inline]
    pub fn new<T, U>(widget: &T, object: U) -> Self
    where
        T: Widget,
        U: Any,
    {
        Self {
            handle: AnyHandle::new(widget),
            object: Box::new(object),
        }
    }

    #[inline]
    pub fn handle(&self) -> AnyHandle {
        self.handle
    }

    #[inline]
    pub fn message<T>(&self, handle: &Handle<T>) -> Option<&T::Message>
    where
        T: WidgetMessage,
    {
        if handle != &self.handle {
            return None;
        }
        self.object.downcast_ref::<T::Message>()
    }

    #[inline]
    pub fn state_changed<T>(&self, handle: &Handle<T>) -> Option<&StateChanged>
    where
        T: Widget,
    {
        if handle != &self.handle {
            return None;
        }
        self.object.downcast_ref::<StateChanged>()
    }

    #[inline]
    pub fn is_set_focus(&self) -> bool {
        self.object.downcast_ref::<SetFocus>().is_some()
    }

    #[inline]
    pub fn downcast_ref<T>(&self) -> Option<&T>
    where
        T: Any,
    {
        self.object.downcast_ref::<T>()
    }
}

#[inline]
pub fn state_changed_exists(v: &Vec<Event>) -> bool {
    v.iter()
        .find_map(|ev| ev.downcast_ref::<StateChanged>())
        .is_some()
}

pub struct Events(Vec<Event>);

impl Events {
    #[inline]
    pub fn new() -> Self {
        Self(vec![])
    }

    #[inline]
    pub fn clear(&mut self) {
        self.0.clear();
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn push(&mut self, widget: &impl Widget, object: impl Any) {
        self.0.push(Event::new(widget, object))
    }

    #[inline]
    pub fn push_message<T>(&mut self, widget: &T, message: T::Message)
    where
        T: WidgetMessage,
    {
        self.0.push(Event::new(widget, message));
    }

    #[inline]
    pub fn push_state_changed(
        &mut self,
        widget: &impl Widget,
        current: WidgetState,
        prev: WidgetState,
    ) -> WidgetState {
        if current != prev {
            self.0
                .push(Event::new(widget, StateChanged::new(current, prev)));
        }
        current
    }

    #[inline]
    pub fn pop(&mut self) -> Option<Event> {
        self.0.pop()
    }

    #[inline]
    pub fn remove(&mut self, index: usize) -> Event {
        self.0.remove(index)
    }

    #[inline]
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &Event> {
        self.0.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Event> {
        self.0.iter_mut()
    }
}

