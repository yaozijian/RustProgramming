extern crate std;

pub fn demo1() {
	trait Summarizable {
		fn summary(&self) -> String;
	}

	struct Tweet {
		username: String,
		content: String,
		pub reply: bool,
		pub retweet: bool,
	}

	impl Summarizable for Tweet {
		fn summary(&self) -> String {
			format!("{}: {}", self.username, self.content)
		}
	}

	// 注意写法
	let obj1: &Summarizable = &Tweet {
		username: String::from("SunWuKong"),
		content: "DaNaoTianGong".to_string(),
		reply: false,
		retweet: false,
	};
	println!("{}", obj1.summary());
}

pub fn demo2() {
	trait Summarizable {
		fn author_summary(&self) -> String;
		// 默认实现可以调用其他没有默认实现的方法
		fn summary(&self) -> String {
			format!("(Read more from {}...)", self.author_summary())
		}
	}

	#[derive(Debug)]
	struct NewsArticle {
		headline: String,
		location: String,
		author: String,
		content: String,
	}

	// 只实现没有默认实现的方法就可以了
	impl Summarizable for NewsArticle {
		fn author_summary(&self) -> String {
			format!("{}, by {} ({})", self.headline, self.author, self.location)
		}
	}

	// 特性限定
	fn notify1<T:Summarizable>(item: &T){
		println!("独家新闻: {}",item.summary());
	}

	// 使用where子句来定义特性限定
	fn notify2<T>(item: &T) where T: Summarizable{
		println!("独家新闻: {}",item.summary());
	}

	// 使用where子句和多个特性限定
	fn notify3<T>(item: &T) where T: Summarizable + std::fmt::Debug{
		println!("独家新闻: {:?}",item.summary());
	}

	let obj1 = &NewsArticle{
		headline: "Chang'eBenYue".to_string(),
		location: String::from("JiuQuan"),
		author: String::from("JiZhe"),
		content: String::from("JinRiShenKong"),
	};

	notify1(obj1);
	notify2(obj1);
	notify3(obj1);
}
