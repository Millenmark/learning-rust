fn main() {
    fn consume_user(u: User) {
        println!("Consuming: {}", u.name);
    }

    struct User {
        name: String,
    }

    impl User {
        fn new(name: String) -> Self {
            Self { name }
        }
    }

    let user1 = User::new("Alice".to_owned());
    let user2 = user1;

    consume_user(user2);
    consume_user(user2);

    // AI autocomplete
    // use &(immutable borrow)
    let user3 = &user1;
    consume_user(user3);

    // use &mut (mutable borrow)
    let user4 = &mut user1;
    consume_user(user4);

    // use std::mem::take to move the value out of the mutable borrow
    let user5 = std::mem::take(user4);
    consume_user(user5);
}
