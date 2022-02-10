use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

// Can it be done w/o allocations?
pub fn group<A: Hash + Eq, B>(xs: Vec<(A, B)>) -> Vec<(A, Vec<B>)> {
    let mut map = HashMap::new();
    for (i, x) in xs {
        let e = map.entry(i).or_insert(Vec::new());
        e.push(x)
    }
    map.into_iter().collect()
}

pub fn find<A: Eq + Copy, B: Clone>(x: A, xs: &[(A, B)]) -> Option<B> {
    xs.iter()
        .find_map(|(y, d)| if *y == x { Some(d.clone()) } else { None })
}

pub fn show<A: Display>(x: A) -> String {
    format!("{}", x)
}

// const is a keyword :^)
pub fn fst<A: Clone, B>(x: A) -> impl Fn(B) -> A {
    move |_| x.clone()
}
