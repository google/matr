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
use crate::list::*;
use crate::r#type::*;

meta!{
    pub type TupleBasedTypePairListToList<
        S: Expr<TupleBasedTypePairList>
    >: Expr<List<Pair<Type, Type>>> =
        VisitTupleBasedTypePairList<List<Pair<Type, Type>>, S, TupleBasedTypePairListToListVisitor>;
}

mod internal {
    pub use super::super::internal::*;
    use crate::list::*;

    meta!{
        pub struct TupleBasedTypePairListToListVisitor: TupleBasedTypePairListVisitor<List<Pair<Type, Type>>> {
            type VisitEmptyTupleBasedTypePairList = EmptyList<Pair<Type, Type>>;
            type VisitCons<Elem: Expr<Pair<Type, Type>>, Tail: Expr<TupleBasedTypePairList>> = Cons<Pair<Type, Type>, Elem, TupleBasedTypePairListToList<Tail>>;
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;
    use super::super::*;
    use crate::list::*;

    #[test]
    fn tuple_based_list_to_list() {
        meta_assert_eq!(List<Pair<Type, Type>>,
            TupleBasedTypePairListToList<WrapTupleBasedTypePairList<()>>,
            meta_list!(<Pair<Type, Type>>[]));
        meta_assert_eq!(List<Pair<Type, Type>>,
            TupleBasedTypePairListToList<WrapTupleBasedTypePairList<((i32, &i32), ((u32, &u32), ()))>>,
            meta_list!(<Pair<Type, Type>>[
                ConsPair<Type, Type, WrapType<i32>, WrapType<&i32>>,
                ConsPair<Type, Type, WrapType<u32>, WrapType<&u32>>
            ]));
    }
}
