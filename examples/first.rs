use insert_linked_list::LinkedList;

fn main() {
    let mut ll: LinkedList<u8> = LinkedList::new();

    ll.add(5);
    ll.append(8);
    ll.add(2);

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