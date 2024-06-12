#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List{
           head: None,
        }
    }

    pub fn push(&mut self, value: T) {
        // let node = Node{value, next:self.head.take().map(Box::new)};
        self.head = Some(Node{value, next: self.head.take().map(Box::new)});
    }

    pub fn pop(&mut self) {
        self.head = self.head.take().and_then(|node| node.next.map(|node| *node))
    }

    pub fn len(&self) -> usize {
        match self.head.as_ref() {
            None => 0, // Base case: if the list is empty, return 0
            Some(head) => 1 + self.tail_len(head), // Recursive case: count the current node and recursively count the rest
        }
    }

    fn tail_len(&self, node: &Node<T>) -> usize {
        match node.next.as_ref() {
            None => 0, // Base case: if the next node is None, return 0
            Some(next_node) => 1 + self.tail_len(next_node), // Recursive case: count the next node and recursively count the rest
        }
    }
}


fn main() {
    let mut new_list_str = List::new();
    new_list_str.push("String Test 1");
    println!("The size of the list is {}", new_list_str.len());

    new_list_str.push("String Test 2");
    println!("The size of the list is {}", new_list_str.len());

    new_list_str.push("String Test 3");
    println!("The size of the list is {}", new_list_str.len());

    new_list_str.pop();
    println!("The size of the list is {}", new_list_str.len());
}