// 100% experimental

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>
}

// trying out a persistent stack implementation
pub struct PersistentStack<T> {
    head: Option<Box<Node<T>>>,
    size: usize
}

impl<T> PersistentStack<T> {
    fn new(data: T) -> Self {
        PersistentStack {
            head: None,
            size: 0
        }
    }

    fn push(&self, data: T) -> Self {
        let new_node = Box::new(Node {
            data: data,
            next: self
        });

        PersistentStack {
            head: Some(new_node),
            size: self.size + 1
        }
    }

    fn pop(&self) -> (T, &Self) {
        let item = match self.head {
          None => None,
          Some(node_ptr) => {
            node.data
          }
        };

        
    }

    fn peek(&self) -> (T, &self) {

    }

    fn size(&self) -> usize {
        self.size
    }
}

