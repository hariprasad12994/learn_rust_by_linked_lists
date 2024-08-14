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
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => { None },
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }        
        }
    }
}
