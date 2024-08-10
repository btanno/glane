use std::sync::atomic::{self, AtomicU64};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Id(u64);

impl Id {
    pub fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(1);
        Self(NEXT_ID.fetch_add(1, atomic::Ordering::SeqCst))
    }
}

pub trait HasId {
    fn id(&self) -> Id;
}

impl HasId for Id {
    #[inline]
    fn id(&self) -> Id {
        *self
    }
}
