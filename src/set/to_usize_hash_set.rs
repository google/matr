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
use std::hash::Hash;
use internal::*;

pub fn to_usize_hash_set<S: Expr<Set<USize>>, OutT: Eq + Hash>() -> HashSet<usize> {
    return to_hash_set::<USize, S, usize, ToUSizeHashSetFunctor>();
}

mod internal {
    pub use super::super::internal::*;
    
    meta!{
        pub struct ToUSizeHashSetFunctor: Functor1<USize, RuntimeFn<usize, ()>> {
            type Apply<N: Expr<USize>> = ToUSizeConstFn<N>;
        }
        
        pub struct ToUSizeConstFn<
            N: Expr<USize>
        >: Expr<RuntimeFn<usize, ()>> {
            type Eval = ToUSizeConstFnValue<N>;
        }
        
        pub struct ToUSizeConstFnValue<
            N: Expr<USize>
        >: RuntimeFnValue<usize, ()> {
            type Impl = ToUSizeConstFnImpl<N>;
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
