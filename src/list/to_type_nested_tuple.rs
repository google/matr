// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use internal::*;

// Converts a List<Type> into a tuple of the form (T0, (T1, (T2, ()))).
meta!{
    pub type ToTypeNestedTuple<
        L: Expr<List<Type>>
    >: Expr<Type> =
        VisitList<Type, Type, L, ToTypeNestedTupleVisitor>;
}

mod internal {
    pub use super::super::internal::*;

    meta!{
        pub struct ToTypeNestedTupleVisitor: ListVisitor<Type, Type> {
            type VisitEmptyList = WrapType<()>;
            type VisitCons<Elem: Expr<Type>, Tail: Expr<List<Type>>> = WrapType<(
                UnwrapType<Elem>,
                UnwrapType<ToTypeNestedTuple<Tail>>
            )>;
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;

    #[test]
    fn empty_list_to_type_nested_tuple() {
        meta_assert_eq!(Type, ToTypeNestedTuple<EmptyList<Type>>, WrapType<()>);
    }

    #[test]
    fn list_to_type_nested_tuple() {
        type L = type_list![usize, f32, u64];
        meta_assert_eq!(Type, ToTypeNestedTuple<L>, WrapType<(usize, (f32, (u64, ())))>);
    }
}
