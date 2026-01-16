// Tracking Issue for const_cmp of std::any::TypeId:
// https://github.com/rust-lang/rust/issues/143800
// Solution with abusing auto-deref specialization:
// https://stackoverflow.com/a/77548783
// Solution with procedural macro which injects a Uid field containing a typenum number:
// https://users.rust-lang.org/t/compile-time-check-if-two-types-are-equal/59521
// Type level programming examples:
// https://github.com/insou22/typing-the-technical-interview-rust/blob/main/src/main.rs
// https://aphyr.com/posts/342-typing-the-technical-interview
use frunk::hlist::{HCons, HNil};
use frunk::{HList, hlist};
use std::ops::BitOr;
use typenum::{B0, B1, Bit, IsEqual, U, Unsigned};

pub trait ParamValue {
  type UID: Unsigned;
}

////////// Contains //////////

trait Contains<Needle: ParamValue> {
  type IsContained: Bit;
}

impl<Needle: ParamValue> Contains<Needle> for HNil {
  type IsContained = B0;
}

impl<Needle: ParamValue, Head: ParamValue, Tail: Contains<Needle>> Contains<Needle> for HCons<Head, Tail>
where
  Needle::UID: IsEqual<Head::UID>,
  <Needle::UID as IsEqual<Head::UID>>::Output: BitOr<<Tail as Contains<Needle>>::IsContained>,
  <<Needle::UID as IsEqual<Head::UID>>::Output as BitOr<<Tail as Contains<Needle>>::IsContained>>::Output: Bit,
{
  type IsContained =
    <<Needle::UID as IsEqual<Head::UID>>::Output as BitOr<<Tail as Contains<Needle>>::IsContained>>::Output;
}

////////// Filter //////////

trait Filter<Head, Tail> {
  type Filtered;
  fn filter(head: Head, tail: Tail) -> Self::Filtered;
}

impl<Head, Tail> Filter<Head, Tail> for B1 {
  type Filtered = HCons<Head, Tail>;

  #[inline(always)]
  fn filter(head: Head, tail: Tail) -> Self::Filtered {
    HCons { head, tail }
  }
}

impl<Head, Tail> Filter<Head, Tail> for B0 {
  type Filtered = Tail;

  #[inline(always)]
  fn filter(_head: Head, tail: Tail) -> Self::Filtered {
    tail
  }
}

////////// Intersection //////////

trait Intersect<RHS> {
  type Intersection;

  fn intersect(self, rhs: RHS) -> Self::Intersection;
}

impl<RHS> Intersect<RHS> for HNil {
  type Intersection = HNil;

  #[inline(always)]
  fn intersect(self, rhs: RHS) -> Self::Intersection {
    HNil
  }
}

impl<Head: ParamValue, Tail: Intersect<RHS>, RHS: Contains<Head>> Intersect<RHS> for HCons<Head, Tail>
where
  <RHS as Contains<Head>>::IsContained: Filter<Head, <Tail as Intersect<RHS>>::Intersection>,
{
  type Intersection =
    <<RHS as Contains<Head>>::IsContained as Filter<Head, <Tail as Intersect<RHS>>::Intersection>>::Filtered;

  #[inline(always)]
  fn intersect(self, rhs: RHS) -> Self::Intersection {
    let intersected_tail = self.tail.intersect(rhs);
    <<RHS as Contains<Head>>::IsContained as Filter<Head, <Tail as Intersect<RHS>>::Intersection>>::filter(
      self.head,
      intersected_tail,
    )
  }
}

////////// Params //////////

#[derive(Debug)]
struct Param0;
impl ParamValue for Param0 {
  type UID = U<0>;
}

#[derive(Debug)]
struct Param1;
impl ParamValue for Param1 {
  type UID = U<1>;
}

#[derive(Debug)]
struct Param2;
impl ParamValue for Param2 {
  type UID = U<2>;
}

#[derive(Debug)]
struct Param3;
impl ParamValue for Param3 {
  type UID = U<3>;
}

////////// Reify //////////

fn main() {
  let list1 = hlist![Param0, Param1, Param2];
  let list2 = hlist![Param2, Param3];
  println!("{:?}", list1.intersect(list2));

  type List1 = HList![Param0, Param1, Param2];
  type List2 = HList![Param1, Param2, Param3];
  println!(
    "{}",
    std::any::type_name::<<List1 as Intersect<List2>>::Intersection>()
      .replace("frunk_core::hlist::", "")
      .replace("hlist_intersection_assuming_manual_id_assignment::", "")
  );
}
