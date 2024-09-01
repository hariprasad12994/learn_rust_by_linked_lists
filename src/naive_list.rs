use std::mem;

struct Node {
    elem: i32,
    next: Link
}

enum Link {
    Empty,
    More(Box<Node>)
}

pub struct List {
    head: Link
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, value: i32) {
        let new_node = Box::new(Node {
            elem: value,
            // head is owned by list, moving leads to head being undefined state
            // mem::replace is under the hood an unsafe fn
            // analogous to std::exchange for exchanging values
            // without dropping them
            // but why head is being moved, because Box is not
            // copyable and is like an unique pointer
            // which is always moved on assignment
            // new + [h: empty] => new.next = empty, h = empty
            // new + [h: some_node(val, empty)] => new.next = some_node(val,empty), h = empty
            // new + [h: some_node(val, some_other_node), some_other_node(val, next)] => new.next = some_node, h = empty
            // h = new_node => [h: new_node]
            //              => [h: new_node(value, some_node), some_node(val, empty)]
            //              => [h: new_node(value, some_node), some_node(val, some_other_node)...]
            next: mem::replace(&mut self.head, Link::Empty)
        });
        // update the head to point to the new_node
        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        // let v;

        // error[E0507]: cannot move out of `self.head` as enum variant `More` which is behind a mutable reference
        //   --> src/naive_list.rs:48:15
        //    |
        // 48 |         match self.head {
        //    |               ^^^^^^^^^
        // 49 |             Link::Empty => { v = None },
        // 50 |             Link::More(node) => {
        //    |                        ----
        //    |                        |
        //    |                        data moved here
        //    |                        move occurs because `node` has type `Box<Node>`, which does not implement the `Copy` trait
        //    |
        // help: consider borrowing here
        //    |
        // 48 |         match &self.head {
        //    |               +

        // match &self.head {
        //     Link::Empty => { v = None },
        //     Link::More(node) => {
        //         v = Some(node.elem);
        //         // a borrow is required since head could be a 
        //         // link pointing to a Link is not copyable
        //         //
        //         // error[E0507]: cannot move out of `node.next` which is behind a shared reference
        //         //   --> src/naive_list.rs:70:29
        //         //    |
        //         // 70 |                 self.head = node.next;
        //         //    |                             ^^^^^^^^^ move occurs because `node.next` has type `Link`, which does not implement the `Copy` trait
        //         //
        //         self.head = node.next;
        //     }
        // };
        // v
        // head = empty
        // temporary variable to which replace is done is dropped at the end of scope
        // if list is not empty move the head, return the temporarily exchanged head.node.elem
        // wrapped in option else if list was empty return none for safe return value
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => { None },
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }        
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        // Due to recursive nature of the data structure the list's 
        // implicit drop impl inserted by the compiler uses recursion 
        // to drop the nodes. For a relatively large list, this 
        // might cause a stackoverflow 
        // Illustation: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=380d5360a9150eb78bc5ff82a49ba74d
        // Hence an iterative version, which take one node at a time
        // replaces the head into a temporary local variable and
        // moves head to the next node, thereby dropping the local
        // variable at the end of scope, the let statement abstracts 
        // away the match required for ending the loop for checking
        // the end of list
        let mut walker = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = walker {
            walker = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn basics() {
        use super::List;

        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}

