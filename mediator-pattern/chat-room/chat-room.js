// Mediator
var ChatRoom = /** @class */ (function () {
    function ChatRoom() {
    }
    ChatRoom.prototype.showMessage = function (user, msg) {
        console.log("[".concat(user, "] :  ").concat(msg));
    };
    return ChatRoom;
}());
// Colleague
var User = /** @class */ (function () {
    function User(name, chatroom) {
        this.name = name;
        this.chatroom = chatroom;
    }
    User.prototype.sendMessage = function (message) {
        this.chatroom.showMessage(this.name, message);
    };
    return User;
}());
// Usage
var chatroom = new ChatRoom();
var u1 = new User("Shreyash", chatroom);
var u2 = new User("Arpit", chatroom);
u1.sendMessage("Hola");
u2.sendMessage("Hi Shreyash, How are you ?");
