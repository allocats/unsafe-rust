use std::alloc::{alloc, dealloc, handle_alloc_error, Layout};
use std::ptr;

#[repr(C)]
struct Node {
    data: i32,
    next: *mut Node
}

unsafe fn create_node(data: i32) -> *mut Node {
    let layout = Layout::new::<Node>();
    unsafe { 
        let ptr = alloc(layout) as *mut Node;

        if ptr.is_null() {
            handle_alloc_error(layout);
        }

        (*ptr).data = data;
        (*ptr).next = ptr::null_mut();

        ptr
    }
}

unsafe fn append_list(root: *mut *mut Node, data: i32) {
    unsafe {
        let new_node = create_node(data); 
        if (*root).is_null() {
            *root = new_node;
            return;
        }

        let mut current = *root;
        while !(*current).next.is_null() {
            current = (*current).next;
        }

        (*current).next = new_node;
    }
}

unsafe fn free_list(mut root: *mut Node) {
    let layout = Layout::new::<Node>();

    unsafe {
        while !root.is_null() {
            let temp = root;
            root = (*root).next;
            dealloc(temp as *mut u8, layout);
        }
    }
}

fn main() {
    println!("=== Unsafe Rust Learning - Linked List ===");

    unsafe {
        let mut root: *mut Node = ptr::null_mut();
        append_list(&mut root, 0);
        append_list(&mut root, 1);
        append_list(&mut root, 2);
        append_list(&mut root, 3);
        append_list(&mut root, 4);

        let mut current = root;
        while !current.is_null() {
            println!("\tNode: {}", (*current).data);
            current = (*current).next;
        }

        free_list(root);
    }
}
