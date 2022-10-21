use list_impl::List;

fn main() {
    let mut list = List::new();

    list.push_front(30);
    list.push_front(40);
    list.push_front(50);
    list.push_front(60);
    println!("{:?} size: {}", list.to_vector(), list.size());
}