/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/


#[derive(Debug)]
pub struct Queue<T> {  //定义了Queue
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {  //入队
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {   //出队
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {     //查看队头的元素
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {         //计算队列长度
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {   //判断队是否为空
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct myStack<T>             
{
	//TODO
	q1:Queue<T>,
	q2:Queue<T>
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
			//TODO
			q1:Queue::<T>::new(),
			q2:Queue::<T>::new()
        }
    }
    pub fn push(&mut self, elem: T) {
        //TODO
        //用两个队列实现入栈
        self.q1.enqueue(elem);
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        //TODO
        //如果q1有i个元素，让q1出队然后进入q2，一共i-1次，剩下的就是要被pop的，然后交换q1和q2
        if !self.q1.is_empty(){
            let length = self.q1.size();
            for _ in 1..length{
                self.q2.enqueue(self.q1.dequeue().unwrap());
            }
            let result = self.q1.dequeue().unwrap();

            std::mem::swap(&mut self.q1, &mut self.q2);

            Ok(result)

        }else{
            Err("Stack is empty")}
    }
    pub fn is_empty(&self) -> bool {
		//TODO
        self.q1.is_empty()
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}