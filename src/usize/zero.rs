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

pub struct Zero {}

impl Expr<USize> for Zero {
    type Eval = ZeroValue;
}

pub struct ZeroValue {}

impl USizeValue for ZeroValue {
    type Impl = ZeroImpl;
}

mod internal {
    pub use super::super::internal::*;

    pub struct ZeroImpl {}

    impl USizeTrait for ZeroImpl {
        type Visit<ResultK: Kind, V: USizeVisitor<ResultK>> = V::VisitZero;
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;

    #[test]
    fn zero() {
        const N: usize = to_usize::<Zero>();
        assert_eq!(N, 0);
    }
}

