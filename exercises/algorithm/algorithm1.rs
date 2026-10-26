/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/
// I AM NOT DONE

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {//这里先定义了Node结点，包含了val值和next一个指针，这个val是一个泛型，而next指针是一个option类型的，可以是Some或者
    val: T,//None，然后
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {                   //这里就是为了Node实现一个方法，返回的是Node类型的一个，这一块新加进来的结点的值在括号里，
    fn new(t: T) -> Node<T> {       //然后指针域是None
        Node {
            val: t,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T> {                    //这个链表，长度为length，剩下两个成员分别是头指针和尾指针
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {             
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {               
    pub fn new() -> Self {    //这里是new一个空的链表
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {  //这里是尾插法                 //这里应该是新建一个结点的意思，这里传进来的是一个链表和一个参数（可能是插）
        let mut node = Box::new(Node::new(obj));      //进来的结点的值域里的东西
        node.next = None;                             //然后node是一个Box的智能指针，指向了一个值为obj，指针与为空的一个Node类型的东西
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });//然后；
        match self.end {                        //如果链表的尾指针是None,就把这个新加进来的作为头指针
            None => self.start = node_ptr,      //如果不是的话，这里先得到了原来的尾结点的指针，然后呢
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {   //这里传入了self和一个index，然后调用了一个get_ith_node
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {//传进来的是链表，一个节点
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
	pub fn merge(list_a:LinkedList<T>,list_b:LinkedList<T>) -> Self
	{       //所以最后就是在这里写出来merge的逻辑什么的
        /*
        pub fn merge(list_a: LinkedList<T>, list_b: LinkedList<T>) -> Self
where
    T: Ord,
{
    // 两个指针分别指向两个链表当前待处理的节点
    let mut p_a = list_a.start;
    let mut p_b = list_b.start;

    // 创建一个空的结果链表
    let mut merged = Self::new();

    // 只要还有一个链表存在未处理的节点，就继续
    while p_a.is_some() || p_b.is_some() {
        // 选择较小的节点，并提前移动对应的指针
        let mut chosen = match (p_a, p_b) {
            (Some(a), Some(b)) => unsafe {
                if a.as_ref().val <= b.as_ref().val {
                    p_a = a.as_ref().next;
                    a
                } else {
                    p_b = b.as_ref().next;
                    b
                }
            },

            // A 为空，直接处理 B 剩余的节点
            (None, Some(b)) => {
                p_b = unsafe { b.as_ref().next };
                b
            }

            // B 为空，直接处理 A 剩余的节点
            (Some(a), None) => {
                p_a = unsafe { a.as_ref().next };
                a
            }

            (None, None) => unreachable!(),
        };

        // 先断开所选节点原来的 next 连接
        unsafe {
            chosen.as_mut().next = None;
        }

        // 将所选节点接到结果链表的末尾
        match merged.end {
            None => {
                // 结果链表为空，所选节点就是头节点
                merged.start = Some(chosen);
            }
            Some(mut tail) => unsafe {
                // 原尾节点指向新接入的节点
                tail.as_mut().next = Some(chosen);
            },
        }

        // 更新尾指针和长度
        merged.end = Some(chosen);
        merged.length += 1;
    }

    merged
}
         */
            //思路，合并升序的两个单链表 self一共有三个参数，length就是两个链表的length之和
		//TODO
        let mut merged = Self::new();//新建一个链表

        let mut ptra = list_a.start;
        let mut ptrb = list_b.start;//这里先两个指针指向要处理的两个链表

        while(ptra.is_some||ptrb.is_some){
            let mut chosen = match (ptra,ptrb){
                (Some(a),Some(b))=> unsafe{
                    
                }
            };

        }
		Self {
            length: 0,
            start: None,
            end: None,
        }
	}
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];
		
		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
}