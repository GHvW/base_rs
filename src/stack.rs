
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>
}

// impl<T> Node<T> {
//     fn new(data: T) -> Self {
//         Node {
//             data: data,
//             next: None
//         }
//     }
// }

pub trait Stack<T> {
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
        let new_node = Box::new(Node {
            data: data,
            next: self.head.take()
        });

        self.head = Some(new_node);
        self.size += 1;
        ()
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
        self.head.as_ref().map(|node| &node.data)
        // match self.head {
        //     None => None,
        //     Some(ref node) => {
        //         Some(&node.data)
        //     }
        // }
    }

    fn size(&self) -> usize {
        self.size
    }
}

pub struct VecStack<T> {
    vec: Vec<T>
}

impl<T> VecStack<T> {
   pub fn new() -> Self {
        VecStack {
            vec: Vec::new()
        }
    }
}

impl<T> Stack<T> for VecStack<T> {

    fn push(&mut self, data: T) -> () {
        self.vec.push(data);
        ()
    }

    fn pop(&mut self) -> Option<T> {
        self.vec.pop()
    }

    fn peek(&self) -> Option<&T> {
        let index = self.vec.len();
        if index == 0 {
            return None
        }
        self.vec.get(self.vec.len() - 1)
    }

    fn size(&self) -> usize {
        self.vec.len()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linked_pop_test() {
        let mut stack = LinkedStack::new();

        assert_eq!(None, stack.pop());

        stack.push(2);
        stack.push(3);

        assert_eq!(Some(3), stack.pop());
        assert_eq!(Some(2), stack.pop());
    }

    #[test]
    fn linked_peek_test() {
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
    fn linked_size_test() {
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


    #[test]
    fn vec_pop_test() {
        let mut stack = VecStack::new();

        assert_eq!(None, stack.pop());

        stack.push(2);
        stack.push(3);

        assert_eq!(Some(3), stack.pop());
        assert_eq!(Some(2), stack.pop());
    }

    #[test]
    fn vec_peek_test() {
        let mut stack = VecStack::new();

        assert_eq!(None, stack.peek());

        stack.push(1);
        stack.push(2);

        assert_eq!(Some(&2), stack.peek());
        assert_eq!(Some(&2), stack.peek());

        stack.pop();
        assert_eq!(Some(&1), stack.peek());
    }

    #[test]
    fn vec_size_test() {
        let mut stack = VecStack::new();

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