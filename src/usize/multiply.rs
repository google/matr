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
    pub type Multiply<
        X: Expr<USize>, 
        Y: Expr<USize>
    >: Expr<USize> =
        VisitUSize<USize, X, MultiplyValueVisitor<Y>>;
}

mod internal {
    pub use super::super::internal::*;
    
    meta!{
        pub struct MultiplyValueVisitor<
            N: Expr<USize>
        >: USizeVisitor<USize> {
            type VisitZero = Zero;
            type VisitIncrement<Y: Expr<USize>> = Sum<N, Multiply<N, Y>>;
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;
    use super::super::*;

    #[test]
    fn multiply() {
        const N: usize = to_usize::<
            Multiply<
                Increment<Increment<Increment<Increment<Increment<Zero>>>>>,
                Increment<Increment<Zero>>
            >>();
        assert_eq!(N, 10);
    }
}
