// https://github.com/insou22/typing-the-technical-interview-rust/blob/main/src/main.rs
// https://aphyr.com/posts/342-typing-the-technical-interview
use std::marker::PhantomData;
use std::ops::BitOr;
////////// List //////////

struct HNil;
struct HCons<X, Xs>(PhantomData<(X, Xs)>);

////////// UIDEquals //////////

use typenum::private::IsEqualPrivate;
use typenum::{B0, B1, Bit, Cmp, IsEqual, U, Unsigned};

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

impl<Needle: ParamValue, ThisHead: ParamValue, Tail: Contains<Needle>> Contains<Needle>
    for HCons<ThisHead, Tail>
where
    Needle::UID: Cmp<ThisHead::UID>,
    Needle::UID: IsEqualPrivate<ThisHead::UID, <Needle::UID as Cmp<ThisHead::UID>>::Output>,
    <Needle::UID as IsEqualPrivate<ThisHead::UID, <Needle::UID as Cmp<ThisHead::UID>>::Output>>::Output: BitOr<<Tail as Contains<Needle>>::Output>,
    <<Needle::UID as IsEqualPrivate<ThisHead::UID, <Needle::UID as Cmp<ThisHead::UID>>::Output>>::Output as BitOr<<Tail as Contains<Needle>>::Output>>::Output: Bit,
{
    type Output = <<(Needle, ThisHead) as UIDEquals>::Output as BitOr<<Tail as Contains<Needle>>::Output >>::Output;
}

////////// PrependIf //////////

trait PrependIf {
    type Output;
}

impl<TheHead, Tail> PrependIf for (B1, TheHead, Tail) {
    type Output = HCons<TheHead, Tail>;
}

impl<TheHead, Tail> PrependIf for (B0, TheHead, Tail) {
    type Output = Tail;
}

////////// Intersection //////////

trait Intersection<Rhs> {
    type Output;
}

impl<Rhs> Intersection<Rhs> for HNil {
    type Output = HNil;
}

impl<TheHead: ParamValue, Tail, Rhs, TailFilterOutput> Intersection<Rhs> for HCons<TheHead, Tail>
where
    Tail: Intersection<Rhs, Output = TailFilterOutput>,
    Rhs: Contains<TheHead>,
    (
        <Rhs as Contains<TheHead>>::Output,
        TheHead,
        TailFilterOutput,
    ): PrependIf,
{
    type Output = <(
        <Rhs as Contains<TheHead>>::Output,
        TheHead,
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
            .replace("nine_queens::", "")
    );
}
