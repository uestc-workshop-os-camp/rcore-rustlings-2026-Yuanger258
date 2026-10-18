// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
   /*  //用迭代器的方式实现阶乘
    //首先创建一个迭代器
    //然后
    let nums:Vec<u64> = (1..=num).collect();
    //我先创建一个Vec数组，然后再用迭代器的东西
    let nums_iter: u64 = nums.iter().product();
    return nums_iter;*/

    //看了一下发现范围也能生成迭代器，然后就不需要使用return了
    (1..=num).product()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
