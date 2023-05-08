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
use crate::bool::*;

meta!{
    pub type IsNone<
        K: Kind,
        X: Expr<Option<K>>
    >: Expr<Bool> =
        VisitOption<K, Bool, X, IsNoneVisitor<K>>;
}

mod internal {
    pub use super::super::internal::*;
    use crate::bool::*;

    meta!{
        pub struct IsNoneVisitor<
            K: Kind
        >: OptionVisitor<K, Bool> {
            type VisitNone = True;
            type VisitSome<X: Expr<K>> = False;
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;
    use super::super::*;
    use crate::r#type::*;
    use crate::bool::*;

    #[test]
    fn is_none() {
        meta_assert_eq!(Bool, IsNone<Type, None<Type>>, True);
        meta_assert_eq!(Bool, IsNone<Type, Some<Type, WrapType<i32>>>, False);
    }
}
