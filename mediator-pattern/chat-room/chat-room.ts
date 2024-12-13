// Mediator
class ChatRoom {
  showMessage(user: string, msg: string) {
    console.log(`[${user}] :  ${msg}`);
  }
}

// Colleague
class User {
  private name: string;
  private chatroom: ChatRoom;

  constructor(name: string, chatroom: ChatRoom) {
    this.name = name;
    this.chatroom = chatroom;
  }

  sendMessage(message: string): void {
    this.chatroom.showMessage(this.name, message);
  }
}

// Usage
const chatroom = new ChatRoom();

const u1 = new User("Shreyash", chatroom);
const u2 = new User("Arpit", chatroom);

u1.sendMessage("Hola");
u2.sendMessage("Hi Shreyash, How are you ?");
