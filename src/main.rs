struct Node<T>{
    data : T,
    next : Option<Box<Node<T>>>
}
struct LinkedList<T>{
    head : Option<Box<Node<T>>>
}

impl<T>LinkedList<T> {
    fn new()->Self{
        LinkedList { head: None }
    }
    fn push(&mut self, data:T){
        let node = Box::new(Node{
            data,
            next : self.head.take()
        });
        self.head = Some(node)

    }
    fn pop(&mut self)->Option<T>{
        let val = self.head.take();
        match val {
            Some(mut node)=>{
                self.head = node.next.take();
                Some(node.data)
            },
            None => None
        }
    }
    fn length(&self)->u8{
        let mut len = 0;
        let mut n = &self.head;
        while let Some(ref node)=*n{
            n = &node.next;
            len +=1;
        }
        len
    }
}


fn main() {
    let mut ll = LinkedList::new();
    ll.push(1);
    ll.push(2);
    ll.push(3);
    ll.push(4);
    ll.pop();
    let len = ll.length();
    println!("{:?}", len);
}
