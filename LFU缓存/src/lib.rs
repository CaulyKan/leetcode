/*
 * @lc app=leetcode.cn id=460 lang=rust
 *
 * [460] LFU缓存
 */

// @lc code=start
use std::cell::RefCell;
use std::collections::*;
use std::rc::Rc;

struct LFUCacheNode {
    val: i32,
    room: Rc<RefCell<LFUCacheRoom>>,
    previous: Option<Rc<RefCell<LFUCacheNode>>>,
    next: Option<Rc<RefCell<LFUCacheNode>>>,
}

struct LFUCacheRoom {
    freq: i32,
    nodes: Option<Rc<RefCell<LFUCacheNode>>>,
    next: Option<Rc<RefCell<LFUCacheRoom>>>,
}

struct LFUCache {
    capacity: usize,
    len: usize,
    first: Option<Rc<RefCell<LFUCacheNode>>>,
    last: Option<Rc<RefCell<LFUCacheNode>>>,
    rooms: Rc<RefCell<LFUCacheRoom>>,
    map: HashMap<i32, Rc<RefCell<LFUCacheNode>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LFUCache {
    fn new(capacity: i32) -> Self {
        LFUCache {
            capacity: capacity as usize,
            map: HashMap::new(),
            rooms: Rc::new(RefCell::new(LFUCacheRoom {
                freq: 0,
                nodes: None,
                next: None,
            })),
            first: None,
            last: None,
            len: 0,
        }
    }

    fn get(&self, key: i32) -> i32 {
        if self.capacity == 0 {
            -1
        } else {
            if let Some(n) = self.map.get(&key) {
                self.remove_node(n);
                let new_room = {
                    let mut room = n.borrow().room.borrow_mut();
                    if room.next.is_none() {
                        room.next = Some(Rc::new(RefCell::new(LFUCacheRoom {
                            freq: room.freq,
                            nodes: None,
                            next: None,
                        })));
                    }
                    let result = room.next.as_ref().unwrap();
                    Rc::clone(result)
                };
                let mut mut_new_room = new_room.borrow_mut();
                if let Some(next) = &mut_new_room.nodes {
                    let mut mut_next = next.borrow_mut();
                    mut_next.previous = Some(Rc::clone(n));
                    node.next = Some(Rc::clone(next));
                }
                mut_new_room.nodes = Some(Rc::clone(n));

                node.val
            } else {
                -1
            }
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.capacity > 0 {
            if self.map.contains_key(&key) {
                let mut node = self.map[&key].borrow_mut();
                node.val = value;
            } else {
                let mut room = self.rooms.borrow_mut();
                let node = Rc::new(RefCell::new(LFUCacheNode {
                    val: value,
                    room: Rc::clone(&self.rooms),
                    next: None,
                    previous: None,
                }));
                if let Some(c) = &room.nodes {
                    let mut cn = node.borrow_mut();
                    cn.next = Some(Rc::clone(c));
                    let mut cc = c.borrow_mut();
                    cc.previous = Some(Rc::clone(&node));
                }
                room.nodes = Some(Rc::clone(&node));
                self.map.insert(key, node);
            }
        }
    }

    fn insert_node(&mut self, node: &Rc<RefCell<LFUCacheNode>>, next: &Rc<RefCell<LFUCacheNode>>) {
        let mut_next = next.borrow();
        if let Some(prev) = &mut_next.previous {
            let mut mut_prev = prev.borrow_mut();
            let mut mut_next = next.borrow_mut();
            let mut mut_node = node.borrow_mut();
            mut_next.previous = Some(Rc::clone(node));
            mut_prev.next = Some(Rc::clone(node));
            mut_node.previous = Some(Rc::clone(prev));
            mut_node.next = Some(Rc::clone(next));
        } else {
            let mut mut_next = next.borrow_mut();
            let mut mut_node = node.borrow_mut();
            mut_next.previous = Some(Rc::clone(node));
            mut_node.next = Some(Rc::clone(next));
        }
        let room = &next.borrow().room;
        let mut mut_room = room.borrow_mut();
        if Rc::ptr_eq(&(mut_room.nodes.as_ref().unwrap()), next) {
            mut_room.nodes = Some(Rc::clone(node));
        }
    }

    fn remove_node(&mut self, node: &Rc<RefCell<LFUCacheNode>>) {
        let mut mut_node = node.borrow_mut();

        if let Some(prev) = &mut_node.previous {
            if let Some(next) = &mut_node.next {
                let mut mut_next = next.borrow_mut();
                let mut mut_prev = prev.borrow_mut();
                mut_next.previous = Some(Rc::clone(prev));
                mut_prev.next = Some(Rc::clone(next));
                let mut mut_room = mut_node.room.borrow_mut();
                if Rc::ptr_eq(&(mut_room.nodes.as_ref().unwrap()), node) {
                    mut_room.nodes = Some(Rc::clone(next));
                }
            } else {
                let mut mut_room = mut_node.room.borrow_mut();
                if Rc::ptr_eq(&(mut_room.nodes.as_ref().unwrap()), node) {
                    mut_room.nodes = None;
                }
            }
        } else {
            if let Some(next) = &mut_node.next {
                let mut mut_next = next.borrow_mut();
                mut_next.previous = None;
                let mut mut_room = mut_node.room.borrow_mut();
                mut_room.nodes = Some(Rc::clone(next));
            } else {
                let mut mut_room = mut_node.room.borrow_mut();
                mut_room.nodes = None;
            }
        }
    }
}

// @lc code=end
