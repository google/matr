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
    pub struct Ok<
        K: Kind, 
        V: Expr<K>
    >: Expr<Result<K>> {
        type Eval = WrapResultValue<K, OkValue<K, V>>;
    }
}

// These have to be public because otherwise Rust would complain that "can't leak private type".
// But they should never be explicitly referenced elsewhere.
mod internal {
    pub use super::super::internal::*;
    
    meta!{
        pub struct OkValue<
            K: Kind, 
            V: Expr<K>
        >: ResultValue<K> {
            type Impl = OkImpl<K, V>;
        }
        
        pub struct OkImpl<
            K: Kind, 
            V: Expr<K>
        >: ResultTrait<K> {
            type Visit<ResultK: Kind, Visitor: ResultVisitor<K, ResultK>> = Visitor::VisitOk<V>;
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

    struct CalledVisitErr<E> {
        e: PhantomData<E>,
    }

    struct CalledVisitOk<Elem> {
        elem: PhantomData<Elem>,
    }

    meta!{
        struct MyVisitor : ResultVisitor<Type, Type> {
            type VisitErr<E> = WrapType<CalledVisitErr<E>>;
            type VisitOk<X: Expr<Type>> = WrapType<CalledVisitOk<UnwrapType<X>>>;
        }
    }

    #[test]
    fn visit() {
        meta_assert_eq!(Type, VisitResult<Type, Type, Ok<Type, WrapType<i32>>, MyVisitor>, WrapType<CalledVisitOk<i32>>);
    }
}
