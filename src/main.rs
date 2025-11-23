#![recursion_limit = "1024"] // necessary to generate solutions past N = 6
// https://github.com/insou22/typing-the-technical-interview-rust/blob/main/src/main.rs
// https://aphyr.com/posts/342-typing-the-technical-interview
use std::marker::PhantomData;

////////// List //////////

struct Nil;
struct Cons<X, Xs>(PhantomData<(X, Xs)>);

////////// Head //////////

trait Head {
    type Output;
}

impl Head for Nil {
    type Output = Nil;
}

impl<TheHead, Tail> Head for Cons<TheHead, Tail> {
    type Output = TheHead;
}

////////// ListConcat //////////

trait ListConcat {
    type Output;
}

impl<RightList> ListConcat for (Nil, RightList) {
    type Output = RightList;
}

impl<LeftHead, LeftTail, RightList> ListConcat for (Cons<LeftHead, LeftTail>, RightList)
where
    (LeftTail, RightList): ListConcat,
{
    type Output = Cons<LeftHead, <(LeftTail, RightList) as ListConcat>::Output>;
}

////////// ListConcatAll //////////

trait ListConcatAll {
    type Output;
}

impl ListConcatAll for Nil {
    type Output = Nil;
}

impl<HeadList, TailLists> ListConcatAll for Cons<HeadList, TailLists>
where
    TailLists: ListConcatAll,
    (HeadList, <TailLists as ListConcatAll>::Output): ListConcat,
{
    type Output = <(HeadList, <TailLists as ListConcatAll>::Output) as ListConcat>::Output;
}

////////// Bool //////////

struct False;
struct True;

trait Bool {}

impl Bool for False {}
impl Bool for True {}

////////// AnyTrue //////////

trait ContainsTrue {
    type Output: Bool;
}

impl ContainsTrue for Nil {
    type Output = False;
}

impl<Tail> ContainsTrue for Cons<True, Tail> {
    type Output = True;
}

impl<Tail> ContainsTrue for Cons<False, Tail>
where
    Tail: ContainsTrue,
{
    type Output = <Tail as ContainsTrue>::Output;
}

////////// Not //////////

trait Not {
    type Output: Bool;
}

impl Not for False {
    type Output = True;
}

impl Not for True {
    type Output = False;
}

////////// Or //////////

trait Or {
    type Output: Bool;
}

impl Or for (True, True) {
    type Output = True;
}

impl Or for (True, False) {
    type Output = True;
}

impl Or for (False, True) {
    type Output = True;
}

impl Or for (False, False) {
    type Output = False;
}

////////// Nats //////////

struct Z;
struct S<N: Nat>(PhantomData<N>);

type N0 = Z;
type N1 = S<N0>;
type N2 = S<N1>;
type N3 = S<N2>;
type N4 = S<N3>;
type N5 = S<N4>;
type N6 = S<N5>;

trait Nat {}
impl Nat for Z {}
impl<N: Nat> Nat for S<N> {}

////////// PeanoEqual //////////

trait NatEqual {
    type Output: Bool;
}

impl NatEqual for (Z, Z) {
    type Output = True;
}

impl<N> NatEqual for (Z, S<N>)
where
    N: Nat,
{
    type Output = False;
}

impl<N> NatEqual for (S<N>, Z)
where
    N: Nat,
{
    type Output = False;
}

impl<N1, N2> NatEqual for (S<N1>, S<N2>)
where
    N1: Nat,
    N2: Nat,
    (N1, N2): NatEqual,
{
    type Output = <(N1, N2) as NatEqual>::Output;
}

////////// PeanoLT //////////

trait NatLessThan {
    type Output: Bool;
}

impl NatLessThan for (Z, Z) {
    type Output = False;
}

impl<N: Nat> NatLessThan for (S<N>, Z) {
    type Output = False;
}

impl<N: Nat> NatLessThan for (Z, S<N>) {
    type Output = True;
}

impl<N1, N2> NatLessThan for (S<N1>, S<N2>)
where
    N1: Nat,
    N2: Nat,
    (N1, N2): NatLessThan,
{
    type Output = <(N1, N2) as NatLessThan>::Output;
}

////////// PeanoAbsDiff //////////

trait NatAbsDiff {
    type Output: Nat;
}

impl NatAbsDiff for (Z, Z) {
    type Output = Z;
}

impl<N: Nat> NatAbsDiff for (Z, S<N>) {
    type Output = S<N>;
}

impl<N: Nat> NatAbsDiff for (S<N>, Z) {
    type Output = S<N>;
}

