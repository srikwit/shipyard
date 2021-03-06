use super::chunk::multiple::*;
use super::chunk_exact::multiple::*;
use super::{
    AbstractMut, CurrentId, DoubleEndedShiperator, ExactSizeShiperator, IntoAbstract, IntoIterator,
    Shiperator,
};
use crate::EntityId;
#[cfg(feature = "parallel")]
use rayon::iter::plumbing::Producer;

macro_rules! impl_iterators {
    (
        $number: literal
        $tight: ident
        $chunk: ident
        $chunk_exact: ident
        $(($type: ident, $index: tt))+
    ) => {
        #[doc = "Tight iterator over"]
        #[doc = $number]
        #[doc = "components.  
Tight iterators are fast but are limited to components tightly packed together."]
        pub struct $tight<$($type: IntoAbstract),+> {
            pub(crate) data: ($($type::AbsView,)+),
            pub(crate) current: usize,
            pub(crate) end: usize,
        }

        impl<$($type: IntoAbstract),+> $tight<$($type),+> {
            /// Return a chunk iterator over `step` component at a time.
            /// If `step` doesn't divide the length perfectly, the last chunk will be smaller.
            pub fn into_chunk(self, step: usize) -> $chunk<$($type),+> {
                $chunk {
                    data: self.data,
                    current: self.current,
                    end: self.end,
                    step,
                }
            }
            /// Return a chunk iterator over `step` component at a time.
            /// If `step` doesn't divide the length perfectly, the remaining elements can be fetched with `remainder`.
            pub fn into_chunk_exact(self, step: usize) -> $chunk_exact<$($type),+> {
                $chunk_exact {
                    data: self.data,
                    current: self.current,
                    end: self.end,
                    step,
                }
            }
        }

        impl<$($type: IntoAbstract),+> Shiperator for $tight<$($type),+> {
            type Item = ($(<$type::AbsView as AbstractMut>::Out,)+);

            fn first_pass(&mut self) -> Option<Self::Item> {
                let current = self.current;
                if current < self.end {
                    self.current += 1;
                    // SAFE we checked for OOB and the lifetime is ok
                    Some(unsafe {($(self.data.$index.get_data(current),)+)})
                } else {
                    None
                }
            }
            fn post_process(&mut self) {}
            fn size_hint(&self) -> (usize, Option<usize>) {
                let len = self.end - self.current;
                (len, Some(len))
            }
        }

        impl<$($type: IntoAbstract),+> CurrentId for $tight<$($type),+> {
            type Id = EntityId;

            unsafe fn current_id(&self) -> Self::Id {
                self.data.0.id_at(self.current - 1)
            }
        }

        impl<$($type: IntoAbstract),+> ExactSizeShiperator for $tight<$($type),+> {}

        impl<$($type: IntoAbstract),+> DoubleEndedShiperator for $tight<$($type),+> {
            fn first_pass_back(&mut self) -> Option<Self::Item> {
                if self.current < self.end {
                    self.end -= 1;
                    // SAFE we checked for OOB and the lifetime is ok
                    Some(unsafe { ($(self.data.$index.get_data(self.end),)+) })
                } else {
                    None
                }
            }
        }

        #[cfg(feature = "parallel")]
        impl<$($type: IntoAbstract),+> Producer for $tight<$($type),+>
        where $($type::AbsView: Clone + Send,)+ $(<$type::AbsView as AbstractMut>::Out: Send),+ {
            type Item = ($(<$type::AbsView as AbstractMut>::Out),+);
            type IntoIter = IntoIterator<Self>;
            fn into_iter(self) -> Self::IntoIter {
                core::iter::IntoIterator::into_iter(self)
            }
            fn split_at(mut self, index: usize) -> (Self, Self) {
                let clone = $tight {
                    data: ($(self.data.$index.clone(),)+),
                    current: self.current + index,
                    end: self.end,
                };
                self.end = clone.current;
                (self, clone)
            }
        }

        impl<$($type: IntoAbstract),+> core::iter::IntoIterator for $tight<$($type),+> {
            type IntoIter = IntoIterator<Self>;
            type Item = <Self as Shiperator>::Item;
            fn into_iter(self) -> Self::IntoIter {
                IntoIterator(self)
            }
        }
    }
}

macro_rules! iterators {
    (
        $($number: literal)*; $number1: literal $($queue_number: literal)+;
        $($tight: ident)*; $tight1: ident $($queue_tight: ident)+;
        $($chunk: ident)*; $chunk1: ident $($queue_chunk: ident)+;
        $($chunk_exact: ident)*; $chunk_exact1: ident $($queue_chunk_exact: ident)+;
        $(($type: ident, $index: tt))*;($type1: ident, $index1: tt) $(($queue_type: ident, $queue_index: tt))*
    ) => {
        impl_iterators![$number1 $tight1 $chunk1 $chunk_exact1 $(($type, $index))*];
        iterators![
            $($number)* $number1; $($queue_number)+;
            $($tight)* $tight1; $($queue_tight)+;
            $($chunk)* $chunk1; $($queue_chunk)+;
            $($chunk_exact)* $chunk_exact1; $($queue_chunk_exact)+;
            $(($type, $index))* ($type1, $index1); $(($queue_type, $queue_index))*
        ];
    };
    (
        $($number: literal)*; $number1: literal;
        $($tight: ident)*; $tight1: ident;
        $($chunk: ident)*; $chunk1: ident;
        $($chunk_exact: ident)*; $chunk_exact1: ident;
        $(($type: ident, $index: tt))*;
    ) => {
        impl_iterators![$number1 $tight1 $chunk1 $chunk_exact1 $(($type, $index))*];
    }
}

iterators![
    ;"2" "3" "4" "5" "6" "7" "8" "9" "10";
    ;Tight2 Tight3 Tight4 Tight5 Tight6 Tight7 Tight8 Tight9 Tight10;
    ;Chunk2 Chunk3 Chunk4 Chunk5 Chunk6 Chunk7 Chunk8 Chunk9 Chunk10;
    ;ChunkExact2 ChunkExact3 ChunkExact4 ChunkExact5 ChunkExact6 ChunkExact7 ChunkExact8 ChunkExact9 ChunkExact10;
    (A, 0) (B, 1); (C, 2) (D, 3) (E, 4) (F, 5) (G, 6) (H, 7) (I, 8) (J, 9)
];
