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

use std::collections::HashSet;
use internal::*;

pub fn to_usize_hash_set<S: Expr<Set<USize>>>() -> HashSet<usize> {
    return to_hash_set::<USize, usize, S, ToUSizeHashSetFunctor>();
}

mod internal {
    pub use super::super::internal::*;
    
    meta!{
        pub struct ToUSizeHashSetFunctor: Functor1<USize, RuntimeFn<usize, ()>> {
            type Apply<N: Expr<USize>> = WrapRuntimeFn<usize, (), ToUSizeConstFnImpl<N>>;
        }
        
        pub struct ToUSizeConstFnImpl<
            N: Expr<USize>
        >: RuntimeFnTrait<usize, ()> {
            fn apply(_: ()) -> usize {
                to_usize::<N>()
            }
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use std::collections::HashSet;
    use crate::*;

    #[test]
    fn empty_set_to_usize_hash_set() {
        let s = to_usize_hash_set::<EmptySet<USize>>();
        assert_eq!(s, HashSet::new());
    }

    #[test]
    fn set_to_usize_hash_set() {
        type N3 = Increment<Increment<Increment<Zero>>>;
        type N7 = Increment<Increment<Increment<Increment<N3>>>>;
        type N8 = Increment<N7>;
        type S = AddToSet<USize, N7, AddToSet<USize, N8, AddToSet<USize, N3, EmptySet<USize>>>>;
        let s = to_usize_hash_set::<S>();
        assert_eq!(s, HashSet::from([
            7,
            8,
            3,
        ]));
    }
}
