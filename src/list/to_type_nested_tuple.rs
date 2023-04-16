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

// Converts a List<Type> into a tuple of the form (T0, (T1, (T2, ()))).
pub struct ToTypeNestedTuple<L: Expr<List<Type>>> {
    l: PhantomData<L>,
}

impl<L: Expr<List<Type>>> Expr<Type> for ToTypeNestedTuple<L> {
    type Eval = <VisitList<Type, Type, L, ToTypeNestedTupleVisitor> as Expr<Type>>::Eval;
}

mod internal {
    pub use super::super::internal::*;
    
    pub struct ToTypeNestedTupleVisitor {}

    impl ListVisitor<Type, Type> for ToTypeNestedTupleVisitor {
        type VisitEmptyList = WrapType<()>;
        type VisitCons<Elem: Expr<Type>, Tail: Expr<List<Type>>> = WrapType<(
            <GetType<Elem> as GetTypeTrait>::Get,
            <GetType<ToTypeNestedTuple<Tail>> as GetTypeTrait>::Get
        )>;
    }

}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;

    #[test]
    fn empty_list_to_type_nested_tuple() {
        assert_type_eq!(ToTypeNestedTuple<EmptyList<Type>>, WrapType<()>);
    }

    #[test]
    fn list_to_type_nested_tuple() {
        type L = type_list![usize, f32, u64];
        assert_type_eq!(ToTypeNestedTuple<L>, WrapType<(usize, (f32, (u64, ())))>);
    }
}
