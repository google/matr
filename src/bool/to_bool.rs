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
use crate::const_fn::*;

pub const fn to_bool<B: Expr<Bool>>() -> bool {
    return call_const_fn::<bool, (), If<ConstFn<bool, ()>, B, WrapConstFn<bool, (), ToTrueBoolConstFnImpl>, WrapConstFn<bool, (), ToFalseBoolConstFnImpl>>>(());
}

mod internal {
    pub use super::super::internal::*;
    use crate::const_fn::*;

    meta! {
        pub struct ToTrueBoolConstFnImpl: const ConstFnTrait<bool, ()> {
            fn apply(_: ()) -> bool {
                return true;
            }
        }
        
        pub struct ToFalseBoolConstFnImpl: const ConstFnTrait<bool, ()> {
            fn apply(_: ()) -> bool {
                return false;
            }
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;
    use super::super::*;

    #[test]
    fn to_bool_false() {
        const B: bool = to_bool::<False>();
        assert_eq!(B, false);
    }

    #[test]
    fn to_bool_true() {
        const B: bool = to_bool::<True>();
        assert_eq!(B, true);
    }
}
