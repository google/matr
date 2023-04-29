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
    pub type OptionOrValue<
        K: Kind, 
        V: Expr<Option<K>>, 
        Fallback: Expr<K>
    >: Expr<K> =
        VisitOption<K, K, V, OptionOrValueVisitor<K, V, Fallback>>;
}

// These have to be public because otherwise Rust would complain that "can't leak private type".
// But they should never be explicitly referenced elsewhere.
mod internal {
    pub use super::super::internal::*;
    
    meta!{
        pub struct OptionOrValueVisitor<
            K: Kind, 
            V: Expr<Option<K>>,
            Fallback: Expr<K>
        >: OptionVisitor<K, K> {
            type VisitNone = Fallback;
            type VisitSome<V2: Expr<K>> = V2;
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;

    #[test]
    fn option_or_value() {
        meta_assert_eq!(Type, OptionOrValue<Type, None<Type>, WrapType<i32>>, WrapType<i32>);
        meta_assert_eq!(Type, OptionOrValue<Type, Some<Type, WrapType<usize>>, WrapType<i32>>, WrapType<usize>);
    }
}
