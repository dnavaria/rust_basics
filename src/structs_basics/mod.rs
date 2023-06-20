struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn build_user(email: String, username: String) -> User{
    User{
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle{
    fn square(size: u32) -> Rectangle{
        Rectangle{
            width: size,
            height: size,
        }
    }
}


pub fn basic(){
    let user1 = User{
        email: String::from("sam@gmail.com"),
        username: String::from("sam"),
        active: true,
        sign_in_count: 1,
    };
    println!("User1 => {} {} {} {}", user1.email, user1.username, user1.active, user1.sign_in_count);

    let user2 = build_user(String::from("Priyanka"), String::from("pri@gmail.com"));
    println!("User2 => {} {} {} {}", user2.email, user2.username, user2.active, user2.sign_in_count);

    let user3 = User{
        email: String::from("Tinker"),
        username: String::from("tk@gmail.com"),
        ..user1
    };
    println!("User3 => {} {} {} {}", user3.email, user3.username, user3.active, user3.sign_in_count);

    // Tuple Structs
    // struct Color(i32, i32, i32);
    // struct Point(i32, i32, i32);

    let rect = Rectangle{
        width: 30,
        height: 50,
    };
    println!("Rectangle => {:#?}", rect);
    println!("Area of rectangle => {}", rect.area());

    let rect1 = Rectangle{
        width: 10,
        height: 40,
    };

    let rect2 = Rectangle{
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? => {}", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect1? => {}", rect2.can_hold(&rect1));

    let sq = Rectangle::square(3);
    println!("Square => {:#?}", sq);

}

