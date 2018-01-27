
fn first_word(s:&String) ->&str{
    for(i,&ch) in s.as_bytes().iter().enumerate(){
        if ch == b' '{
            return &s[..i]
        }
    }
    &s[..]
}

pub fn slice_demo(){

    let s = String::from("hello world");
    let w = first_word(&s);
    println!("word is: {}",w);

    let s = String::from("中国China");

    for (i,ch) in s.chars().enumerate(){
        println!("{}: {}",i,ch);
    }

    let w = &s[..3];
    println!("{}",w);
}

pub fn slice_string(){
    let x = "中国China";
    let s = String::from(x);
    let w = &s[..];
    println!("{} {} {}",x,s,w);
}