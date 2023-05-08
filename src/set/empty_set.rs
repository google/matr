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
use crate::list::*;

meta!{
    pub type EmptySet<K: EqualityComparableKind>: Expr<Set<K>> =
        ListToSetUnchecked<K, EmptyList<K>>;
}

mod internal {
    pub use super::super::internal::*;
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use std::marker::PhantomData;
    use crate::*;
    use super::super::*;
    use crate::r#type::*;

    struct CalledVisitEmptySet {}

    struct CalledVisitCons<X: Expr<Type>, Tail: Expr<Set<Type>>> {
        x: PhantomData<X>,
        tail: PhantomData<Tail>,
    }

    meta!{
        struct MyVisitor : SetVisitor<Type, Type> {
            type VisitEmptySet = WrapType<CalledVisitEmptySet>;
            type VisitCons<X: Expr<Type>, Tail: Expr<Set<Type>>> = WrapType<CalledVisitCons<X, Tail>>;
        }
    }

    #[test]
    fn visit() {
        meta_assert_eq!(Type, VisitSet<Type, Type, EmptySet<Type>, MyVisitor>, WrapType<CalledVisitEmptySet>);
    }
}
