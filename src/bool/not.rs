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

use crate::*;

meta!{
    pub type Not<
        X: Expr<Bool>
    >: Expr<Bool> =
        If<Bool, X, False, True>;
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;

    #[test]
    fn test_not() {
        meta_assert_eq!(Bool, Not<False>, True);
        meta_assert_eq!(Bool, Not<True>, False);
        meta_assert_eq!(Bool, Not<Not<True>>, True);
    }
}
