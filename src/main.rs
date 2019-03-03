mod stack;
use stack::{ Stack, LinkedStack, VecStack };

fn main() {
    println!("Hello, world!");

    let mut lstack: LinkedStack<i32> = LinkedStack::new();
    let mut vstack: VecStack<i32> = VecStack::new();

    lstack.push(1);
    // vstack.push(2);
    println!("{:?}", vstack.peek());
    vstack.push(3);
    println!("{:?}", vstack.peek());
}
