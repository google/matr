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
    use std::marker::PhantomData;
    pub use super::super::internal::*;

    pub struct ToReversedVec<K: Kind, L: Expr<List<K>>, OutT, F: Functor1<K, RuntimeFn<OutT, ()>>> {
        k: PhantomData<K>,
        l: PhantomData<L>,
        out_t: PhantomData<OutT>,
        f: PhantomData<F>,
    }

    impl<K: Kind, L: Expr<List<K>>, OutT, F: Functor1<K, RuntimeFn<OutT, ()>>> Expr<RuntimeFn<Vec<OutT>, ()>> for ToReversedVec<K, L, OutT, F> {
        type Eval = ToReversedVecValue<K, L, OutT, F>;
    }

    pub struct ToReversedVecValue<K: Kind, L: Expr<List<K>>, OutT, F: Functor1<K, RuntimeFn<OutT, ()>>> {
        k: PhantomData<K>,
        l: PhantomData<L>,
        out_t: PhantomData<OutT>,
        f: PhantomData<F>,
    }

    impl<K: Kind, L: Expr<List<K>>, OutT, F: Functor1<K, RuntimeFn<OutT, ()>>> RuntimeFnValue<Vec<OutT>, ()> for ToReversedVecValue<K, L, OutT, F> {
        type Impl = AsRuntimeFn<Vec<OutT>, (), <AsList<K, L> as ListTrait<K>>::Visit<RuntimeFn<Vec<OutT>, ()>, ToReversedVecVisitor<K, OutT, F>>>;
    }

    pub struct ToReversedVecVisitor<K: Kind, OutT, F: Functor1<K, RuntimeFn<OutT, ()>>> {
        k: PhantomData<K>,
        out_t: PhantomData<OutT>,
        f: PhantomData<F>,
    }

    impl<K: Kind, OutT, F: Functor1<K, RuntimeFn<OutT, ()>>> ListVisitor<K, RuntimeFn<Vec<OutT>, ()>> for ToReversedVecVisitor<K, OutT, F> {
        type VisitEmptyList = EmptyVec<OutT>;
        type VisitCons<Elem: Expr<K>, Tail: Expr<List<K>>> = AddToVec<K, Elem, OutT, F, ToReversedVec<K, Tail, OutT, F>>;
    }

    pub struct EmptyVec<OutT> {
        out_t: PhantomData<OutT>,
    }

    impl<OutT> Expr<RuntimeFn<Vec<OutT>, ()>> for EmptyVec<OutT> {
        type Eval = EmptyVecValue<OutT>;
    }

    pub struct EmptyVecValue<OutT> {
        out_t: PhantomData<OutT>,
    }

    impl<OutT> RuntimeFnValue<Vec<OutT>, ()> for EmptyVecValue<OutT> {
        type Impl = EmptyVecImpl<OutT>;
    }

    pub struct EmptyVecImpl<OutT> {
        out_t: PhantomData<OutT>,
    }

    impl<OutT> RuntimeFnTrait<Vec<OutT>, ()> for EmptyVecImpl<OutT> {
        fn apply(_: ()) -> Vec<OutT> {
            return vec![];
        }
    }

    pub struct AddToVec<K: Kind, Elem: Expr<K>, OutT, F: Functor1<K, RuntimeFn<OutT, ()>>, TailVec: Expr<RuntimeFn<Vec<OutT>, ()>>> {
        k: PhantomData<K>,
        elem: PhantomData<Elem>,
        out_t: PhantomData<OutT>,
        f: PhantomData<F>,
        tail_vec: PhantomData<TailVec>,
    }

    impl<K: Kind, Elem: Expr<K>, OutT, F: Functor1<K, RuntimeFn<OutT, ()>>, TailVec: Expr<RuntimeFn<Vec<OutT>, ()>>> Expr<RuntimeFn<Vec<OutT>, ()>> for AddToVec<K, Elem, OutT, F, TailVec> {
        type Eval = AddToVecValue<K, Elem, OutT, F, TailVec>;
    }

    pub struct AddToVecValue<K: Kind, Elem: Expr<K>, OutT, F: Functor1<K, RuntimeFn<OutT, ()>>, TailVec: Expr<RuntimeFn<Vec<OutT>, ()>>> {
        k: PhantomData<K>,
        elem: PhantomData<Elem>,
        out_t: PhantomData<OutT>,
        f: PhantomData<F>,
        tail_vec: PhantomData<TailVec>,
    }

    impl<K: Kind, Elem: Expr<K>, OutT, F: Functor1<K, RuntimeFn<OutT, ()>>, TailVec: Expr<RuntimeFn<Vec<OutT>, ()>>> RuntimeFnValue<Vec<OutT>, ()> for AddToVecValue<K, Elem, OutT, F, TailVec> {
        type Impl = AddToVecImpl<K, Elem, OutT, F, TailVec>;
    }

    pub struct AddToVecImpl<K: Kind, Elem: Expr<K>, OutT, F: Functor1<K, RuntimeFn<OutT, ()>>, TailVec: Expr<RuntimeFn<Vec<OutT>, ()>>> {
        k: PhantomData<K>,
        elem: PhantomData<Elem>,
        out_t: PhantomData<OutT>,
        f: PhantomData<F>,
        tail_vec: PhantomData<TailVec>,
    }

    impl<K: Kind, Elem: Expr<K>, OutT, F: Functor1<K, RuntimeFn<OutT, ()>>, TailVec: Expr<RuntimeFn<Vec<OutT>, ()>>> RuntimeFnTrait<Vec<OutT>, ()> for AddToVecImpl<K, Elem, OutT, F, TailVec> {
        fn apply(_: ()) -> Vec<OutT> {
            let mut v = call_runtime_fn::<Vec<OutT>, (), TailVec>(());
            v.push(call_runtime_fn::<OutT, (), F::Apply<Elem>>(()));
            return v;
        }
    }
}
