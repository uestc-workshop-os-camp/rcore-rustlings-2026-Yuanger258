/*
	stack
	This question requires you to use a stack to achieve a bracket match//就是实现括号匹配
*/

#[derive(Debug)]
struct Stack<T> {         //定义了Stack
	size: usize,
	data: Vec<T>,
}
impl<T> Stack<T> {
	fn new() -> Self {
		Self {
			size: 0,        //创建一个Stack
			data: Vec::new(),
		}
	}
	fn is_empty(&self) -> bool {   //判断栈是否为空
		0 == self.size
	}
	fn len(&self) -> usize {   //长度
		self.size
	}
	fn clear(&mut self) {   //
		self.size = 0;
		self.data.clear();//移除向量中的所有元素
	}
	fn push(&mut self, val: T) {  //进栈，把元素添加到向量尾部
		self.data.push(val);
		self.size += 1;
	}
	fn pop(&mut self) -> Option<T> {//这里需要我实现pop，出栈的话size减小1，然后这里应该可以用pop
		//移除最后一个元素，先用is_empty检查一下
		// TODO
		let resultt = self.is_empty();
		match resultt{
			false => {
				//如果这里的这个栈不为空
				self.size -= 1;
				self.data.pop()
				
			},
			true => None,
		}
	}
	fn peek(&self) -> Option<&T> {
		if 0 == self.size {
			return None;
		}
		self.data.get(self.size - 1)
	}
	fn peek_mut(&mut self) -> Option<&mut T> {
		if 0 == self.size {
			return None;
		}
		self.data.get_mut(self.size - 1)
	}
	fn into_iter(self) -> IntoIter<T> {
		IntoIter(self)
	}
	fn iter(&self) -> Iter<T> {
		let mut iterator = Iter { 
			stack: Vec::new() 
		};
		for item in self.data.iter() {
			iterator.stack.push(item);
		}
		iterator
	}
	fn iter_mut(&mut self) -> IterMut<T> {
		let mut iterator = IterMut { 
			stack: Vec::new() 
		};
		for item in self.data.iter_mut() {
			iterator.stack.push(item);
		}
		iterator
	}
}
struct IntoIter<T>(Stack<T>);
impl<T: Clone> Iterator for IntoIter<T> {
	type Item = T;
	fn next(&mut self) -> Option<Self::Item> {
		if !self.0.is_empty() {
			self.0.size -= 1;self.0.data.pop()
		} 
		else {
			None
		}
	}
}
struct Iter<'a, T: 'a> {
	stack: Vec<&'a T>,
}
impl<'a, T> Iterator for Iter<'a, T> {
	type Item = &'a T;
	fn next(&mut self) -> Option<Self::Item> {
		self.stack.pop()
	}
}
struct IterMut<'a, T: 'a> {
	stack: Vec<&'a mut T>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
	type Item = &'a mut T;
	fn next(&mut self) -> Option<Self::Item> {
		self.stack.pop()
	}
}

fn bracket_match(bracket: &str) -> bool
{
	//思路，匹配括号，应该需要考虑的是括号能不能匹配，有三种括号，然后就是用迭代器遍历字符串，只有六种字符可以被压栈，
	//如果没有可以压入栈的，true，
	//然后每次对于要入栈的元素，如果是左括号，直接入栈，如果是右括号，match语句，例如小括号的右括号就peek一下，如果是的话，就pop
	//如果不能对应，就返回false
	//这样子循环之后，如果不匹配的话，就已经在对应的那次循环中false了，所以在循环执行后调用一次is_empty,如果结果是true，那么说明匹配成功
	//然后就可以返回true作为结果了，如果是false，就返回false
	//这里默认了输入的字符串至少第一个进来的括号就为左括号，但是每次对于进栈的元素，已经用match区分了左括号还是有括号的不同，所以不需要担心这样的情况

	//TODO
	//首先把这个str放进vec之中
	let data: Vec<char> = bracket.chars().collect();
	//这里应该先创建一个栈
	let mut stack:Stack <char> = Stack::new();
	//接下来就是遍历字符了
	for ch in data{
		match ch {
			'(' | '{' | '[' =>{stack.push(ch)
			},
			')'|']'|'}'=>{
				match stack.peek(){
					Some(ch1)=>{
						match ch1{
							'(' =>{
								if ch != ')'{
									return false;
								}else {
									stack.pop();
								}
								
								
							},
							'{' =>{
								if ch != '}'{
									return false;
								}else{
									stack.pop();
								}
							},
							'[' =>{
								if ch != ']'{
									return false;
								}else{
									stack.pop();
								}
							}
							_ => return false,
						}
					},
					None => return false,
				}
			},
			_ =>(),
		}

		
		}
		match stack.is_empty(){
			true => return true,
			false =>return false,
	}

}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn bracket_matching_1(){
		let s = "(2+3){func}[abc]";
		assert_eq!(bracket_match(s),true);
	}
	#[test]
	fn bracket_matching_2(){
		let s = "(2+3)*(3-1";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_3(){
		let s = "{{([])}}";
		assert_eq!(bracket_match(s),true);
	}
	#[test]
	fn bracket_matching_4(){
		let s = "{{(}[)]}";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_5(){
		let s = "[[[]]]]]]]]]";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_6(){
		let s = "";
		assert_eq!(bracket_match(s),true);
	}
}