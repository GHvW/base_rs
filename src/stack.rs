
pub struct Node<T> {
    pub data: T,
    pub next: Option<Box<Node<T>>>
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node {
            data: data,
            next: None
        }
    }
}

trait Stack<T> {
    fn push(&mut self, data: T) -> ();
    fn pop(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
    fn size(&self) -> usize;
}

pub struct LinkedStack<T> {
    head: Option<Box<Node<T>>>,
    size: usize
}

impl<T> LinkedStack<T> {
    pub fn new() -> Self {
        LinkedStack {
            head: None,
            size: 0
        }
    }
}

impl<T> Stack<T> for LinkedStack<T> {
    fn push(&mut self, data: T) -> () {
        let mut new_node = Box::new(Node::new(data));
        new_node.next = self.head.take();
        self.head = Some(new_node);
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(node_ptr) => {
                // can't use node.next and Some(node.data) because Option<Box<>> does not implement copy trait
                let node = *node_ptr;
                self.head = node.next;
                self.size -= 1;
                Some(node.data)
            }
        }
    }

    fn peek(&self) -> Option<&T> {
        // self.head.as_ref().map(|node| &node.data)
        match self.head {
            None => None,
            Some(ref node) => {
                Some(&node.data)
            }
        }
    }

    fn size(&self) -> usize {
        self.size
    }
}

// struct VecStack<T> {
//     vec: Vec<T>
// }

// impl<T> Stack<T> for VecStack<T> {

//     fn push(&self, data: T) -> () {
//         self.vec.push(data);
//         ()
//     }

//     fn pop(&mut self) -> Option<T> {
//         self.vec.pop()
//     }

//     fn peek(&self) -> Option<&T> {
//         self.vec[self.vec.len() - 1]
//     }

//     fn size(&self) -> u32 {
//         self.vec.len()
//     }
// }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pop_test() {
        let mut stack = LinkedStack::new();

        assert_eq!(None, stack.pop());

        stack.push(2);
        stack.push(3);

        assert_eq!(Some(3), stack.pop());
        assert_eq!(Some(2), stack.pop());
    }

    #[test]
    fn peek_test() {
        let mut stack = LinkedStack::new();

        assert_eq!(None, stack.peek());

        stack.push(1);
        stack.push(2);

        assert_eq!(Some(&2), stack.peek());
        assert_eq!(Some(&2), stack.peek());

        stack.pop();
        assert_eq!(Some(&1), stack.peek());
    }

    #[test]
    fn size_test() {
        let mut stack = LinkedStack::new();

        assert_eq!(0, stack.size());

        stack.pop();
        assert_eq!(0, stack.size());

        stack.push(1);
        stack.push(2);

        assert_eq!(2, stack.size());

        stack.pop();
        assert_eq!(1, stack.size());

        stack.peek();
        assert_eq!(1, stack.size());
    }
}