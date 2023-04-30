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
    pub type IsEven<
        N: Expr<USize>
    >: Expr<Bool> = 
        VisitUSize<Bool, N, IsEvenVisitor>;
    
    pub type IsOdd<
        N: Expr<USize>
    >: Expr<Bool> = 
        VisitUSize<Bool, N, IsOddVisitor>;
}

mod internal {
    pub use super::super::internal::*;
    
    meta!{
        pub struct IsEvenVisitor: USizeVisitor<Bool> {
            type VisitZero = True;
            type VisitIncrement<N: Expr<USize>> = VisitUSize<Bool, N, IsOddVisitor>;
        }

        pub struct IsOddVisitor: USizeVisitor<Bool> {
            type VisitZero = False;
            type VisitIncrement<N: Expr<USize>> = VisitUSize<Bool, N, IsEvenVisitor>;
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;

    #[test]
    fn is_even() {
        meta_assert_eq!(Bool, IsEven<Zero>, True);
        meta_assert_eq!(Bool, IsEven<Increment<Zero>>, False);
        meta_assert_eq!(Bool, IsEven<Increment<Increment<Zero>>>, True);
    }

    #[test]
    fn is_odd() {
        meta_assert_eq!(Bool, IsOdd<Zero>, False);
        meta_assert_eq!(Bool, IsOdd<Increment<Zero>>, True);
        meta_assert_eq!(Bool, IsOdd<Increment<Increment<Zero>>>, False);
    }
}
