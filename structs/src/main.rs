struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn main() {
    
    let mut user1 = User {
        email: String::from("example@example.com"),
        username: String::from("kaisersoze"),
        active: true,
        sign_in_count: 1
    };
    // To have a mutable field, entire struct instance should be mutable.
    user1.email = String::from("newemail@example.com");

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("user2@example.com"),
        sign_in_count: 1
    };

    // Using .. syntax, the remaining fields not explicitly set 
    // should have the same values with user2
    let user3 = User {
        email: String::from("user3@example.com"),
        ..user2
    };

    println!("{}", user2.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username, 
        active: true,
        sign_in_count: 1
    }
}

fn build_user_shorthand(email: String, username: String) -> User {
    //Because the email field and the email parameter have the same name, 
    // we only need to write email rather than email: email
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}
