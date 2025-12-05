use std::io::BufRead;
use std::rc::Rc;

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

fn parse_start(mut input:&[u8])->(&[u8],PNode){
    let mut ans = None;
    loop{
        let (start,end,rest) = parse_pair(input);
        ans = insert_to_range(ans,start,end);
        let (rest,stop) = check_spaces(rest);
        input=rest;
        if stop {
            return (input,ans)
        }
    }
}

pub struct Node{
    me:(u64,u64),
    less:Option<Box<Node>>,
    more:Option<Box<Node>>,
}

impl Node{
    pub fn new_empty(me:(u64,u64))->Box<Self>{
        Box::new(Self{
            me,less:None,more:None
        })
    }


}

type PNode = Option<Box<Node>>;

pub fn insert_to_range(op:PNode,start:u64,end:u64)->PNode{
    let Some(mut cur) = op else {
        return Some(Node::new_empty((start,end)))
    };

    if end<cur.me.0 {
        cur.less = insert_to_range(cur.less,start,end);
        return Some(cur);
    }

    if start>cur.me.1 {
        cur.more = insert_to_range(cur.more,start,end);
        return Some(cur);
    }


    //we could do better here but meh

    if start<=cur.me.0 {
        cur.me.0=start;
    }

    if cur.me.1<=end {
        cur.me.1=end;
    }

    Some(cur)
}

pub fn tree_contains(op:&PNode,x:u64)->bool{
    let Some(node) = op else {
        return false;
    };

    if x < node.me.0 {
        return tree_contains(&node.less,x)
    }

    if x > node.me.1 {
        return tree_contains(&node.more,x)
    }

    true
}

fn main() {
    let input = include_str!("input.txt").as_bytes();
    let (mut cur,tree) = parse_start(input);

    let mut ans = 0;

    while !cur.is_empty(){
        let (x,rest) = consume_digits(cur);
        cur = rest;
        if cur.get(0)==Some(&b'\n'){
            cur = &cur[1..];
        }

        let contains = tree_contains(&tree,x);
        println!("{x} {contains}");

        if contains {
            ans+=1;
        }
    }

    println!("ans is {ans}")

}
