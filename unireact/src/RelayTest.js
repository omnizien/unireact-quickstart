import React, { useState } from 'react';

const RelayTest = () => {
  const [inputMessage, setInputMessage] = useState('');

  const handleChange = (event) => {
    setInputMessage(event.target.value);
  };

  const handleClick = async () => {
    console.log(`Button clicked, sending message to server: ${inputMessage}`);
    try {
      const response = await fetch(`http://localhost:3333/send-to-unity`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ message: inputMessage }),
      });
      const data = await response.json();
      console.log('Response from server:', data);
    } catch (error) {
      console.error('Error sending message to server:', error);
    }
  };

  return (
    <div>
      <input type="text" value={inputMessage} onChange={handleChange} />
      <button onClick={handleClick}>Send Message to Unity via Server</button>
    </div>
  );
};

export default RelayTest;
