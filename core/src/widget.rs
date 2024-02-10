use super::*;
use std::any::{Any, TypeId};

pub trait Widget: Any {
    fn id(&self) -> Id;
    fn input(&mut self, ctx: &Context, input: &Input, events: &mut Vec<Event>);
    fn apply(&mut self, funcs: &mut ApplyFuncs);
    fn layout(&self, ctx: LayoutContext, result: &mut LayoutConstructor);
}

pub trait WidgetMessage: Widget {
    type Message;
}

#[derive(Copy, PartialEq, Eq, Debug)]
pub struct Handle<T>
where
    T: Widget,
{
    id: Id,
    _t: std::marker::PhantomData<T>,
}

impl<T> Handle<T>
where
    T: Widget,
{
    #[inline]
    pub fn new(widget: &T) -> Self {
        Self {
            id: widget.id(),
            _t: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn id(&self) -> Id {
        self.id
    }
}

impl<T> Clone for Handle<T>
where
    T: Widget,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            _t: std::marker::PhantomData,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct AnyHandle {
    id: Id,
    t: TypeId,
}

impl AnyHandle {
    #[inline]
    pub fn new<T>(widget: &T) -> Self
    where
        T: Widget,
    {
        Self {
            id: widget.id(),
            t: TypeId::of::<T>(),
        }
    }

    #[inline]
    pub fn id(&self) -> Id {
        self.id
    }

    #[inline]
    pub fn type_id(&self) -> TypeId {
        self.t
    }

    #[inline]
    pub fn downcast<T>(&self) -> Option<Handle<T>>
    where
        T: Widget,
    {
        (self.t == TypeId::of::<T>()).then(|| Handle {
            id: self.id,
            _t: std::marker::PhantomData,
        })
    }
}

impl<T> PartialEq<Handle<T>> for AnyHandle
where
    T: Widget,
{
    #[inline]
    fn eq(&self, other: &Handle<T>) -> bool {
        self.id == other.id
    }
}

impl<T> PartialEq<AnyHandle> for Handle<T>
where
    T: Widget,
{
    #[inline]
    fn eq(&self, other: &AnyHandle) -> bool {
        self.id == other.id
    }
}

impl<T> From<Handle<T>> for AnyHandle
where
    T: Widget,
{
    #[inline]
    fn from(value: Handle<T>) -> Self {
        AnyHandle {
            id: value.id,
            t: TypeId::of::<T>(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WidgetState {
    None,
    Hover,
    Pressed,
}
