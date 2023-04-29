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
    pub type GetFirst<
        FirstK: KindWithDefault,
        SecondK: KindWithDefault,
        P: Expr<Pair<FirstK, SecondK>>
    >: Expr<FirstK> =
        <AsPair<FirstK, SecondK, P> as PairTrait<FirstK, SecondK>>::Visit<FirstK, GetFirstVisitor<FirstK, SecondK>>;
}

mod internal {
    pub use super::super::internal::*;
    
    meta!{
        pub struct GetFirstVisitor<FirstK: Kind, SecondK: Kind>: PairVisitor<FirstK, SecondK, FirstK> {
            type Visit<First: Expr<FirstK>, Second: Expr<SecondK>> = First;
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;

    #[test]
    fn get_first() {
        meta_assert_eq!(Type,
            GetFirst<Type, Type, ConsPair<Type, Type, WrapType<i32>, WrapType<i64>>>,
            WrapType<i32>);
    }
}
