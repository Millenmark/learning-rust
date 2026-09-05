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
}
