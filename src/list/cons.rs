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

pub struct Cons<K: Kind, Elem: Expr<K>, Tail: Expr<List<K>>> {
    k: PhantomData<K>,
    elem: PhantomData<Elem>,
    tail: PhantomData<Tail>,
}

impl<K: Kind, Elem: Expr<K>, Tail: Expr<List<K>>> Expr<List<K>> for Cons<K, Elem, Tail> {
    type Eval = ConsValue<K, Elem, Tail>;
}

mod internal {
    use std::marker::PhantomData;
    pub use super::super::internal::*;

    pub struct ConsValue<K: Kind, Elem: Expr<K>, Tail: Expr<List<K>>> {
        k: PhantomData<K>,
        elem: PhantomData<Elem>,
        tail: PhantomData<Tail>,
    }

    impl<K: Kind, Elem: Expr<K>, Tail: Expr<List<K>>> ListValue<K> for ConsValue<K, Elem, Tail> {
        type Impl = ConsImpl<K, Elem, Tail>;
    }

    pub struct ConsImpl<K: Kind, Elem: Expr<K>, Tail: Expr<List<K>>> {
        k: PhantomData<K>,
        elem: PhantomData<Elem>,
        tail: PhantomData<Tail>,
    }

    impl<K: Kind, Elem: Expr<K>, Tail: Expr<List<K>>> ListTrait<K> for ConsImpl<K, Elem, Tail> {
        type Visit<ResultK: Kind, V: ListVisitor<K, ResultK>> = V::VisitCons<Elem, Tail>;
    }
}
