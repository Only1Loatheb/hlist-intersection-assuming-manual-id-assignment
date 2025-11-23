// Tracking Issue for const_cmp of std::any::TypeId:
// https://github.com/rust-lang/rust/issues/143800
// Solution with abusing auto-deref specialization:
// https://stackoverflow.com/a/77548783
// Solution with procedural macro which injects a Uid field containing a typenum number:
// https://users.rust-lang.org/t/compile-time-check-if-two-types-are-equal/59521
// Type level programming examples:
// https://github.com/insou22/typing-the-technical-interview-rust/blob/main/src/main.rs
// https://aphyr.com/posts/342-typing-the-technical-interview
use std::ops::BitOr;
use frunk::{HCons, HNil};
use typenum::private::IsEqualPrivate;
use typenum::{B0, B1, Bit, Cmp, IsEqual, U, Unsigned};

////////// UIDEquals //////////

pub trait ParamValue {
    type UID: Unsigned;
}

trait UIDEquals {
    type Output: Bit;
}

impl<Left: ParamValue, Right: ParamValue> UIDEquals for (Left, Right)
where
    Left::UID: Cmp<Right::UID>,
    Left::UID: IsEqualPrivate<Right::UID, <Left::UID as Cmp<Right::UID>>::Output>,
{
    type Output = <Left::UID as IsEqual<Right::UID>>::Output;
}

////////// Contains //////////

trait Contains<Needle: ParamValue> {
    type Output: Bit;
}

impl<Needle: ParamValue> Contains<Needle> for HNil {
    type Output = B0;
}

impl<Needle: ParamValue, Head: ParamValue, Tail: Contains<Needle>> Contains<Needle>
    for HCons<Head, Tail>
where
    Needle::UID: Cmp<Head::UID>,
    Needle::UID: IsEqualPrivate<Head::UID, <Needle::UID as Cmp<Head::UID>>::Output>,
    <Needle::UID as IsEqualPrivate<Head::UID, <Needle::UID as Cmp<Head::UID>>::Output>>::Output: BitOr<<Tail as Contains<Needle>>::Output>,
    <<Needle::UID as IsEqualPrivate<Head::UID, <Needle::UID as Cmp<Head::UID>>::Output>>::Output as BitOr<<Tail as Contains<Needle>>::Output>>::Output: Bit,
{
    type Output = <<(Needle, Head) as UIDEquals>::Output as BitOr<<Tail as Contains<Needle>>::Output >>::Output;
}

////////// PrependIf //////////

trait PrependIf {
    type Output;
}

impl<Head, Tail> PrependIf for (B1, Head, Tail) {
    type Output = HCons<Head, Tail>;
}

impl<Head, Tail> PrependIf for (B0, Head, Tail) {
    type Output = Tail;
}

////////// Intersection //////////

trait Intersection<Rhs> {
    type Output;
}

impl<Rhs> Intersection<Rhs> for HNil {
    type Output = HNil;
}

impl<Head: ParamValue, Tail, Rhs, TailFilterOutput> Intersection<Rhs> for HCons<Head, Tail>
where
    Tail: Intersection<Rhs, Output = TailFilterOutput>,
    Rhs: Contains<Head>,
    (
        <Rhs as Contains<Head>>::Output,
        Head,
        TailFilterOutput,
    ): PrependIf,
{
    type Output = <(
        <Rhs as Contains<Head>>::Output,
        Head,
        <Tail as Intersection<Rhs>>::Output,
    ) as PrependIf>::Output;
}

////////// Params //////////

#[derive(Clone)]
struct Param0;
impl ParamValue for Param0 {
    type UID = U<0>;
}

#[derive(Clone)]
struct Param1;
impl ParamValue for Param1 {
    type UID = U<1>;
}

#[derive(Clone)]
struct Param2;
impl ParamValue for Param2 {
    type UID = U<2>;
}

#[derive(Clone)]
struct Param3;
impl ParamValue for Param3 {
    type UID = U<3>;
}

////////// Reify //////////

fn main() {
    type List1 = HCons<Param0, HCons<Param1, HCons<Param2, HNil>>>;
    type List2 = HCons<Param2, HCons<Param3, HNil>>;
    println!(
        "{}",
        std::any::type_name::<<List1 as Intersection<List2>>::Output>()
            .replace("frunk_core::hlist::", "")
            .replace("hlist_intersection_assuming_manual_id_assignment::", "")
    );
}
