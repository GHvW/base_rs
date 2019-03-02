
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

pub struct LLStack<T> {
    head: Option<Box<Node<T>>>,
    size: usize
}

impl<T> LLStack<T> {
    pub fn new() -> Self {
        LLStack {
            head: None,
            size: 0
        }
    }
}

impl<T> Stack<T> for LLStack<T> {
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

