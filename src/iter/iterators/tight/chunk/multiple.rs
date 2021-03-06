use super::{AbstractMut, IntoAbstract, IntoIterator, Shiperator};

macro_rules! impl_iterators {
    (
        $number: literal
        $chunk: ident
        $(($type: ident, $index: tt))+
    ) => {
        #[doc = "Chunk iterator over"]
        #[doc = $number]
        #[doc = "components.  
Returns a tuple of `size` long slices and not single elements.  
The last chunk's length will be smaller than `size` if `size` does not divide the iterator's length perfectly."]
        pub struct $chunk<$($type: IntoAbstract),+> {
            pub(crate) data: ($($type::AbsView,)+),
            pub(crate) current: usize,
            pub(crate) end: usize,
            pub(crate) step: usize,
        }

        impl<$($type: IntoAbstract),+> Shiperator for $chunk<$($type),+> {
            type Item = ($(<$type::AbsView as AbstractMut>::Slice,)+);

            fn first_pass(&mut self) -> Option<Self::Item> {
                let current = self.current;
                if current + self.step <= self.end {
                    self.current += self.step;
                    // SAFE we checked for OOB and the lifetime is ok
                    Some(unsafe {($(self.data.$index.get_data_slice(current..(current + self.step)),)+)})
                } else if current < self.end {
                    self.current = self.end;
                    // SAFE we checked for OOB and the lifetime is ok
                    Some(unsafe {($(self.data.$index.get_data_slice(current..self.end),)+)})
                } else {
                    None
                }
            }
            fn post_process(&mut self) {

            }
            fn size_hint(&self) -> (usize, Option<usize>) {
                let len = (self.end - self.current + self.step - 1) / self.step;
                (len, Some(len))
            }
        }

        impl<$($type: IntoAbstract),+> core::iter::IntoIterator for $chunk<$($type),+> {
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
        $($chunk: ident)*; $chunk1: ident $($queue_chunk: ident)+;
        $(($type: ident, $index: tt))*;($type1: ident, $index1: tt) $(($queue_type: ident, $queue_index: tt))*
    ) => {
        impl_iterators![$number1 $chunk1 $(($type, $index))*];
        iterators![
            $($number)* $number1; $($queue_number)+;
            $($chunk)* $chunk1; $($queue_chunk)+;
            $(($type, $index))* ($type1, $index1); $(($queue_type, $queue_index))*
        ];
    };
    (
        $($number: literal)*; $number1: literal;
        $($chunk: ident)*; $chunk1: ident;
        $(($type: ident, $index: tt))*;
    ) => {
        impl_iterators![$number1 $chunk1 $(($type, $index))*];
    }
}

iterators![
    ;"2" "3" "4" "5" "6" "7" "8" "9" "10";
    ;Chunk2 Chunk3 Chunk4 Chunk5 Chunk6 Chunk7 Chunk8 Chunk9 Chunk10;
    (A, 0) (B, 1); (C, 2) (D, 3) (E, 4) (F, 5) (G, 6) (H, 7) (I, 8) (J, 9)
];
