
fn consume_digits(input:&[u8])->(i64,&[u8]){
    let mut sum = 0i64;

    for(i,b) in input.iter().enumerate(){
        let d = *b as i8-b'0' as i8;
        if d<0 || d>9{
            return (sum,&input[i..])
        }
        sum=sum*10+d as i64;
    }
    (sum,&[])
}

fn consume_dash(input:&[u8])->&[u8]{
    if input.get(0)!=Some(&b'-'){
        panic!("expected '-'");
    }
    &input[1..]
}

fn parse_pair(input:&[u8])->(i64,i64,&[u8]){
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
        println!("after inserting [{start} {end}]");
        println!("{ans:#?}");

        let (rest,stop) = check_spaces(rest);
        input=rest;
        if stop {
            return (input,ans)
        }
    }
}

#[derive(Debug)]
pub struct Node{
    me:(i64,i64),
    less:Option<Box<Node>>,
    more:Option<Box<Node>>,
}

impl Node{
    pub fn new_empty(me:(i64,i64))->Box<Self>{
        Box::new(Self{
            me,less:None,more:None
        })
    }


}

type PNode = Option<Box<Node>>;

pub fn insert_to_range(op:PNode,start:i64,end:i64)->PNode{
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


    if start<=cur.me.0 {
        // println!("growing {:?} to [{start} {}]",cur.me,cur.me.1);
        cur.me.0=start;
        merge_small(&mut cur.me,&mut cur.less);
    }

    if cur.me.1<=end {
        // println!("growing {:?} to [{} {end}]",cur.me,cur.me.0);

        cur.me.1=end;
        merge_big(&mut cur.me,&mut cur.more);
    }

    Some(cur)
}

pub fn find_bigest(child_op:&mut PNode)->&mut PNode{
    let p : *mut _ = child_op;
    let Some(node) = (unsafe{&mut*p}) else {
        return child_op;
    };

    if node.more.is_none() {
        return unsafe{&mut*p};
    }
    
    find_bigest(&mut node.more)

}

pub fn find_smallest(child_op:&mut PNode)->&mut PNode{
    let p : *mut _=  child_op;
    let Some(node) = (unsafe{&mut*p}) else {
        return child_op;
    };

    if node.less.is_none(){
        return unsafe{&mut*p};
    }

    find_smallest(&mut node.less)

}

pub fn merge_small(range:&mut (i64,i64),child_op:&mut PNode){
    let Some(child) = child_op else {
        return;
    };

    merge_small(range,&mut child.more);
    if child.me.1>=range.0 {
        range.0=child.me.0;
        //fine not removing yet
        merge_small(range,&mut child.less);

        //now take
        let left_take = find_bigest(&mut child.less);
        if let Some(take_node) = left_take {
            child.me = take_node.me;
            let t  = take_node.less.take();
            *left_take=t;
            return;
        }

        let right_take = find_smallest(&mut child.more);
        if let Some(take_node) = right_take {
            child.me = take_node.me;
            let t  = take_node.more.take();
            *right_take=t;
            return;
        }

        child_op.take();
    }
}

pub fn merge_big(range:&mut (i64,i64),child_op:&mut PNode){
    let Some(child) = child_op else {
        return;
    };

    merge_big(range,&mut child.less);
    if child.me.0<=range.1 {
        range.1=child.me.1;
        //fine not removing yet
        merge_big(range,&mut child.more);

        //now take
        let left_take = find_bigest(&mut child.less);
        if let Some(take_node) = left_take {
            child.me = take_node.me;
            let t  = take_node.less.take();
            *left_take=t;
            return;
        }

        let right_take = find_smallest(&mut child.more);
        if let Some(take_node) = right_take {
            child.me = take_node.me;
            let t  = take_node.more.take();
            *right_take=t;
            return;
        }

        child_op.take();
    }
}

pub fn tree_contains(op:&PNode,x:i64)->bool{
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

pub fn tree_len(op:&PNode)->i64{
    let Some(node) = op else{
        return 0;
    };


    let ans =
    (node.me.1-node.me.0+1)+
    tree_len(&node.less)+
    tree_len(&node.more);

    println!("in {:?} returning {ans}",node.me.0..=node.me.1);
    ans

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

    println!("tree is\n{tree:#?}");
    println!("part1 is {ans}");
    println!("part2 is {}",tree_len(&tree))

}