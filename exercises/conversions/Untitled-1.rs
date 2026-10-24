// from_str.rs
//
// 这道题与 from_into.rs 类似，
// 但这次我们要实现 FromStr trait，
// 并在转换失败时返回错误，而不是返回默认值。
//
// 此外，实现 FromStr 后，
// 就可以使用字符串的 parse() 方法，
// 将字符串解析为实现了该 trait 的类型的对象。
//
// 官方文档：
// https://doc.rust-lang.org/std/str/trait.FromStr.html
//
// 运行 `rustlings hint from_str` 获取提示，
// 或使用 `hint` 子命令查看提示。

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

// 我们将使用下面定义的错误类型，
// 作为 FromStr 实现中的错误类型。
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // 输入字符串为空
    Empty,

    // 字段数量不正确
    BadLen,

    // 姓名字段为空
    NoName,

    // 保存 parse::<usize>() 产生的整数解析错误
    ParseInt(ParseIntError),
}

// I AM NOT DONE

// 实现步骤：
//
// 1. 如果输入字符串的长度为 0，返回一个错误。
//
// 2. 按照字符串中出现的逗号拆分输入字符串。
//
// 3. 拆分后必须恰好得到两个元素，
//    否则返回一个错误。
//
// 4. 提取拆分后的第一个元素，
//    并将其作为姓名。
//
// 5. 提取另一个元素，
//    并将其解析为 usize 类型的年龄。
//    可以使用类似 "4".parse::<usize>() 的写法。
//
// 6. 如果提取姓名或年龄的过程中出现问题，
//    就应该返回一个错误。
//
// 如果所有操作都成功，
// 就返回一个包含 Person 对象的 Result。
//
// 补充说明：
//
// Box<dyn Error> 实现了 From<&str>。
// 这意味着，如果你想返回一条字符串作为错误信息，
// 可以直接使用下面这样的写法：
//
// return Err("my error message".into());

impl FromStr for Person {
    type Err = ParsePersonError;

    fn from_str(s: &str) -> Result<Person, Self::Err> {
        // TODO：在这里实现字符串解析逻辑。
        //如果长度为0
        if s.is_empty(){
            return Err(ParsePersonError::Empty);
        }

        //然后拆分输入字符串
        let mut name_and_age = s.split(',');

        //然后先完成4 5 6 最后检查元素的个数
        let name_ = match name_and_age.next(){
            Some(value) => value,
            None => return Err(ParsePersonError::BadLen),            
        };

        let agestr = match name_and_age.next(){
            Some(value) => value,
            None => return Err(ParsePersonError::BadLen),
        };

        if name_and_age.next().is_some() {
            return Err(ParsePersonError::BadLen);
        }

        if name_.is_empty(){
            return Err(ParsePersonError::NoName);
        }

        let age = match agestr.parse::<usize>(){
            Ok(value) =>value,
            Err(error) => {return Err(ParsePersonError::ParseInt(error))}
        };

        Ok(Person{
            name: name_.to_string(),
            age:age.unwrap()})



    }
}

fn main() {
    // 将字符串 "Mark,20" 解析为 Person 对象。
    // 如果解析失败，unwrap() 会触发 panic。
    let p = "Mark,20".parse::<Person>().unwrap();

    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        // 测试输入字符串为空的情况。
        // 预期返回 Empty 错误。
        assert_eq!("".parse::<Person>(), Err(ParsePersonError::Empty));
    }

    #[test]
    fn good_input() {
        // 测试合法输入。
        // 预期将 "John,32" 解析为姓名为 John、年龄为 32 的 Person。
        let p = "John,32".parse::<Person>();

        // 检查解析是否成功。
        assert!(p.is_ok());

        // 取出解析成功的 Person 对象。
        let p = p.unwrap();

        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }

    #[test]
    fn missing_age() {
        // 测试缺少年龄的情况。
        // 预期返回整数解析错误。
        assert!(matches!(
            "John,".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn invalid_age() {
        // 测试年龄不是合法整数的情况。
        // 预期返回整数解析错误。
        assert!(matches!(
            "John,twenty".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_comma_and_age() {
        // 测试缺少逗号和年龄的情况。
        // 预期返回字段数量错误。
        assert_eq!("John".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn missing_name() {
        // 测试缺少姓名的情况。
        // 预期返回 NoName 错误。
        assert_eq!(",1".parse::<Person>(), Err(ParsePersonError::NoName));
    }

    #[test]
    fn missing_name_and_age() {
        // 测试姓名和年龄都缺失的情况。
        // 预期返回 NoName 或整数解析错误。
        assert!(matches!(
            ",".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        // 测试姓名缺失且年龄无法解析的情况。
        // 预期返回 NoName 或整数解析错误。
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn trailing_comma() {
        // 测试字符串末尾多出一个逗号的情况。
        // 预期返回字段数量错误。
        assert_eq!("John,32,".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        // 测试字符串末尾多出逗号和额外内容的情况。
        // 预期返回字段数量错误。
        assert_eq!(
            "John,32,man".parse::<Person>(),
            Err(ParsePersonError::BadLen)
        );
    }
}