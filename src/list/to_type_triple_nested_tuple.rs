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

use std::marker::PhantomData;
use internal::*;

// Converts a List<Pair<Type, Pair<Type, Type>> into a tuple of the form (T0, (T1, (T2, ()))).
pub struct ToTypeTripleNestedTuple<L: Expr<List<Pair<Type, Pair<Type, Type>>>>> {
    l: PhantomData<L>,
}

impl<L: Expr<List<Pair<Type, Pair<Type, Type>>>>> Expr<Type> for ToTypeTripleNestedTuple<L> {
    type Eval = <VisitList<Pair<Type, Pair<Type, Type>>, Type, L, ToTypeTripleNestedTupleVisitor> as Expr<Type>>::Eval;
}

mod internal {
    pub use super::super::internal::*;

    pub struct ToTypeTripleNestedTupleVisitor {}

    impl ListVisitor<Pair<Type, Pair<Type, Type>>, Type> for ToTypeTripleNestedTupleVisitor {
        type VisitEmptyList = WrapType<()>;
        type VisitCons<Elem: Expr<Pair<Type, Pair<Type, Type>>>, Tail: Expr<List<Pair<Type, Pair<Type, Type>>>>> = WrapType<(
            (
                <GetType<GetFirst<Type, Pair<Type, Type>, Elem>> as GetTypeTrait>::Get,
                (
                    <GetType<GetFirst<Type, Type, GetSecond<Type, Pair<Type, Type>, Elem>>> as GetTypeTrait>::Get,
                    <GetType<GetSecond<Type, Type, GetSecond<Type, Pair<Type, Type>, Elem>>> as GetTypeTrait>::Get,
                ),
            ),
            <GetType<ToTypeTripleNestedTuple<Tail>> as GetTypeTrait>::Get
        )>;
    }

}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;

    #[test]
    fn empty_list_to_type_triple_nested_tuple() {
        assert_type_eq!(ToTypeTripleNestedTuple<EmptyList<Pair<Type, Pair<Type, Type>>>>, WrapType<()>);
    }

    #[test]
    fn list_to_type_triple_nested_tuple() {
        type L = list!(<Pair<Type, Pair<Type, Type>>>[
            ConsPair<Type, Pair<Type, Type>, WrapType<usize>, ConsPair<Type, Type, WrapType<isize>, WrapType<(isize,)>>>,
            ConsPair<Type, Pair<Type, Type>, WrapType<f32>, ConsPair<Type, Type, WrapType<f64>, WrapType<(f64,)>>>,
            ConsPair<Type, Pair<Type, Type>, WrapType<u64>, ConsPair<Type, Type, WrapType<i64>, WrapType<(i64,)>>>]);
        assert_type_eq!(ToTypeTripleNestedTuple<L>, WrapType<((usize, (isize, (isize,))), ((f32, (f64, (f64,))), ((u64, (i64, (i64,))), ())))>);
    }
}
