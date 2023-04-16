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

use std::collections::HashSet;
use std::hash::Hash;
use internal::*;

pub fn to_hash_set<K: EqualityComparableKind, S: Expr<Set<K>>, OutT: Eq + Hash, F: Functor1<K, RuntimeFn<OutT, ()>>>() -> HashSet<OutT> {
    return call_runtime_fn::<HashSet<OutT>, (), VisitSet<K, RuntimeFn<HashSet<OutT>, ()>, S, ToHashSetVisitor<K, OutT, F>>>(());
}

mod internal {
    use std::collections::HashSet;
    use std::hash::Hash;
    use std::marker::PhantomData;
    pub use super::super::internal::*;

    pub struct ToHashSetVisitor<K: EqualityComparableKind, OutT: Eq + Hash, F: Functor1<K, RuntimeFn<OutT, ()>>> {
        k: PhantomData<K>,
        out_t: PhantomData<OutT>,
        f: PhantomData<F>,
    }

    impl<K: EqualityComparableKind, OutT: Eq + Hash, F: Functor1<K, RuntimeFn<OutT, ()>>> SetVisitor<K, RuntimeFn<HashSet<OutT>, ()>> for ToHashSetVisitor<K, OutT, F> {
        type VisitEmptySet = EmptyHashSet<OutT>;
        type VisitCons<Elem: Expr<K>, Tail: Expr<Set<K>>> = AddToHashSet<K, Elem, OutT, F, VisitSet<K, RuntimeFn<HashSet<OutT>, ()>, Tail, ToHashSetVisitor<K, OutT, F>>>;
    }

    pub struct EmptyHashSet<OutT: Eq + Hash> {
        out_t: PhantomData<OutT>,
    }

    impl<OutT: Eq + Hash> Expr<RuntimeFn<HashSet<OutT>, ()>> for EmptyHashSet<OutT> {
        type Eval = EmptyHashSetValue<OutT>;
    }

    pub struct EmptyHashSetValue<OutT: Eq + Hash> {
        out_t: PhantomData<OutT>,
    }

    impl<OutT: Eq + Hash> RuntimeFnValue<HashSet<OutT>, ()> for EmptyHashSetValue<OutT> {
        type Impl = EmptyHashSetImpl<OutT>;
    }

    pub struct EmptyHashSetImpl<OutT: Eq + Hash> {
        out_t: PhantomData<OutT>,
    }

    impl<OutT: Eq + Hash> RuntimeFnTrait<HashSet<OutT>, ()> for EmptyHashSetImpl<OutT> {
        fn apply(_: ()) -> HashSet<OutT> {
            return HashSet::new();
        }
    }

    pub struct AddToHashSet<K: Kind, Elem: Expr<K>, OutT: Eq + Hash, F: Functor1<K, RuntimeFn<OutT, ()>>, TailHashSet: Expr<RuntimeFn<HashSet<OutT>, ()>>> {
        k: PhantomData<K>,
        elem: PhantomData<Elem>,
        out_t: PhantomData<OutT>,
        f: PhantomData<F>,
        tail_vec: PhantomData<TailHashSet>,
    }

    impl<K: Kind, Elem: Expr<K>, OutT: Eq + Hash, F: Functor1<K, RuntimeFn<OutT, ()>>, TailHashSet: Expr<RuntimeFn<HashSet<OutT>, ()>>> Expr<RuntimeFn<HashSet<OutT>, ()>> for AddToHashSet<K, Elem, OutT, F, TailHashSet> {
        type Eval = AddToHashSetValue<K, Elem, OutT, F, TailHashSet>;
    }

    pub struct AddToHashSetValue<K: Kind, Elem: Expr<K>, OutT: Eq + Hash, F: Functor1<K, RuntimeFn<OutT, ()>>, TailHashSet: Expr<RuntimeFn<HashSet<OutT>, ()>>> {
        k: PhantomData<K>,
        elem: PhantomData<Elem>,
        out_t: PhantomData<OutT>,
        f: PhantomData<F>,
        tail_vec: PhantomData<TailHashSet>,
    }

    impl<K: Kind, Elem: Expr<K>, OutT: Eq + Hash, F: Functor1<K, RuntimeFn<OutT, ()>>, TailHashSet: Expr<RuntimeFn<HashSet<OutT>, ()>>> RuntimeFnValue<HashSet<OutT>, ()> for AddToHashSetValue<K, Elem, OutT, F, TailHashSet> {
        type Impl = AddToHashSetImpl<K, Elem, OutT, F, TailHashSet>;
    }

    pub struct AddToHashSetImpl<K: Kind, Elem: Expr<K>, OutT: Eq + Hash, F: Functor1<K, RuntimeFn<OutT, ()>>, TailHashSet: Expr<RuntimeFn<HashSet<OutT>, ()>>> {
        k: PhantomData<K>,
        elem: PhantomData<Elem>,
        out_t: PhantomData<OutT>,
        f: PhantomData<F>,
        tail_vec: PhantomData<TailHashSet>,
    }

    impl<K: Kind, Elem: Expr<K>, OutT: Eq + Hash, F: Functor1<K, RuntimeFn<OutT, ()>>, TailHashSet: Expr<RuntimeFn<HashSet<OutT>, ()>>> RuntimeFnTrait<HashSet<OutT>, ()> for AddToHashSetImpl<K, Elem, OutT, F, TailHashSet> {
        fn apply(_: ()) -> HashSet<OutT> {
            let mut s = call_runtime_fn::<HashSet<OutT>, (), TailHashSet>(());
            s.insert(call_runtime_fn::<OutT, (), F::Apply<Elem>>(()));
            return s;
        }
    }
}
