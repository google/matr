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

pub fn to_hash_map<K: EqualityComparableKind + KindWithDefault, V: KindWithDefault, M: Expr<Map<K, V>>, OutK: Eq + Hash, OutV, KeyF: Functor1<K, RuntimeFn<OutK, ()>>, ValueF: Functor1<V, RuntimeFn<OutV, ()>>>() -> HashMap<OutK, OutV> {
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
            type VisitEmptyMap = EmptyHashMap<OutK, OutV>;
            type VisitEntry<Key: Expr<K>, Value: Expr<V>, Tail: Expr<Map<K, V>>> = AddToHashMap<K, V, ConsPair<K, V, Key, Value>, OutK, OutV, KeyF, ValueF, VisitMap<K, V, RuntimeFn<HashMap<OutK, OutV>, ()>, Tail, ToHashMapVisitor<K, V, OutK, OutV, KeyF, ValueF>>>;
        }

        pub struct EmptyHashMap<
            OutK: Eq + Hash, 
            OutV
        >: Expr<RuntimeFn<HashMap<OutK, OutV>, ()>> {
            type Eval = EmptyHashMapValue<OutK, OutV>;
        }

        pub struct EmptyHashMapValue<
            OutK: Eq + Hash,
            OutV
        >: RuntimeFnValue<HashMap<OutK, OutV>, ()> {
            type Impl = EmptyHashMapImpl<OutK, OutV>;
        }

        pub struct EmptyHashMapImpl<OutK: Eq + Hash, OutV>: RuntimeFnTrait<HashMap<OutK, OutV>, ()> {
            fn apply(_: ()) -> HashMap<OutK, OutV> {
                return HashMap::new();
            }
        }

        pub struct AddToHashMap<
            K: KindWithDefault,
            V: KindWithDefault,
            Elem: Expr<Pair<K, V>>,
            OutK: Eq + Hash, 
            OutV, 
            KeyF: Functor1<K, RuntimeFn<OutK, ()>>,
            ValueF: Functor1<V, RuntimeFn<OutV, ()>>, 
            TailHashMap: Expr<RuntimeFn<HashMap<OutK, OutV>, ()>>
        >: Expr<RuntimeFn<HashMap<OutK, OutV>, ()>> {
            type Eval = AddToHashMapValue<K, V, Elem, OutK, OutV, KeyF, ValueF, TailHashMap>;
        }

        pub struct AddToHashMapValue<
            K: KindWithDefault, 
            V: KindWithDefault, 
            Elem: Expr<Pair<K, V>>, 
            OutK: Eq + Hash, 
            OutV, 
            KeyF: Functor1<K, RuntimeFn<OutK, ()>>, 
            ValueF: Functor1<V, RuntimeFn<OutV, ()>>,
            TailHashMap: Expr<RuntimeFn<HashMap<OutK, OutV>, ()>>
        >: RuntimeFnValue<HashMap<OutK, OutV>, ()> {
            type Impl = AddToHashMapImpl<K, V, Elem, OutK, OutV, KeyF, ValueF, TailHashMap>;
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
