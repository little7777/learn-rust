fn main1() {
    let s = String::from("hello");

    let len = cal_length(&s);
    println!("{} lengtth is {}:", s, len);
}

fn main() {
    let mut s1 = String::from("hello");
    change(&mut s1);
}

fn change( s: &mut  String)  {
    s.push_str(", world");
}


fn cal_length(s: &String) ->  usize {
    s.len()
}
