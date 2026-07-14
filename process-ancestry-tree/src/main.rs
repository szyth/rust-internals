// 1.8 — Smart pointers
// Exercise: Process Ancestry Tree for a Security Monitor

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

struct ProcessNode {
    pid: u32,
    children: RefCell<Vec<Rc<ProcessNode>>>, // parent owns children
    parent: RefCell<Weak<ProcessNode>>,      // children observes parent
}

fn new_root(pid: u32) -> Rc<ProcessNode> {
    let node = ProcessNode {
        pid,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    };

    Rc::new(node)
}

fn spawn_child(parent: &Rc<ProcessNode>, pid: u32) -> Rc<ProcessNode> {
    let child_node = ProcessNode {
        pid,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Rc::downgrade(parent)),
    };

    let child = Rc::new(child_node);

    parent.children.borrow_mut().push(Rc::clone(&child));

    child
}

impl ProcessNode {
    // recursively check ancestors
    fn ancestry(&self) -> Vec<u32> {
        let mut ancestor_pids: Vec<u32> = vec![];

        if let Some(parent) = self.parent.borrow().upgrade() {
            ancestor_pids.push(parent.pid);
            ancestor_pids.extend(parent.ancestry());
        }

        ancestor_pids
    }
    // recursively check children count
    fn descendant_count(&self) -> usize {
        let node = self.children.borrow();
        let mut count: usize = node.len();

        for child in node.iter() {
            count += child.descendant_count();
        }
        count
    }
}

impl Drop for ProcessNode {
    fn drop(&mut self) {
        println!("Dropping PID: {}", self.pid);
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_ancestry_walks_up_to_root() {
        let root = new_root(0);
        let child1 = spawn_child(&root, 1);
        let _child2 = spawn_child(&root, 2);
        let _child3 = spawn_child(&root, 3);
        let _grand_child11 = spawn_child(&child1, 11);
        let _grand_child12 = spawn_child(&child1, 12);
        let grand_child13 = spawn_child(&child1, 13);

        assert_eq!(
            grand_child13.ancestry(),
            vec![1, 0],
            "Grand child should have 2 ancestors"
        );
    }

    #[test]
    fn test_descendant_count_across_multiple_levels() {
        let root = new_root(0);
        let child1 = spawn_child(&root, 1);
        let _child2 = spawn_child(&root, 2);
        let _child3 = spawn_child(&root, 3);
        let _grand_child11 = spawn_child(&child1, 11);
        let _grand_child12 = spawn_child(&child1, 12);
        let _grand_child13 = spawn_child(&child1, 13);

        println!("descendant_count: {}", root.descendant_count());
        assert_eq!(
            root.descendant_count(),
            6,
            "Root must have 6 descendant count"
        )
    }
    #[test]
    fn test_upgrade_returns_none_only_after_last_strong_owner_drops() {
        let root = new_root(0);
        let child = spawn_child(&root, 1);

        let weak_child = Rc::downgrade(&child);

        // both strong owners alive here; root and child
        assert!(weak_child.upgrade().is_some());
        println!("strong: {}", weak_child.strong_count(),);
        println!("weak  : {}", weak_child.weak_count(),);

        // remove owner #1: the strong clone sitting in root's children
        root.children.borrow_mut().pop();
        assert!(
            weak_child.upgrade().is_some(),
            "child binding is still alive, should still upgrade"
        );
        println!("strong: {}", weak_child.strong_count(),);
        println!("weak  : {}", weak_child.weak_count(),);

        // remove owner #2: the local `child` binding itself
        drop(child);

        assert!(
            weak_child.upgrade().is_none(),
            "no strong owners left, upgrade must fail now"
        );
        println!("strong: {}", weak_child.strong_count(),);
        println!("weak  : {}", weak_child.weak_count(),);
    }

    #[test]
    fn test_tree_frees_without_leak() {
        let root = new_root(0);
        let child = spawn_child(&root, 1);
        let grand_child = spawn_child(&child, 11);

        // SC count (self and inward edge):
        // root = 1, child = 2, grand_child = 2
        drop(grand_child); // grand_child's strong_count -= 1. child still points, so SC=1
        drop(child); // child's strong_count -= 1. Root still points, so SC=1

        // cascade drop: root -> child -> grandchild
        // root dropped -[child's sc=0]-> child dropped -[granchild's sc=0]-> grandchild dropped
        drop(root); // all strong owners dropped here


        println!("All owners dropped");

        // prints:
            // Dropping PID: 0
            // Dropping PID: 1
            // Dropping PID: 11
            // All owners dropped
    }
}
