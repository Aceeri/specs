use std;
use tuple_utils::Split;
use bitset::{BitIter, BitSetAnd, BitSetLike};
use Index;


/// BitAnd is a helper method to & bitsets togather resulting in a tree
pub trait BitAnd {
    type Value: BitSetLike;
    fn and(self) -> Self::Value;
}

/// This needs to be special cased
impl<A> BitAnd for (A,)
    where A: BitSetLike
{
    type Value = A;
    fn and(self) -> Self::Value {
        self.0
    }
}

impl<A> BitAnd for A
    where A: BitSetLike
{
    type Value = A;
    fn and(self) -> Self::Value {
        self
    }
}

macro_rules! bitset_and {
    // use variables to indicate the arity of the tuple
    ($($from:ident),*) => {
        impl<$($from),*> BitAnd for ($($from),*)
            where $($from: BitSetLike),*
        {
            type Value = BitSetAnd<
                <<Self as Split>::Left as BitAnd>::Value,
                <<Self as Split>::Right as BitAnd>::Value
            >;
            fn and(self) -> Self::Value {
                let (l, r) = self.split();
                BitSetAnd(l.and(), r.and())
            }
        }
    }
}

bitset_and!{A, B}
bitset_and!{A, B, C}
bitset_and!{A, B, C, D}
bitset_and!{A, B, C, D, E}
bitset_and!{A, B, C, D, E, F}
bitset_and!{A, B, C, D, E, F, G}
bitset_and!{A, B, C, D, E, F, G, H}
bitset_and!{A, B, C, D, E, F, G, H, I}
bitset_and!{A, B, C, D, E, F, G, H, I, J}
bitset_and!{A, B, C, D, E, F, G, H, I, J, K}
bitset_and!{A, B, C, D, E, F, G, H, I, J, K, L}
bitset_and!{A, B, C, D, E, F, G, H, I, J, K, L, M}
bitset_and!{A, B, C, D, E, F, G, H, I, J, K, L, M, N}
bitset_and!{A, B, C, D, E, F, G, H, I, J, K, L, M, N, O}
bitset_and!{A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P}


/// The purpose of the `Join` trait is to provide a way
/// to access multiple storages at the same time with
/// the merged bit set.
pub trait Join {
    /// Type of joined components.
    type Type;
    /// Type of joined storages.
    type Value;
    /// Type of joined bit mask.
    type Mask: BitSetLike;
    /// Create a joined iterator over the contents.
    fn iter(self) -> JoinIter<Self> where Self: Sized {
        JoinIter::new(self)
    }
    /// Open this join by returning the mask and the storages.
    fn open(self) -> (Self::Mask, Self::Value);
    /// Get a joined component value by a gien index.
    unsafe fn get(&mut Self::Value, Index) -> Self::Type;
}


/// `JoinIter` is an Iterator over a group of `Storages`.
#[must_use]
pub struct JoinIter<J: Join> {
    keys: BitIter<J::Mask>,
    values: J::Value,
}

impl<J: Join> JoinIter<J> {
    /// Create a new join iterator.
    pub fn new(j: J) -> Self {
        let (keys, values) = j.open();
        JoinIter {
            keys: keys.iter(),
            values: values,
        }
    }
}

impl<J: Join> std::iter::Iterator for JoinIter<J> {
    type Item = J::Type;
    fn next(&mut self) -> Option<J::Type> {
        self.keys.next().map(|idx| unsafe {
            J::get(&mut self.values, idx)
        })
    }
}

/// Implementators of `MaskClone` can join bitsets using `Join`
/// without returning the contents.
pub trait MaskClone {
    /// Type of a cloned bit mask (should not be a reference to the original).
    type MaskClone: BitSetLike;
    /// Create a join that does not return the contents.
    fn check(&self) -> CheckJoin<Self::MaskClone> where Self: Sized + MaskClone {
        CheckJoin {
            mask: self.mask_clone(),
        }
    }
    /// Get a cloned bit mask.
    fn mask_clone(&self) -> Self::MaskClone;
}

/// Holder for bitmask of joins
pub struct CheckJoin<M: BitSetLike> {
    mask: M,
}

impl<M: BitSetLike> Join for CheckJoin<M> {
    type Type = ();
    type Value = ();
    type Mask = M;
    fn open(self) -> (Self::Mask, Self::Value) {
        (self.mask, ())
    }
    unsafe fn get(_: &mut Self::Value, _: Index) -> Self::Type {
        ()
    }
}

impl<'a, M: BitSetLike> Join for &'a CheckJoin<M> {
    type Type = ();
    type Value = ();
    type Mask = &'a M;
    fn open(self) -> (Self::Mask, Self::Value) {
        (&self.mask, ())
    }
    unsafe fn get(_: &mut Self::Value, _: Index) -> Self::Type {
        ()
    }
}

macro_rules! define_open {
    // use variables to indicate the arity of the tuple
    ($($from:ident : $position:tt ),*) => {
        impl<'a, $($from,)*> Join for ($($from),*,)
            where $($from: Join),*,
                  ($(<$from as Join>::Mask,)*): BitAnd,
        {
            type Type = ($($from::Type),*,);
            type Value = ($($from::Value),*,);
            type Mask = <($($from::Mask,)*) as BitAnd>::Value;
            #[allow(non_snake_case)]
            fn open(self) -> (Self::Mask, Self::Value) {
                let ($($from,)*) = self;
                let ($($from,)*) = ($($from.open(),)*);
                (
                    ($($from.0),*,).and(),
                    ($($from.1),*,)
                )
            }
            #[allow(non_snake_case)]
            unsafe fn get(v: &mut Self::Value, i: Index) -> Self::Type {
                let &mut ($(ref mut $from,)*) = v;
                ($($from::get($from, i),)*)
            }
        }

        impl<'a, $($from,)*> MaskClone for ($($from),*,)
            where $($from: MaskClone),*
        {
            type MaskClone = <($($from::MaskClone,)*) as BitAnd>::Value;
            #[allow(non_snake_case)]
            fn mask_clone(&self) -> Self::MaskClone {
                $(
                let $from = self.$position.mask_clone();
                )*
                ($($from),*,).and()
            }
        }
    }
}

define_open!{A:0}
define_open!{A:0, B:1}
define_open!{A:0, B:1, C:2}
define_open!{A:0, B:1, C:2, D:3}
define_open!{A:0, B:1, C:2, D:3, E:4}
define_open!{A:0, B:1, C:2, D:3, E:4, F:5}
define_open!{A:0, B:1, C:2, D:3, E:4, F:5, G:6}
define_open!{A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7}
define_open!{A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8}
define_open!{A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9}
define_open!{A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10}
define_open!{A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11}
define_open!{A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12}
define_open!{A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, N:13}
define_open!{A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, N:13, O:14}
define_open!{A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, N:13, O:14, P:15}
