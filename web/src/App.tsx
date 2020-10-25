import React, { useState } from 'react';
import styled from 'styled-components';

interface Message {
  message: string;
  sender: string;
  receiver: string;
  artist: string;
  song: string;
  link: string;
  timesent: Date;
  timeread?: Date;
}

const Root = styled.div`
  width: 500px;
  margin: 2rem auto;
  padding: 1rem;
  background-color: #eee;
  min-height: 800px;
  display: flex;
  flex-direction: column;
`;

const MessagesList = styled.ul`
  list-style: none;
  padding: 0;
  display: flex;
  flex-direction: column;
  flex: 1 1 auto;
`;

const MessageItem = styled.li<{ right: boolean; }>`
  align-self: ${props => props.right ? 'flex-end' : 'flex-start'};
  margin-bottom: 1rem;
`;

const MessageItemContent = styled.div<{ right: boolean; }>`
  background-color: ${props => props.right ? 'blue' : '#111'};
  color: white;
  padding: 0.5rem;
  border-radius: 4px;
`;

const MessageItemMeta = styled.div<{ right: boolean; }>`
  text-align: ${props => props.right ? 'inline-end' : 'inline-start'};;
  font-size: 0.75rem;
  color: #666;
`;

const TEST_MESSAGES: Message[] = [{
  message: 'test testing 123',
  sender: 'user1',
  receiver: 'user2',
  artist: 'Artistname',
  song: 'Somesong',
  link: '#todo',
  timesent: new Date('2020-10-21')
}, {
  message: 'yep it works!',
  sender: 'user2',
  receiver: 'user1',
  artist: 'Anotherartist',
  song: 'Othersong',
  link: '#alsotodo',
  timesent: new Date('2020-10-22')
}];

/** Given global messages, return username of all users this user has had conversations with. */
const getConversations = (user: string, messages: Message[]) => Object.keys(messages.filter(message => message.receiver === user || message.sender === user).reduce((acc, message) => ({ ...acc, [message.sender === user ? message.receiver : message.sender]: true }), {}));

/** Given global messages, return all messages between two users. */
const getConversationMessages = (user: string, viewing: string, messages: Message[]) => messages.filter(message => (message.receiver === user && message.sender === viewing) || (message.sender === user && message.receiver === viewing));

const App = () =>{
  const [user, setUser] = useState<string | null>(null);
  const [viewing, setViewing] = useState<string | null>();
  const [messages, setMessages] = useState<Message[]>(TEST_MESSAGES);

  const [usernameInput, setUsernameInput] = useState<string>('');
  const [viewingInput, setViewingInput] = useState<string>('');
  const [messageInput, setMessageInput] = useState<string>('');

  const sendMessage = (message: string, sender: string, receiver: string) => setMessages([...messages, { message, sender, receiver, artist: 'Willbebackend', song: 'That Generates This', link: '#aaaaa', timesent: new Date() }]);

  if (!user) {
    return (
      <Root>
        <h2>Login</h2>
        <p>Welcome to msglyrics, please enter your username to continue.</p>
        <form onSubmit={e => {
          e.preventDefault();
          setUser(usernameInput);
        }}>
          <input type="text" placeholder="Username" value={usernameInput} onChange={event => setUsernameInput(event.target.value)} />
          <input type="submit" />
        </form>
    </Root>
    );
  }

  if (!viewing) {
    return (
      <Root>
        <h2>Logged in as {user}</h2>
        <div>
          <button onClick={() => setUser(null)}>Logout</button>
        </div>
        <form onSubmit={e => {
          e.preventDefault();
          setViewing(viewingInput);
        }}>
          <input type="text" placeholder="Their username" value={viewingInput} onChange={event => setViewingInput(event.target.value)} />
          <input type="submit" />
        </form>
        <ul>
          {getConversations(user, messages).map((sender, i) => (
            <li key={i}>
              <button onClick={() => setViewing(sender)}>{sender}</button>
            </li>
          ))}
        </ul>
      </Root>
    );
  }

  return (
    <Root>
      <h2>Messages with {viewing}</h2>
      <div>
        <button onClick={() => setViewing(null)}>Back</button>
      </div>
      <MessagesList>
        {getConversationMessages(user, viewing, messages).map((message, i) => (
          <MessageItem key={i} right={message.sender === user}>
            <MessageItemContent right={message.sender === user}>{message.message}</MessageItemContent>
        <MessageItemMeta right={message.sender === user}>{message.timesent.toLocaleString()}, <a href={message.link}>{message.artist} - {message.song}</a></MessageItemMeta>
          </MessageItem>
        ))}
      </MessagesList>
      <form onSubmit={e => {
        e.preventDefault();
        sendMessage(messageInput, user, viewing);
        setMessageInput('');
      }}>
        <input type="text" placeholder="Send a message..." value={messageInput} onChange={event => setMessageInput(event.target.value)} />
        <input type="submit" />
      </form>
    </Root>
  );
};

export default App;
