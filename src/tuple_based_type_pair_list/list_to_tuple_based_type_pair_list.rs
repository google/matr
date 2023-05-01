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
    pub type ListToTupleBasedTypePairList<
        L: Expr<List<Pair<Type, Type>>>
    >: Expr<TupleBasedTypePairList> =
        VisitList<Pair<Type, Type>, TupleBasedTypePairList, L, ListToTupleBasedTypePairListVisitor>;
}

mod internal {
    pub use super::super::internal::*;
    
    meta!{
        pub struct ListToTupleBasedTypePairListVisitor: ListVisitor<Pair<Type, Type>, TupleBasedTypePairList> {
            type VisitEmptyList = WrapTupleBasedTypePairList<()>;
            type VisitCons<Elem: Expr<Pair<Type, Type>>, Tail: Expr<List<Pair<Type, Type>>>> = WrapTupleBasedTypePairList<(
                    (
                        UnwrapType<GetFirst<Type, Type, Elem>>,
                        UnwrapType<GetSecond<Type, Type, Elem>>
                    ),
                    UnwrapTupleBasedTypePairList<ListToTupleBasedTypePairList<Tail>>
            )>;
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;

    #[test]
    fn list_to_tuple_based_list() {
        meta_assert_eq!(TupleBasedTypePairList,
            ListToTupleBasedTypePairList<meta_list!(<Pair<Type, Type>>[])>,
            WrapTupleBasedTypePairList<()>);
        meta_assert_eq!(TupleBasedTypePairList,
            ListToTupleBasedTypePairList<meta_list!(<Pair<Type, Type>>[
                ConsPair<Type, Type, WrapType<i32>, WrapType<&i32>>,
                ConsPair<Type, Type, WrapType<u32>, WrapType<&u32>>,
                ConsPair<Type, Type, WrapType<i32>, WrapType<&i32>>
            ])>,
            WrapTupleBasedTypePairList<((i32, &i32), ((u32, &u32), ((i32, &i32), ())))>);
    }
}
