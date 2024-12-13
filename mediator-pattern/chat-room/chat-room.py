# Mediator
class ChatRoom:
    def show_message(self, user: str, msg: str):
        # Mediator handles displaying of messages
        print(f"[{user}]: {msg}")


# Colleague
class User:
    def __init__(self, name: str, chatroom: ChatRoom):
        self.name = name
        self.chatroom = chatroom  # User is associated with a chatroom mediator

    def send_message(self, message: str):
        # User sends message via chatroom mediator
        self.chatroom.show_message(self.name, message)


# Dry run
if __name__ == "__main__":
    chatroom = ChatRoom()

    user1 = User("Ayush", chatroom)
    user2 = User("Shreyash", chatroom)

    user1.send_message("Hello, Shreyash")
    user2.send_message("Hey Ayush")
