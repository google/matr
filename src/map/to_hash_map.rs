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

use std::collections::HashMap;
use std::hash::Hash;
use internal::*;

pub fn to_hash_map<K: EqualityComparableKind + KindWithDefault, V: KindWithDefault, OutK: Eq + Hash, OutV, M: Expr<Map<K, V>>, KeyF: Functor1<K, RuntimeFn<OutK, ()>>, ValueF: Functor1<V, RuntimeFn<OutV, ()>>>() -> HashMap<OutK, OutV> {
    return call_runtime_fn::<HashMap<OutK, OutV>, (),
        VisitMap<K, V, RuntimeFn<HashMap<OutK, OutV>, ()>, M, ToHashMapVisitor<K, V, OutK, OutV, KeyF, ValueF>>>(());
}

mod internal {
    use std::collections::HashMap;
    use std::hash::Hash;
    pub use super::super::internal::*;
    
    meta!{
        pub struct ToHashMapVisitor<
            K: KindWithDefault + EqualityComparableKind, 
            V: KindWithDefault, 
            OutK: Eq + Hash, 
            OutV, 
            KeyF: Functor1<K, RuntimeFn<OutK, ()>>, 
            ValueF: Functor1<V, RuntimeFn<OutV, ()>>
        >: MapVisitor<K, V, RuntimeFn<HashMap<OutK, OutV>, ()>> {
            type VisitEmptyMap = WrapRuntimeFn<HashMap<OutK, OutV>, (), EmptyHashMapImpl<OutK, OutV>>;
            type VisitEntry<Key: Expr<K>, Value: Expr<V>, Tail: Expr<Map<K, V>>> =
                WrapRuntimeFn<HashMap<OutK, OutV>, (),
                    AddToHashMapImpl<K, V, ConsPair<K, V, Key, Value>, OutK, OutV, KeyF, ValueF, VisitMap<K, V, RuntimeFn<HashMap<OutK, OutV>, ()>, Tail, ToHashMapVisitor<K, V, OutK, OutV, KeyF, ValueF>>>>;
        }

        pub struct EmptyHashMapImpl<OutK: Eq + Hash, OutV>: RuntimeFnTrait<HashMap<OutK, OutV>, ()> {
            fn apply(_: ()) -> HashMap<OutK, OutV> {
                return HashMap::new();
            }
        }

        pub struct AddToHashMapImpl<
            K: KindWithDefault, 
            V: KindWithDefault, 
            Elem: Expr<Pair<K, V>>, 
            OutK: Eq + Hash, 
            OutV, 
            KeyF: Functor1<K, RuntimeFn<OutK, ()>>,
            ValueF: Functor1<V, RuntimeFn<OutV, ()>>, 
            TailHashMap: Expr<RuntimeFn<HashMap<OutK, OutV>, ()>>
        >: RuntimeFnTrait<HashMap<OutK, OutV>, ()> {
            fn apply(_: ()) -> HashMap<OutK, OutV> {
                let mut s = call_runtime_fn::<HashMap<OutK, OutV>, (), TailHashMap>(());
                s.insert(
                    call_runtime_fn::<OutK, (), KeyF::Apply<GetFirst<K, V, Elem>>>(()),
                    call_runtime_fn::<OutV, (), ValueF::Apply<GetSecond<K, V, Elem>>>(())
                );
                return s;
            }
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use std::any::type_name;
    use std::collections::HashMap;
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
    fn empty_map_to_hash_map() {
        let m = to_hash_map::<Type, Type, &'static str, &'static str, EmptyMap<Type, Type>, TypeToStr, TypeToStr>();
        assert_eq!(m, HashMap::new());
    }

    #[test]
    fn map_to_hash_map() {
        struct Foo {}
        struct Bar {}

        type M = Put<Type, Type, WrapType<Foo>, WrapType<Bar>, EmptyMap<Type, Type>>;
        let m = to_hash_map::<Type, Type, &'static str, &'static str, M, TypeToStr, TypeToStr>();
        assert_eq!(m, HashMap::from([
            ("matr::map::to_hash_map::tests::map_to_hash_map::Foo", "matr::map::to_hash_map::tests::map_to_hash_map::Bar")
        ]));
    }
}
