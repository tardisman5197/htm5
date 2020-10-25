import React, { useState } from 'react';

interface Message {
  message: string;
  sender: string;
  receiver: string;
}

const TEST_MESSAGES: Message[] = [{
  message: 'test testing 123',
  sender: 'user1',
  receiver: 'user2'
}, {
  message: 'yep it works!',
  sender: 'user2',
  receiver: 'user1'
}];

/** Given global messages, return username of all users this user has had conversations with. */
const getConversations = (user: string, messages: Message[]) => messages.filter(message => message.receiver === user || message.sender === user).map(message => message.sender === user ? message.receiver : message.sender);

/** Given global messages, return all messages between two users. */
const getConversationMessages = (user: string, viewing: string, messages: Message[]) => messages.filter(message => (message.receiver === user && message.sender === viewing) || (message.sender === user && message.receiver === viewing));

const App = () =>{
  const [user, setUser] = useState<string | null>(null);
  const [viewing, setViewing] = useState<string | null>();
  const [messages] = useState<Message[]>(TEST_MESSAGES);

  const [usernameInput, setUsernameInput] = useState<string>('');

  if (!user) {
    return (
      <form onSubmit={() => setUser(usernameInput)}>
        <input type="text" placeholder="Username" onChange={event => setUsernameInput(event.target.value)} />
        <input type="submit" />
      </form>
    );
  }

  if (!viewing) {
    return (
      <div>
        <h2>Logged in as {user}</h2>
        <button onClick={() => setUser(null)}>Logout</button>
        <ul>
          {getConversations(user, messages).map((sender, i) => (
            <li key={i}>
              <button onClick={() => setViewing(sender)}>{sender}</button>
            </li>
          ))}
        </ul>
      </div>
    );
  }

  return (
    <div>
      <h2>Messages with {viewing}</h2>
      <button onClick={() => setViewing(null)}>Back</button>
      <ul>
        {getConversationMessages(user, viewing, messages).map((message, i) => (
          <li key={i}>{message.message}</li>
        ))}
      </ul>
    </div>
  );
};

export default App;
