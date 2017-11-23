
use std::marker::PhantomData;
use hlist::{Cons, Find, Nil};

use Index;

/// Observes interactions with the component's storage and stores state alongside it.
///
/// Useful for things like tracking modifications to components, storing sorted lists related
/// to the storage, etc.
pub trait Metadata<T>: Default {
    /// Forwards `clean` from the component storage.
    fn clean<F>(&mut self, _: &F)
    where
        F: Fn(Index) -> bool { }
    /// Forwards `get` from the component storage.
    fn get(&self, _: Index, _: &T) { }
    /// Forwards `get_mut` from the component storage.
    fn get_mut(&mut self, _: Index, _: &mut T) { }
    /// Forwards `insert` from the component storage.
    fn insert(&mut self, _: Index, _: &T) { }
    /// Forwards `remove` from the component storage.
    fn remove(&mut self, _: Index, _: &T) { }


impl<T> Metadata<T> for () { }
impl<T> Metadata<T> for PhantomData<T> { }
impl<T> Metadata<T> for Nil { }

impl<A, B, T> Metadata<T> for Cons<A, B>
where
    A: Metadata<T>,
    B: Metadata<T>,
{
    fn clean<F>(&mut self, f: &F)
    where
        F: Fn(Index) -> bool
    {
        self.0.clean(f);     
        self.1.clean(f);
    }
    fn get(&self, id: Index, value: &T) {
        self.0.get(id, value);     
        self.1.get(id, value);     
    }
    fn get_mut(&mut self, id: Index, value: &mut T) {
        self.0.get_mut(id, value);     
        self.1.get_mut(id, value);     
    }
    fn insert(&mut self, id: Index, value: &T) {
        self.0.insert(id, value);     
        self.1.insert(id, value);     
    }
    fn remove(&mut self, id: Index, value: &T) {
        self.0.remove(id, value);     
        self.1.remove(id, value);     
    }
}

#[macro_export]
macro_rules! metadata {
    () => ();
    ( $( $metadata:ty ),* ) => {
        metadata! [ @construct [ $( $metadata, )* ] $crate::Nil ]
    };
    ( @construct [ $current:ty, $( $leftover:ty, )* ] $constructed:ty ) => {
        metadata! [ @construct [ $( $leftover, )* ] $crate::Cons<$current, $constructed> ]
    };
    ( @construct [ ] $constructed:ty ) => { $constructed };
}
