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

pub fn to_hash_set<K: EqualityComparableKind, OutT: Eq + Hash, S: Expr<Set<K>>, F: Functor1<K, RuntimeFn<OutT, ()>>>() -> HashSet<OutT> {
    return call_runtime_fn::<HashSet<OutT>, (), VisitSet<K, RuntimeFn<HashSet<OutT>, ()>, S, ToHashSetVisitor<K, OutT, F>>>(());
}

mod internal {
    use std::collections::HashSet;
    use std::hash::Hash;
    pub use super::super::internal::*;
    
    meta!{
        pub struct ToHashSetVisitor<
            K: EqualityComparableKind, 
            OutT: Eq + Hash,
            F: Functor1<K, RuntimeFn<OutT, ()>>
        >: SetVisitor<K, RuntimeFn<HashSet<OutT>, ()>> {
            type VisitEmptySet = WrapRuntimeFn<HashSet<OutT>, (), EmptyHashSetImpl<OutT>>;
            type VisitCons<Elem: Expr<K>, Tail: Expr<Set<K>>> = WrapRuntimeFn<HashSet<OutT>, (), AddToHashSetImpl<K, Elem, OutT, F, VisitSet<K, RuntimeFn<HashSet<OutT>, ()>, Tail, ToHashSetVisitor<K, OutT, F>>>>;
        }

        pub struct EmptyHashSetImpl<
            OutT: Eq + Hash
        >: RuntimeFnTrait<HashSet<OutT>, ()> {
            fn apply(_: ()) -> HashSet<OutT> {
                return HashSet::new();
            }
        }
        
        pub struct AddToHashSetImpl<
            K: Kind, 
            Elem: Expr<K>, 
            OutT: Eq + Hash, 
            F: Functor1<K, RuntimeFn<OutT, ()>>,
            TailHashSet: Expr<RuntimeFn<HashSet<OutT>, ()>>
        >: RuntimeFnTrait<HashSet<OutT>, ()> {
            fn apply(_: ()) -> HashSet<OutT> {
                let mut s = call_runtime_fn::<HashSet<OutT>, (), TailHashSet>(());
                s.insert(call_runtime_fn::<OutT, (), F::Apply<Elem>>(()));
                return s;
            }
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use std::any::type_name;
    use std::collections::HashSet;
    use crate::*;

    meta!{
        struct TypeToStr: Functor1<Type, RuntimeFn<&'static str, ()>> {
            type Apply<X: Expr<Type>> = WrapRuntimeFn<&'static str, (), TypeToStrImpl<X>>;
        }

        struct TypeToStrImpl<
            X: Expr<Type>
        >: RuntimeFnTrait<&str, ()> {
            fn apply(_: ()) -> &'static str {
                type_name::<UnwrapType<X>>()
            }
        }
    }

    #[test]
    fn empty_set_to_hash_set() {
        let s = to_hash_set::<Type, &'static str, EmptySet<Type>, TypeToStr>();
        assert_eq!(s, HashSet::new());
    }

    #[test]
    fn set_to_hash_set() {
        struct Foo {}
        struct Bar {}

        type S = type_set!{Foo, Bar};
        let s = to_hash_set::<Type, &'static str, S, TypeToStr>();
        assert_eq!(s, HashSet::from([
            "matr::set::to_hash_set::tests::set_to_hash_set::Foo",
            "matr::set::to_hash_set::tests::set_to_hash_set::Bar",
        ]));
    }
}
