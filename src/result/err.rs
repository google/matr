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
    pub struct Err<
        K: Kind, 
        E
    >: Expr<Result<K>> {
        type Eval = WrapResultValue<K, ErrValue<K, E>>;
    }
}

// These have to be public because otherwise Rust would complain that "can't leak private type".
// But they should never be explicitly referenced elsewhere.
mod internal {
    pub use super::super::internal::*;
    
    meta!{
        pub struct ErrImpl<K: Kind, E>: ResultTrait<K> {
            type Visit<ResultK: Kind, Visitor: ResultVisitor<K, ResultK>> = Visitor::VisitErr<E>;
        }
        
        pub struct ErrValue<K: Kind, E>: ResultValue<K> {
            type Impl = ErrImpl<K, E>;
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use std::marker::PhantomData;
    use crate::*;
    use crate::r#type::*;
    use super::super::*;

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

    struct MyError {}

    #[test]
    fn visit() {
        meta_assert_eq!(Type, VisitResult<Type, Type, Err<Type, MyError>, MyVisitor>, WrapType<CalledVisitErr<MyError>>);
    }
}
