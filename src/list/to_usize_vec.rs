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

pub fn to_usize_vec<L: Expr<List<USize>>>() -> Vec<usize> {
    return to_vec::<USize, L, usize, ToUsizeFunctor>();
}

mod internal {
    pub use super::super::internal::*;

    meta!{
        pub struct ToUsizeFunctor: Functor1<USize, RuntimeFn<usize, ()>> {
            type Apply<N: Expr<USize>> = ToUSize<N>;
        }

        pub struct ToUSize<
            N: Expr<USize>
        >: Expr<RuntimeFn<usize, ()>> {
            type Eval = ToUSizeValue<N>;
        }

        pub struct ToUSizeValue<
            N: Expr<USize>
        >: RuntimeFnValue<usize, ()> {
            type Impl = ToUSizeImpl<N>;
        }

        pub struct ToUSizeImpl<
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
    use crate::*;

    #[test]
    fn empty_list_to_usize_vec() {
        let v = to_usize_vec::<EmptyList<USize>>();
        assert_eq!(v, vec![]);
    }

    #[test]
    fn list_to_usize_vec() {
        type N3 = Increment<Increment<Increment<Zero>>>;
        type N7 = Increment<Increment<Increment<Increment<N3>>>>;
        type N8 = Increment<N7>;
        type L = Cons<USize, N7, Cons<USize, N8, Cons<USize, N3, EmptyList<USize>>>>;
        let v = to_usize_vec::<L>();
        assert_eq!(v, vec![7, 8, 3]);
    }
}
