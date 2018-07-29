mod green;
mod red;
mod syntax;
mod builder;

use std::{
    sync::{Arc, Weak},
    mem
};
pub(crate) use self::{
    green::{GreenNode, GreenNodeBuilder},
    red::RedNode,
    syntax::SyntaxError,
    builder::GreenBuilder,
};
pub use self::syntax::SyntaxNode;

// This could be just `*const T`, but we use `Weak` for additional checks
#[derive(Debug)]
pub(crate) struct Ptr<T>(Weak<T>);

impl<T> Clone for Ptr<T> {
    fn clone(&self) -> Self {
        Ptr(self.0.clone())
    }
}

impl<T> Ptr<T> {
    fn clone(self_: &Ptr<T>) -> Ptr<T> {
        Ptr(Weak::clone(&self_.0))
    }

    fn new(arc: &Arc<T>) -> Ptr<T> {
        Ptr(Arc::downgrade(arc))
    }

    unsafe fn get(&self) -> &T {
        let t = self.0.upgrade()
            .expect("caller must guarantee that Ptr is not null");
        let t: &T = &*t;
        mem::transmute(t)
    }
}
