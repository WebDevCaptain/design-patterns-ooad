// Mediator trait
trait ChatRoomMediator {
    fn show_message(&self, user: &str, message: &str);
}

// Concrete Mediator
struct ChatRoom;

impl ChatRoomMediator for ChatRoom {
    fn show_message(&self, user: &str, message: &str) {
        println!("{}: {}", user, message);
    }
}

// Colleague
struct User<'a> {
    name: &'a str,
    chatroom: &'a dyn ChatRoomMediator,
}

impl<'a> User<'a> {
    fn new(name: &'a str, chatroom: &'a dyn ChatRoomMediator) -> Self {
        Self { name, chatroom }
    }

    fn send_message(&self, message: &str) {
        self.chatroom.show_message(self.name, message);
    }
}

fn main() {
    let chatroom1 = ChatRoom;

    let u1 = User::new("Shreyash", &chatroom1);
    let u2 = User::new("Gaurav", &chatroom1);

    u1.send_message("Hello Gaurav");
    u2.send_message("Hi Shreyash, How are you ?");
}
