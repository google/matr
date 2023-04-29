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
    pub struct None<
        K: Kind
    >: Expr<Option<K>> {
        type Eval = NoneValue<K>;
    }
}

mod internal {
    pub use super::super::internal::*;
    
    meta!{
        pub struct NoneValue<
            K: Kind
        >: OptionValue<K> {
            type Impl = NoneImpl<K>;
        }

        pub struct NoneImpl<
            K: Kind
        >: OptionTrait<K> {
            type Visit<ResultK: Kind, Visitor: OptionVisitor<K, ResultK>> = Visitor::VisitNone;
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use std::marker::PhantomData;
    use crate::*;

    struct CalledVisitNone {}

    struct CalledVisitSome<Elem: Expr<Type>> {
        elem: PhantomData<Elem>,
    }

    meta!{
        struct MyVisitor : OptionVisitor<Type, Type> {
            type VisitNone = WrapType<CalledVisitNone>;
            type VisitSome<X: Expr<Type>> = WrapType<CalledVisitSome<X>>;
        }
    }

    #[test]
    fn visit() {
        meta_assert_eq!(Type, VisitOption<Type, Type, None<Type>, MyVisitor>, WrapType<CalledVisitNone>);
    }
}
