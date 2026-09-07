
#[allow(unused_variables,dead_code)]
// Example code around structs
fn main() {

    let user = User {
        email: String::from("me@gmail.com"),
        username: String::from("Me"),
        active: false,
        sign_in_count: 0,
    };
    println!("{}", user.email);
    let mut user2 = User {
        email: String::from("me@gmail.com"),
        username: String::from("Me"),
        active: false,
        sign_in_count: 1,
    };
    assert!(!user2.is_active());
    user2.set_inactive();
    assert!(user2.is_active());
    assert!(user2.sign_in_count > 0);
    user2.email = String::from("me2@gmail.com");
    let user3 = build_user(String::from("kek@kek.com"), String::from("Kek"));

    let user4 = User::default("keks2".to_string(), "kek2@kek2.com".to_string());

    assert!(!user4.same_name(&user3));

    println!("{:#?}", user4); // pretty print thanks to the Debug trait

    let color = assign_tuple(0, 1, 2);
    println!("{:#?}", color); // pretty print thanks to the Debug trait

    let point = Point(0, 1, 2);
    println!("{:#?}", point); // pretty print thanks to the Debug trait
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: i64,
    active: bool,
}

impl User {
    // Immutable operation, use self
    fn is_active(&self) -> bool {
        self.active
    }

    // State mutation on self if self is mut
    fn set_inactive(&mut self) {
        self.active = false
    }

    fn same_name(&self, u2: &User) -> bool {
        self.username == u2.username
    }

    // Static method User::default
    fn default(username: String, email: String) -> User {
        User {
            email,
            username,
            sign_in_count: 0,
            active: false
        }
    }
}

// Infix notation, if the field and the variable have the same name the shorthand notation can be used
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// Named structs

#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

fn assign_tuple(i1: i32, i2: i32, i3: i32) -> Color {
    Color(i1, i2, i3)
}

// Warning : holding references in structs (using '&') requires additional lifetime indicators, see chapter 10
