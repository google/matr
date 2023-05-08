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
    pub type ListToTupleBasedTypeList<
        L: Expr<List<Type>>
    >: Expr<TupleBasedTypeList> =
        VisitList<Type, TupleBasedTypeList, L, ListToTupleBasedTypeListVisitor>;
}

mod internal {
    pub use super::super::internal::*;
    use crate::list::*;

    meta!{
        pub struct ListToTupleBasedTypeListVisitor: ListVisitor<Type, TupleBasedTypeList> {
            type VisitEmptyList = WrapTupleBasedTypeList<()>;
            type VisitCons<Elem: Expr<Type>, Tail: Expr<List<Type>>> = WrapTupleBasedTypeList<(UnwrapType<Elem>, UnwrapTupleBasedTypeList<ListToTupleBasedTypeList<Tail>>)>;
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::*;
    use super::super::*;

    #[test]
    fn list_to_tuple_based_list() {
        meta_assert_eq!(TupleBasedTypeList,
            ListToTupleBasedTypeList<type_list![]>,
            WrapTupleBasedTypeList<()>);
        meta_assert_eq!(TupleBasedTypeList,
            ListToTupleBasedTypeList<type_list![i32, u32, i32]>,
            WrapTupleBasedTypeList<(i32, (u32, (i32, ())))>);
    }
}
