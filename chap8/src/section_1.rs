
pub fn vec_demo(){
	
	let _v = vec![1,2,3];
	let mut v : Vec<i32> = Vec::new();
	
	v.push(5);
	v.push(6);
	v.push(7);
	
	{
	let a = &mut v[0];
	*a *= 10;
	println!("a={}",a);
	}
	
	{
	let a = &v[0];
	let mut b = v[1];
	let c : Option<&i32> = v.get(2);
	let d = v.get(3);
	b *= 10;
	println!("a={} b={} v[1]={} c={:?} d={:?}",a,b,v[1],c,d);
	}
	
	for x in &mut v{
		*x *= 10;
		println!("{}",x);
	}
	
	for x in &v{
		println!("{}",x);
	}
}

