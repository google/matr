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

meta!{
    pub struct Cons<
        K: Kind, 
        Elem: Expr<K>, 
        Tail: Expr<List<K>>
    >: Expr<List<K>> {
        type Eval = WrapListValue<K, ConsValue<K, Elem, Tail>>;
    }
}

mod internal {
    pub use super::super::internal::*;
    
    meta!{
        pub struct ConsValue<
            K: Kind, 
            Elem: Expr<K>, 
            Tail: Expr<List<K>>
        >: ListValue<K> {
            type Impl = ConsImpl<K, Elem, Tail>;
        }

        pub struct ConsImpl<
            K: Kind, 
            Elem: Expr<K>, 
            Tail: Expr<List<K>>
        >: ListTrait<K> {
            type Visit<ResultK: Kind, V: ListVisitor<K, ResultK>> = V::VisitCons<Elem, Tail>;
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use std::marker::PhantomData;
    use crate::*;
    use super::super::*;
    use crate::r#type::*;

    struct CalledVisitEmptyList {}

    struct CalledVisitCons<Elem: Expr<Type>, Tail: Expr<List<Type>>> {
        elem: PhantomData<Elem>,
        tail: PhantomData<Tail>,
    }

    meta!{
        struct MyVisitor : ListVisitor<Type, Type> {
            type VisitEmptyList = WrapType<CalledVisitEmptyList>;
            type VisitCons<Elem: Expr<Type>, Tail: Expr<List<Type>>> = WrapType<CalledVisitCons<Elem, Tail>>;
        }
    }

    #[test]
    fn visit() {
        meta_assert_eq!(Type, VisitList<Type, Type, Cons<Type, WrapType<i32>, Cons<Type, WrapType<u32>, EmptyList<Type>>>, MyVisitor>, WrapType<CalledVisitCons<WrapType<i32>, Cons<Type, WrapType<u32>, EmptyList<Type>>>>);
    }
}
