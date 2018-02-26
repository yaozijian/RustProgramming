
use std::collections::HashMap;

pub fn demo1(){

	let mut array = vec![9,4,5,6];
	array.push(2);
	array.push(7);
	array.push(3);
	array.push(7);

	array.sort();

	let mididx = array.len() / 2;
	let midval = array[mididx];

	let mut sum = 0;
	let mut maxcnt = 0;
	let mut maxval = 0;

	let mut val2cnt = HashMap::new();

	for v in &array{

		let val = *v;
		let cnt = val2cnt.entry(val).or_insert(0);
		*cnt += 1;

		if *cnt > maxcnt{
			maxcnt = *cnt;
			maxval = val;
		}

		sum += v;
	}

	println!("平均数={} 中位数={}",(sum as f64) / (array.len() as f64),midval);
	println!("众数: {} 个数: {}",maxval,maxcnt);
}

pub fn demo2(){

	let mut strs = Vec::new();
	strs.push(String::from("world"));
	strs.push(String::from("apple"));
	strs.push(String::from("中国China"));
	strs.push(String::from("中国offer"));
	strs.push(String::from("云想衣裳花想容"));

	for (i,x) in strs.iter().enumerate(){

		let mut t = x.clone();
		let mut item = &mut t;
		let mut found = false;

		for (chidx,chval) in x.char_indices(){
			match chval{
				// 为保证逻辑正确,这个子句必须是第一个,因为英文元音字母也满足第二个子句
				'a'|'e'|'i'|'o'|'u'|'A'|'E'|'I'|'O'|'U' => {
					if chidx == 0 {
						item.push_str("-hay");
						found = true;
					}
				},
				_ if ('a'<= chval && chval <='z') || ('A' <= chval && chval <='Z') =>{
					item.push('-');
					item.push(chval);
					item.push_str("ay");
					let mut head = String::from(&item[..chidx]);
					*item = head + &item[chidx+1..];
					found = true;
				}
				// 必须有这个子句以满足穷尽匹配的要求
				_ => ()
			}

			if found {
				break;
			}
		}

		if found {
			println!("idx: {} old: {} new: {}", i, x, *item);
		}else{
			println!("idx: {} old: {} 没有英文字母",i,x);
		}
	}
}