impl<N1, N2> NatAbsDiff for (S<N1>, S<N2>)
where
    N1: Nat,
    N2: Nat,
    (N1, N2): NatAbsDiff,
{
    type Output = <(N1, N2) as NatAbsDiff>::Output;
}

////////// Range //////////

trait Range {
    type Output;
}

impl Range for Z {
    type Output = Nil;
}

impl<N> Range for S<N>
where
    N: Nat + Range,
{
    type Output = Cons<N, <N as Range>::Output>;
}

////////// Higher order functions //////////

trait Apply<A> {
    type Output;
}

struct Conj1<L>(PhantomData<L>);

impl<X, L> Apply<X> for Conj1<L> {
    type Output = Cons<X, L>;
}

////////// Map //////////

trait Map {
    type Output;
}

impl<Function> Map for (Function, Nil) {
    type Output = Nil;
}

impl<Function, TheHead, Tail> Map for (Function, Cons<TheHead, Tail>)
where
    Function: Apply<TheHead>,
    (Function, Tail): Map,
{
    type Output = Cons<<Function as Apply<TheHead>>::Output, <(Function, Tail) as Map>::Output>;
}

////////// MapCat //////////

trait MapConcat {
    type Output;
}

impl<FunctionThatReturnsList, List> MapConcat for (FunctionThatReturnsList, List)
where
    (FunctionThatReturnsList, List): Map,
    <(FunctionThatReturnsList, List) as Map>::Output: ListConcatAll,
{
    type Output = <<(FunctionThatReturnsList, List) as Map>::Output as ListConcatAll>::Output;
}

////////// AppendIf //////////

trait PrependIf {
    type Output;
}

impl<TheHead, Tail> PrependIf for (True, TheHead, Tail) {
    type Output = Cons<TheHead, Tail>;
}

impl<TheHead, Tail> PrependIf for (False, TheHead, Tail) {
    type Output = Tail;
}

////////// Filter //////////

trait Filter {
    type Output;
}

impl<FilterFunction> Filter for (FilterFunction, Nil) {
    type Output = Nil;
}

impl<FilterFunction, TheHead, Tail, FilterOutput> Filter for (FilterFunction, Cons<TheHead, Tail>)
where
    FilterFunction: Apply<TheHead>,
    (FilterFunction, Tail): Filter<Output = FilterOutput>,
    (
        <FilterFunction as Apply<TheHead>>::Output,
        TheHead,
        FilterOutput,
    ): PrependIf,
{
    type Output = <(
        <FilterFunction as Apply<TheHead>>::Output,
        TheHead,
        <(FilterFunction, Tail) as Filter>::Output,
    ) as PrependIf>::Output;
}

////////// Queen //////////

struct Queen<X, Y>(PhantomData<(X, Y)>);
struct Queen1<X>(PhantomData<X>);

impl<X: Nat, Y> Apply<Y> for Queen1<X> {
    type Output = Queen<X, Y>;
}

////////// QueensInRow //////////

trait QueensInRow {
    type Output;
}

impl<N, X> QueensInRow for (N, X)
where
    N: Range,
    (Queen1<X>, <N as Range>::Output): Map,
{
    type Output = <(Queen1<X>, <N as Range>::Output) as Map>::Output;
}

////////// Threatens //////////

trait Threatens {
    type Output: Bool;
}

impl<Ax, Ay, Bx, By> Threatens for (Queen<Ax, Ay>, Queen<Bx, By>)
where
    (Ax, Bx): NatEqual,
    (Ay, By): NatEqual,
    (Ax, Bx): NatAbsDiff,
    (Ay, By): NatAbsDiff,
    (
        <(Ax, Bx) as NatEqual>::Output,
        <(Ay, By) as NatEqual>::Output,
    ): Or,
    (
        <(Ax, Bx) as NatAbsDiff>::Output,
        <(Ay, By) as NatAbsDiff>::Output,
    ): NatEqual,
    (
        <(
            <(Ax, Bx) as NatEqual>::Output,
            <(Ay, By) as NatEqual>::Output,
        ) as Or>::Output,
        <(
            <(Ax, Bx) as NatAbsDiff>::Output,
            <(Ay, By) as NatAbsDiff>::Output,
        ) as NatEqual>::Output,
    ): Or,
{
    type Output = <(
        <(
            <(Ax, Bx) as NatEqual>::Output,
            <(Ay, By) as NatEqual>::Output,
        ) as Or>::Output,
        <(
            <(Ax, Bx) as NatAbsDiff>::Output,
            <(Ay, By) as NatAbsDiff>::Output,
        ) as NatEqual>::Output,
    ) as Or>::Output;
}

