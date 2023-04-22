// This macro allows to define meta-types and meta-functions using a more concise and readable
// syntax. Using this is optional, but recommended.
//
// A single meta!{} block can contain multiple declarations. 2 kinds of declarations are supported:
// struct and type.
//
// Struct declarations are used to declare an empty struct type (well, empty except for the
// PhantomData fields, see below) together with an impl block for that type and a trait. The body in
// the declaration is used as body of the impl block.
// For example:
//
// meta!{
//     pub struct IsInListVisitor<
//         K: EqualityComparableKind,
//         X: Expr<K>
//     >: ListVisitor<K, Bool> {
//         type VisitEmptyList = False;
//         type VisitCons<Elem: Expr<K>, Tail: Expr<List<K>>> = Or<Equals<K, Elem, X>, IsInList<K, X, Tail>>;
//     }
// }
//
// Expands to:
//
// #[allow(non_snake_case)]
// pub struct IsInListVisitor<
//     K: EqualityComparableKind,
//     X: Expr<K>
// > {
//     K: std::marker::PhantomData<K>,
//     X: std::marker::PhantomData<X>,
// }
// impl<
//     K: EqualityComparableKind,
//     X: Expr<K>
// > ListVisitor<K, Bool> for IsInListVisitor<K, X> {
//     type VisitEmptyList = False;
//     type VisitCons<Elem: Expr<K>, Tail: Expr<List<K>>> = Or<Equals<K, Elem, X>, IsInList<K, X, Tail>>;
// }
//
// Type declarations look like generic Rust type declarations but they have trait bounds on the type
// parameter and on the result type (while Rust normally ignores the former and does not allow the
// latter). Despite actually looking like a type declaration, they are expanded into a struct
// metafunction so that the Rust compiler checks those trait bounds and to support lazy evaluation.
// For these, the result type is expected to be of the form Expr<...>.
// For example:
//
// meta!{
//     pub type And<
//         X: Expr<Bool>,
//         Y: Expr<Bool>
//     >: Expr<Bool> =
//         If<Bool, X, Y, False>;
// }
//
// is equivalent to:
//
// meta!{
//     pub struct And<
//         X: Expr<Bool>,
//         Y: Expr<Bool>
//     >: Expr<Bool> {
//         type Eval = <If<Bool, X, Y, False> as Expr<Bool>>::Eval;
//     }
// }
//
// The type arguments are optional, e.g. the following is ok:
//
// meta!{
//     pub struct Zero: Expr<USize> {
//         type Eval = ZeroValue;
//     }
// }
//
// Known limitations: if the code is not indented as above and two `>`s are adjacent before the
// result type (e.g. `And<X: Expr<Bool>, Y: Expr<Bool>>: Expr<Bool>`), Rust will parse the `>>` as a
// single token and a compilation error will be reported.
// To avoid this it's recommended to indent the declarations as in the examples shown (or at least
// add a space between the two `>`s).
#[macro_export]
macro_rules! meta {
    {} => {};
    {
        $Vis:vis type $Fn:ident $(<
            $(
                $Args:ident $(:
                    $(~$Arg1Bound1Const:ident)? $($Arg1Bound1Idents:ident $(<$Arg1Bound1IdentsTypeArg1:ty $(, $Arg1Bound1IdentsTypeArgs:ty)* >)?)::*
                    $(+
                        $(~$Arg1BoundsConst:ident)? $($Arg1BoundsIdents:ident $(<$Arg1BoundsIdentsTypeArg1:ty $(, $Arg1BoundsIdentsTypeArgs:ty)* >)?)::*
                    )*
                )?
            ),*
        >)?: $($ExprIdent:ident)::+ <$ReturnK:ty> = $Body:ty;
        $($Tail:tt)*
    } => {
        meta!{
            $Vis struct $Fn $(<
                $(
                    $Args $(:
                        $(~$Arg1Bound1Const)* $($Arg1Bound1Idents $(<$Arg1Bound1IdentsTypeArg1 $(, $Arg1Bound1IdentsTypeArgs)* >)?)::*
                        $(+
                            $(~$Arg1BoundsConst)? $($Arg1BoundsIdents $(<$Arg1BoundsIdentsTypeArg1 $(, $Arg1BoundsIdentsTypeArgs)* >)?)::*
                        )*
                    )*
                ),*
            >)*: $($ExprIdent)::* <$ReturnK> {
                type Eval = <$Body as $($ExprIdent)::*<$ReturnK> >::Eval;
            }
            $($Tail)*
        }
    };

    {
        $Vis:vis struct $Fn:ident $(<
            $(
                $Args:ident $(:
                    $(~$Arg1Bound1Const:ident)? $($Arg1Bound1Idents:ident $(<$Arg1Bound1IdentsTypeArg1:ty $(, $Arg1Bound1IdentsTypeArgs:ty)* >)?)::*
                    $(+
                        $(~$Arg1BoundsConst:ident)? $($Arg1BoundsIdents:ident $(<$Arg1BoundsIdentsTypeArg1:ty $(, $Arg1BoundsIdentsTypeArgs:ty)* >)?)::*
                    )*
                )?
            ),*
        // Semantically `$ReturnIdents1:ident $($ReturnIdents2:ident)?` is really
        // `$($ConstQualifier:ident)? $ReturnIdents:ident` but Rust doesn't support matching that
        // directly.
        >)?: $($ReturnIdents1:ident $($ReturnIdents2:ident)? $(<$($ReturnIdentsTypeArgs:ty),*>)?)::* {
            $($Body:tt)*
        }
        $($Tail:tt)*
    } => {
        #[allow(non_snake_case)]
        $Vis struct $Fn $(<
                $(
                    $Args $(:
                        $(~$Arg1Bound1Const)* $($Arg1Bound1Idents $(<$Arg1Bound1IdentsTypeArg1 $(, $Arg1Bound1IdentsTypeArgs)* >)?)::*
                        $(+
                            $(~$Arg1BoundsConst)* $($Arg1BoundsIdents $(<$Arg1BoundsIdentsTypeArg1 $(, $Arg1BoundsIdentsTypeArgs)* >)?)::*
                        )*
                    )*
                ),*
        >)* {
            $(
                $($Args: std::marker::PhantomData<$Args>),*
            )*
        }

        impl $(<
            $(
                $Args $(:
                    $(~$Arg1Bound1Const)* $($Arg1Bound1Idents $(<$Arg1Bound1IdentsTypeArg1 $(, $Arg1Bound1IdentsTypeArgs)* >)?)::*
                    $(+
                        $(~$Arg1BoundsConst)* $($Arg1BoundsIdents $(<$Arg1BoundsIdentsTypeArg1 $(, $Arg1BoundsIdentsTypeArgs)* >)?)::*
                    )*
                )*
            ),*
        >)* $($ReturnIdents1 $($ReturnIdents2)* $(<$($ReturnIdentsTypeArgs),*>)*)::* for $Fn $(<$($Args),*>)* {
            $($Body)*
        }

        meta! {
            $($Tail)*
        }
    };
}
pub use meta;
