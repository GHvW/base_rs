struct Node<T> {
    data: T,
    next: Box<Node<T>>
}


struct LinkedList<T> {
    head: Option<Node<T>>,
    len: u32
}
