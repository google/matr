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
use crate::pair::*;

meta!{
    pub type EmptyMap<
        K: EqualityComparableKind,
        V: Kind
    >: Expr<Map<K, V>> =
        ListToMapUnchecked<K, V, EmptyList<Pair<K, V>>>;
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

    struct CalledVisitEmptyMap {}

    struct CalledVisitEntry<Key: Expr<Type>, Value: Expr<Type>, Tail: Expr<Map<Type, Type>>> {
        key: PhantomData<Key>,
        value: PhantomData<Value>,
        tail: PhantomData<Tail>,
    }

    meta!{
        struct MyVisitor : MapVisitor<Type, Type, Type> {
            type VisitEmptyMap = WrapType<CalledVisitEmptyMap>;
            type VisitEntry<Key: Expr<Type>, Value: Expr<Type>, Tail: Expr<Map<Type, Type>>> = WrapType<CalledVisitEntry<Key, Value, Tail>>;
        }
    }

    #[test]
    fn visit() {
        meta_assert_eq!(Type, VisitMap<Type, Type, Type, EmptyMap<Type, Type>, MyVisitor>, WrapType<CalledVisitEmptyMap>);
    }
}
