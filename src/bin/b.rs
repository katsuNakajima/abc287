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
    let mut s = Vec::new();
    let mut t = Vec::new();
    let mut ans = 0;
    for _i in 0..n {
        let stmp = parse_line!(String);
        s.push(stmp);
    }
    for _i in 0..m {
        let ttmp = parse_line!(String);
        t.push(ttmp);
        // for x in s.clone() {
        //     let leng = x.len() - 3;
        //     let ss = x[leng..].to_string();
        //     if ss == t {
        //         ans += 1;
        //     }
        // }
    }
    for x in s {
        let leng = x.len() - 3;
        let ss = x[leng..].to_string();
        for y in t.clone() {
            if ss == y {
                ans += 1;
                break;
            }
        }
    }
    println!("{}", ans);
}
