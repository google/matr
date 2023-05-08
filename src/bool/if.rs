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
    pub type If<
        ResultK: Kind,
        B: Expr<Bool>,
        Then: Expr<ResultK>,
        Else: Expr<ResultK>
    >: Expr<ResultK> =
        <AsBool<B> as BoolTrait>::Cond<ResultK, Then, Else>;
}

// These have to be public because otherwise Rust would complain that "can't leak private type".
// But they should never be explicitly referenced elsewhere.
mod internal {
    pub use super::super::internal::*;
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;
    use super::super::*;
    use crate::r#type::*;
    use crate::usize::*;

    #[test]
    fn if_function() {
        meta_assert_eq!(Type, If<Type, True, WrapType<u32>, WrapType<f64>>, WrapType<u32>);
        meta_assert_eq!(Type, If<Type, False, WrapType<u32>, WrapType<f64>>, WrapType<f64>);

        meta_assert_eq!(Bool, If<Bool, False, False, True>, True);
        meta_assert_eq!(Bool, If<Bool, True, False, True>, False);

        meta_assert_eq!(Bool, If<Bool, False, LongRecursion<OneBillion>, False>, False);
        meta_assert_eq!(Bool, If<Bool, True, False, LongRecursion<OneBillion>>, False);
    }
}
