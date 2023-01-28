use std::collections::HashMap;

#[allow(unused_macros)]
macro_rules! parse_line {
    ( $t:ty ) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            iter.next().unwrap().parse::<$t>().unwrap()
        }
    );

    ( $( $t:ty), +) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            ( $(iter.next().unwrap().parse::<$t>().unwrap()),* )
        }
    );
}

#[allow(unused_macros)]
macro_rules! read_line {
    () => {{
        let mut line = String::new();
        ::std::io::stdin().read_line(&mut line).unwrap();
        line.pop();
        line
    }};
}

#[allow(unused_macros)]
macro_rules! parse_vec {
    ( $t:ty ) => {{
        let mut line = String::new();
        ::std::io::stdin().read_line(&mut line).unwrap();
        let iter = line.split_whitespace();
        iter.map(|v| v.parse::<$t>().unwrap()).collect::<Vec<_>>()
    }};
}

fn main() {
    let (n, m) = parse_line!(usize, usize);
    let mut hm = HashMap::new();
    for _i in 0..m {
        let (u, v) = parse_line!(i32, i32);
        hm.entry(u).and_modify(|counter| *counter += 1).or_insert(1);
        hm.entry(v).and_modify(|counter| *counter += 1).or_insert(1);
    }
    // let cnt1 = hm.iter().filter(|x| *x.1 == 1).count();
    // let is_term2 = cnt1 == 2;
    let cnt2 = hm.iter().filter(|x| *x.1 <= 2).count();
    // let is_dim2 = (n - 2) = cnt2;
    let is_dim2 = cnt2 == n;
    let is_p1 = m == n - 1;
    let ans = if is_dim2 && is_p1 { "Yes" } else { "No" };
    println!("{}", ans);
}
