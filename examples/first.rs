use insert_linked_list::LinkedList;

fn main() {
    let mut ll: LinkedList<u8> = LinkedList::new();

    ll.append(4);
    ll.append(6);
    ll.append(7);
    ll.append(8);
    ll.append(9);
    ll.insert(5, 1);
    // ll.add(2);

    let mut counter = 0;

    for elem in ll.iter() {
        if counter != 0 {
            print!(" -> ");
        }
        print!("{}", elem);
        counter = counter + 1;
    }
    print!("\n");
}