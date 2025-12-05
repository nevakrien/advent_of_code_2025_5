
use std::ops::RangeInclusive;

fn consume_digits(input:&[u8])->(u64,&[u8]){
    let mut sum = 0u64;

    for(i,b) in input.iter().enumerate(){
        let d = *b as i8-b'0' as i8;
        if d<0 || d>9{
            return (sum,&input[i..])
        }
        sum=sum*10+d as u64;
    }
    (sum,&[])
}

fn consume_dash(input:&[u8])->&[u8]{
    if input.get(0)!=Some(&b'-'){
        panic!("expected '-'");
    }
    &input[1..]
}

fn parse_pair(input:&[u8])->(u64,u64,&[u8]){
    let (first,rest) = consume_digits(input);
    let rest = consume_dash(rest);
    let (second,rest) = consume_digits(rest);
    (first,second,rest)
}

fn check_spaces(input:&[u8])->(&[u8],bool){
    assert!(input[0]==b'\n');
    if input[1]==b'\n'{
        (&input[2..],true)
    }else{
        (&input[1..],false)
    }
}

fn parse_start(mut input:&[u8])->(&[u8],Vec<RangeInclusive<u64>>){
    let mut ans = Vec::new();
    loop{
        let (start,end,rest) = parse_pair(input);
        ans.push(RangeInclusive::new(start,end));
        let (rest,stop) = check_spaces(rest);
        input=rest;
        if stop {
            return (input,ans)
        }
    }
}


fn main() {
    let input = include_str!("input.txt").as_bytes();
    let (mut cur,mut ranges) = parse_start(input);
    ranges.sort_unstable_by_key(|r| *r.start());
    ranges = ranges.into_iter().fold(Vec::new(),|mut v,r|{
        if let Some(old) = v.last_mut(){
            let s = *old.start().min(r.start());
            let e = *old.end().max(r.end());
            
            if old.end() >= r.start() {
                *old = RangeInclusive::new(s,e);
                return v;
            }
        }

        v.push(r);
        v
    });
    println!("ranges {ranges:?}");

    let mut ans =0;

    while !cur.is_empty(){
        let (x,rest) = consume_digits(cur);
        cur = rest;
        if cur.get(0)==Some(&b'\n'){
            cur = &cur[1..];
        }

        let contains = match ranges.binary_search_by(|r| r.start().cmp(&x)) {
            Ok(_) =>true,
            Err(i) => {
                let mut ans = false;
                if let Some(r) = ranges.get(i){
                    ans|=r.contains(&x);
                }
                if let Some(r) = ranges.get(i.saturating_sub(1)){
                    ans|=r.contains(&x);
                }
                ans
            },
        };
        // println!("{x} {contains}");

        if contains {
            ans+=1;
        }
    }

    // println!("tree is\n{tree:#?}");
    println!("part1 is {ans}");
    println!("part2 is {}",ranges.iter().map(|r| 1+r.end()-r.start()).sum::<u64>())

}