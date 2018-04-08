use super::*;

#[test]
fn one_result(){
    let query = "duct";
    let content = "Rust:
Safe,Fast,Productive.
Pick three.";
    assert_eq!(vec!["Safe,Fast,Productive."],search(query,content));
}

#[test]
fn case_insensitive(){
    let query = "rUsT";
    let content = "Rust:
Safe,Fast,Productive.
Pick three.
Trust me.";
    assert_eq!(vec!["Rust:","Trust me."],search_insensitive(query,content));
}