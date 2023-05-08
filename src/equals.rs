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

use crate::*;
use crate::bool::*;

pub trait EqualityComparableKind: Kind {
    type Eq<X: Expr<Self>, Y: Expr<Self>>: Expr<Bool>;
}

meta!{
    pub type Equals<
        K: EqualityComparableKind,
        X: Expr<K>,
        Y: Expr<K>
    >: Expr<Bool> =
        <K as EqualityComparableKind>::Eq<X, Y>;
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;
    use crate::bool::*;
    use crate::r#type::*;

    #[test]
    fn equals_types() {
        meta_assert_eq!(Bool, Equals<Type, WrapType<i32>, WrapType<i32>>, True);
        meta_assert_eq!(Bool, Equals<Type, WrapType<i32>, WrapType<i64>>, False);
    }

    #[test]
    fn equals_bools() {
        meta_assert_eq!(Bool, Equals<Bool, True, True>, True);
        meta_assert_eq!(Bool, Equals<Bool, False, False>, True);
        meta_assert_eq!(Bool, Equals<Bool, True, False>, False);
        meta_assert_eq!(Bool, Equals<Bool, False, True>, False);
    }
}
