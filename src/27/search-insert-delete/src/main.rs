// The search-insert-delete problem

// Three kinds of threads share access to a singly-linked list:
// searchers, inserters and deleters. Searchers merely examine the list;
// hence they can execute concurrently with each other. Inserters add
// new items to the end of the list; insertions must be mutually exclusive to preclude two inserters from inserting new items at about
// the same time. However, one insert can proceed in parallel with
// any number of searches. Finally, deleters remove items from anywhere in the list. At most one deleter process can access the list at
// a time, and deletion must also be mutually exclusive with searches and insertions.
use std::collections::LinkedList;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mut list: LinkedList<u32> = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    let mut mutex: Arc<Mutex<LinkedList<u32>>> = Arc::new(Mutex::new(list));

    let mut searches = vec![];

    searches.push(create_searcher_thread(&mut mutex, "ST1"));
    searches.push(create_searcher_thread(&mut mutex, "ST2"));

    for handle in searches {
        handle.join().unwrap();
    }
}

fn create_searcher_thread(
    mutex: &mut Arc<Mutex<LinkedList<u32>>>,
    name: &'static str,
) -> thread::JoinHandle<()> {
    let list = mutex.clone();
    let t = thread::spawn(move || {
        let locked_list = list.lock().unwrap();
        println!("Searcher thread {}: {:?}", name, *locked_list);
        for item in locked_list.deref() {
            println!("Searcher thread {}: {}", name, item);
        }
    });

    return t;
}
