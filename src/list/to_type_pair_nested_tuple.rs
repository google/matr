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

// Converts a List<Pair<Type, Type> into a tuple of the form ((T0, U0), ((T1, U1), ((T2, U2), ()))).
meta!{
    pub type ToTypePairNestedTuple<
        L: Expr<List<Pair<Type, Type>>>
    >: Expr<Type> =
        VisitList<Pair<Type, Type>, Type, L, ToTypePairNestedTupleVisitor>;
}

mod internal {
    pub use super::super::internal::*;
    
    meta!{
        pub struct ToTypePairNestedTupleVisitor: ListVisitor<Pair<Type, Type>, Type> {
            type VisitEmptyList = WrapType<()>;
            type VisitCons<Elem: Expr<Pair<Type, Type>>, Tail: Expr<List<Pair<Type, Type>>>> = WrapType<(
                (
                    UnwrapType<GetFirst<Type, Type, Elem>>,
                    UnwrapType<GetSecond<Type, Type, Elem>>,
                ),
                UnwrapType<ToTypePairNestedTuple<Tail>>
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
        meta_assert_eq!(Type, ToTypePairNestedTuple<EmptyList<Pair<Type, Type>>>, WrapType<()>);
    }

    #[test]
    fn list_to_type_nested_tuple() {
        type L = meta_list!(<Pair<Type, Type>>[
            ConsPair<Type, Type, WrapType<usize>, WrapType<isize>>,
            ConsPair<Type, Type, WrapType<f32>, WrapType<f64>>,
            ConsPair<Type, Type, WrapType<u64>, WrapType<i64>>]);
        meta_assert_eq!(Type, ToTypePairNestedTuple<L>, WrapType<((usize, isize), ((f32, f64), ((u64, i64), ())))>);
    }
}