struct Threatens1<A>(PhantomData<A>);
impl<Qa, Qb> Apply<Qb> for Threatens1<Qa>
where
    (Qa, Qb): Threatens,
{
    type Output = <(Qa, Qb) as Threatens>::Output;
}

////////// Safe //////////

trait Safe {
    type Output: Bool;
}

impl<C, Q> Safe for (C, Q)
where
    (Threatens1<Q>, C): Map,
    <(Threatens1<Q>, C) as Map>::Output: ContainsTrue,
    <<(Threatens1<Q>, C) as Map>::Output as ContainsTrue>::Output: Not,
{
    type Output = <<<(Threatens1<Q>, C) as Map>::Output as ContainsTrue>::Output as Not>::Output;
}

struct Safe1<C>(PhantomData<C>);
impl<C, Q> Apply<Q> for Safe1<C>
where
    (C, Q): Safe,
{
    type Output = <(C, Q) as Safe>::Output;
}

////////// AddQueen //////////

trait AddQueen {
    type Output;
}

impl<N, X, C> AddQueen for (N, X, C)
where
    (N, X): QueensInRow,
    (Safe1<C>, <(N, X) as QueensInRow>::Output): Filter,
    (
        Conj1<C>,
        <(Safe1<C>, <(N, X) as QueensInRow>::Output) as Filter>::Output,
    ): Map,
{
    type Output = <(
        Conj1<C>,
        <(Safe1<C>, <(N, X) as QueensInRow>::Output) as Filter>::Output,
    ) as Map>::Output;
}

struct AddQueen2<N, X>(PhantomData<(N, X)>);
impl<N, X, C> Apply<C> for AddQueen2<N, X>
where
    (N, X, C): AddQueen,
{
    type Output = <(N, X, C) as AddQueen>::Output;
}

trait AddQueenToAll {
    type Output;
}

impl<N, X, Cs> AddQueenToAll for (N, X, Cs)
where
    (AddQueen2<N, X>, Cs): MapConcat,
{
    type Output = <(AddQueen2<N, X>, Cs) as MapConcat>::Output;
}

////////// AddQueensIf //////////

trait AddQueensIf {
    type Output;
}

impl<N, X, Cs> AddQueensIf for (False, N, X, Cs) {
    type Output = Cs;
}

impl<N, X, Cs, AddQueenToAllOutput> AddQueensIf for (True, N, X, Cs)
where
    X: Nat,
    (N, X, Cs): AddQueenToAll<Output = AddQueenToAllOutput>,
    (N, S<X>, AddQueenToAllOutput): AddQueens,
{
    type Output = <(N, S<X>, <(N, X, Cs) as AddQueenToAll>::Output) as AddQueens>::Output;
}

trait AddQueens {
    type Output;
}

impl<N, X, Cs, PeanoLTOutput> AddQueens for (N, X, Cs)
where
    (X, N): NatLessThan<Output = PeanoLTOutput>,
    (PeanoLTOutput, N, X, Cs): AddQueensIf,
{
    type Output = <(<(X, N) as NatLessThan>::Output, N, X, Cs) as AddQueensIf>::Output;
}

////////// Solution //////////

trait Solution {
    type Output;
}

impl<N, AddQueensIfOutput> Solution for N
where
    N: Nat,
    (Z, N): NatLessThan,
    (<(Z, N) as NatLessThan>::Output, N, Z, Cons<Nil, Nil>):
        AddQueensIf<Output = AddQueensIfOutput>,
    AddQueensIfOutput: Head,
{
    type Output = <<(N, Z, Cons<Nil, Nil>) as AddQueens>::Output as Head>::Output;
}

////////// Name Equals //////////

use typenum::private::IsEqualPrivate;
use typenum::{Bit, Cmp, Integer, IsEqual, Unsigned, U};

pub trait ParamValue {
    type UID: Unsigned;
}

trait NameEquals {
    type Output: Bit;
}

impl<Left: ParamValue, Right: ParamValue> NameEquals for (Left, Right)
where
    Left::UID: Cmp<Right::UID>,
    Left::UID: IsEqualPrivate<Right::UID, <Left::UID as Cmp<Right::UID>>::Output>,
{
    type Output = <Left::UID as IsEqual<Right::UID>>::Output;
}

////////// Params //////////

#[derive(Clone)]
struct Param1;
impl ParamValue for Param1{
  type UID = U<1>;
}

////////// Reify //////////

fn main() {
    type List1 = Cons<Elem0, Cons<Elem1, Cons<Elem2, Nil>>>;
    type List2 = Cons<Elem2, Cons<Elem3, Cons<Elem4, Nil>>>;
    println!(
        "{}",
        std::any::type_name::<<N6 as Solution>::Output>().replace("nine_queens::", "")
    );
}
