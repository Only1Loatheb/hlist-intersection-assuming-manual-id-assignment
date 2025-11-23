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

impl<X, Xs> Head for Cons<X, Xs> {
    type Output = X;
}


////////// ListConcat //////////

trait ListConcat {
    type Output;
}

impl<L2> ListConcat for (Nil, L2) {
    type Output = L2;
}

impl<X, Xs, L2> ListConcat for (Cons<X, Xs>, L2)
where
    (Xs, L2): ListConcat,
{
    type Output = Cons<X, <(Xs, L2) as ListConcat>::Output>;
}


////////// ListConcatAll //////////

trait ListConcatAll {
    type Output;
}

impl ListConcatAll for Nil {
    type Output = Nil;
}

impl<L, Ls> ListConcatAll for Cons<L, Ls>
where
    Ls: ListConcatAll,
    (L, <Ls as ListConcatAll>::Output): ListConcat,
{
    type Output = <(L, <Ls as ListConcatAll>::Output) as ListConcat>::Output;
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

impl<L> ContainsTrue for Cons<True, L> {
    type Output = True;
}

impl<L> ContainsTrue for Cons<False, L>
where
    L: ContainsTrue,
{
    type Output = <L as ContainsTrue>::Output;
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

impl<F> Map for (F, Nil) {
    type Output = Nil;
}

impl<F, X, Xs> Map for (F, Cons<X, Xs>)
where
    F:  Apply<X>,
    (F, Xs): Map,
{
    type Output = Cons<<F as Apply<X>>::Output, <(F, Xs) as Map>::Output>;
}


////////// MapCat //////////

trait MapConcat {
    type Output;
}

impl<F, L> MapConcat for (F, L)
where
    (F, L): Map,
    <(F, L) as Map>::Output: ListConcatAll,
{
    type Output = <<(F, L) as Map>::Output as ListConcatAll>::Output;
}


////////// AppendIf //////////

trait PrependIf {
    type Output;
}

impl<X, Ys> PrependIf for (True, X, Ys) {
    type Output = Cons<X, Ys>;
}

impl<X, Ys> PrependIf for (False, X, Ys) {
    type Output = Ys;
}


////////// Filter //////////

trait Filter {
    type Output;
}

impl<F> Filter for (F, Nil) {
    type Output = Nil;
}

impl<F, X, Xs, FilterOutput> Filter for (F, Cons<X, Xs>)
where
    F: Apply<X>,
    (F, Xs): Filter<Output = FilterOutput>,
    (<F as Apply<X>>::Output, X, FilterOutput): PrependIf,
{
    type Output = <(<F as Apply<X>>::Output, X, <(F, Xs) as Filter>::Output) as PrependIf>::Output;
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
    (<(Ax, Bx) as NatEqual>::Output, <(Ay, By) as NatEqual>::Output): Or,
    (<(Ax, Bx) as NatAbsDiff>::Output, <(Ay, By) as NatAbsDiff>::Output): NatEqual,
    (<(<(Ax, Bx) as NatEqual>::Output, <(Ay, By) as NatEqual>::Output) as Or>::Output, <(<(Ax, Bx) as NatAbsDiff>::Output, <(Ay, By) as NatAbsDiff>::Output) as NatEqual>::Output): Or,
{
    type Output = <
        (
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
    (  Threatens1<Q>, C): Map,
    <( Threatens1<Q>, C) as Map>::Output: ContainsTrue,
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
    (Conj1<C>, <(Safe1<C>, <(N, X) as QueensInRow>::Output) as Filter>::Output): Map,
{
    type Output = <(Conj1<C>, <(Safe1<C>, <(N, X) as QueensInRow>::Output) as Filter>::Output) as Map>::Output;
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
    (<(Z, N) as NatLessThan>::Output, N, Z, Cons<Nil, Nil>): AddQueensIf<Output = AddQueensIfOutput>,
    AddQueensIfOutput: Head,
{
    type Output = <<(N, Z, Cons<Nil, Nil>) as AddQueens>::Output as Head>::Output;
}


////////// Reify //////////

fn main() {
    println!("{}", std::any::type_name::< <N6 as Solution>::Output >().replace("nine_queens::", ""));
}
