
# 1 vector

* 创建

```
let _v = vec![1,2,3];
let mut v : Vec<i32> = Vec::new();
```

* 索引访问: 下标越界时panic
* get访问：返回Option<T>类型，下标越界时返回None
* 不可变遍历: ```for x in &v```
* 可变遍历: ```for x in &mut v```
* 可通过枚举绑定多种类型，然后在vector中存储枚举，间接达到存储多种不同类型的目标

# 2 字符串

* 字面字符串的类型是&str,这是由语言核心定义的
* String由标准库定义，内部由一个```Vec<u8>```构成
* <font color="red">不能对String进行索引操作，只能进行切片操作，且切片时要注意下标在正确的字符边界上</font>
* 字面字符串到String: 需要另外分配内存，开销大
  * ```"initial contents".to_string();```
  * ```String::from("initial contents");```
  * ```let s = "initial contents"; s.to_string();```
* String转到&str：```&s[..]```，不需要另外分配内存，开销小
* 更新String
  * ```s.push('l')```
  * ```s.push_str("abc")```
* 遍历String
  * ```for c in s.chars()```
  * ```for (i,c) in s.bytes().enumerate()```

# 3 HashMap

* prelude不包含HashMap，使用之前需要从外部导入: ```use std::collections::HashMap```
* 新建HashMap: ```HashMap::new()```
* 加入元素：```map.insert(k,v)```
* 多次加入相同key的元素，则后面的加入操作覆盖前面的加入操作
* 获取元素：```map.get(key)```返回```Option<T>```类型，试图获取不存在的元素时返回```None```
* 获取元素: 用```map[key]```返回不存在的元素时会发生panic
* 只在元素不存在时增加元素：```map.entry(key).or_insert(val)```
* 可使用zip方法创建元组vector，然后使用collect方法转化成HashMap  