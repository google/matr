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

pub fn to_vec<K: Kind, L: Expr<List<K>>, OutT, F: Functor1<K, RuntimeFn<OutT, ()>>>() -> Vec<OutT> {
    return call_runtime_fn::<Vec<OutT>, (), ToReversedVec<K, ReverseList<K, L>, OutT, F>>(());
}

mod internal {
    pub use super::super::internal::*;

    meta!{
        pub type ToReversedVec<
            K: Kind,
            L: Expr<List<K>>,
            OutT,
            F: Functor1<K, RuntimeFn<OutT, ()>>
        >: Expr<RuntimeFn<Vec<OutT>, ()>> =
            VisitList<K, RuntimeFn<Vec<OutT>, ()>, L, ToReversedVecVisitor<K, OutT, F>>;

        pub struct ToReversedVecVisitor<
            K: Kind,
            OutT,
            F: Functor1<K, RuntimeFn<OutT, ()>>
        >: ListVisitor<K, RuntimeFn<Vec<OutT>, ()>> {
            type VisitEmptyList = WrapRuntimeFn<Vec<OutT>, (), EmptyVec<OutT>>;
            type VisitCons<Elem: Expr<K>, Tail: Expr<List<K>>> = WrapRuntimeFn<Vec<OutT>, (), AddToVecImpl<K, Elem, OutT, F, ToReversedVec<K, Tail, OutT, F>>>;
        }

        pub struct EmptyVec<OutT>: RuntimeFnTrait<Vec<OutT>, ()> {
            fn apply(_: ()) -> Vec<OutT> {
                return vec![];
            }
        }

        pub struct AddToVecImpl<
            K: Kind,
            Elem: Expr<K>,
            OutT,
            F: Functor1<K, RuntimeFn<OutT, ()>>,
            TailVec: Expr<RuntimeFn<Vec<OutT>, ()>>
        >: RuntimeFnTrait<Vec<OutT>, ()> {
            fn apply(_: ()) -> Vec<OutT> {
                let mut v = call_runtime_fn::<Vec<OutT>, (), TailVec>(());
                v.push(call_runtime_fn::<OutT, (), F::Apply<Elem>>(()));
                return v;
            }
        }
    }
}
