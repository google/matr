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
use crate::pair::*;
use crate::r#type::*;

// Converts a List<Pair<Type, Pair<Type, Type>> into a tuple of the form ((T0, U0, V0), ((T1, U1, V1), ((T2, U2, V2), ()))).
meta!{
    pub type ToTypeTripleNestedTuple<
        L: Expr<List<Pair<Type, Pair<Type, Type>>>>
    >: Expr<Type> =
        VisitList<Pair<Type, Pair<Type, Type>>, Type, L, ToTypeTripleNestedTupleVisitor>;
}

mod internal {
    pub use super::super::internal::*;
    use crate::pair::*;
    use crate::r#type::*;

    meta!{
        pub struct ToTypeTripleNestedTupleVisitor: ListVisitor<Pair<Type, Pair<Type, Type>>, Type> {
            type VisitEmptyList = WrapType<()>;
            type VisitCons<Elem: Expr<Pair<Type, Pair<Type, Type>>>, Tail: Expr<List<Pair<Type, Pair<Type, Type>>>>> = WrapType<(
                (
                    UnwrapType<GetFirst<Type, Pair<Type, Type>, Elem>>,
                    (
                        UnwrapType<GetFirst<Type, Type, GetSecond<Type, Pair<Type, Type>, Elem>>>,
                        UnwrapType<GetSecond<Type, Type, GetSecond<Type, Pair<Type, Type>, Elem>>>,
                    ),
                ),
                UnwrapType<ToTypeTripleNestedTuple<Tail>>
            )>;
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;
    use super::super::*;
    use crate::pair::*;
    use crate::r#type::*;
    
    #[test]
    fn empty_list_to_type_triple_nested_tuple() {
        meta_assert_eq!(Type, ToTypeTripleNestedTuple<EmptyList<Pair<Type, Pair<Type, Type>>>>, WrapType<()>);
    }

    #[test]
    fn list_to_type_triple_nested_tuple() {
        type L = meta_list!(<Pair<Type, Pair<Type, Type>>>[
            ConsPair<Type, Pair<Type, Type>, WrapType<usize>, ConsPair<Type, Type, WrapType<isize>, WrapType<(isize,)>>>,
            ConsPair<Type, Pair<Type, Type>, WrapType<f32>, ConsPair<Type, Type, WrapType<f64>, WrapType<(f64,)>>>,
            ConsPair<Type, Pair<Type, Type>, WrapType<u64>, ConsPair<Type, Type, WrapType<i64>, WrapType<(i64,)>>>]);
        meta_assert_eq!(Type, ToTypeTripleNestedTuple<L>, WrapType<((usize, (isize, (isize,))), ((f32, (f64, (f64,))), ((u64, (i64, (i64,))), ())))>);
    }
}
