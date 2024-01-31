import React, { useState } from 'react';

const SendMessageToUnityButton = ({ sendMessage }) => {
  // State to hold the input text
  const [inputText, setInputText] = useState('');

  // Update the state with the input text
  const handleChange = (event) => {
    setInputText(event.target.value);
  };

  // Send the input text to Unity when the button is clicked
  const handleClick = () => {
    console.log(`Button clicked, sending message: ${inputText}`);
    sendMessage(inputText);
    setInputText(''); // Optional: Clear the input field after sending the message
  };

  return (
    <div>
      {/* Text input for entering the message */}
      <input
        type="text"
        value={inputText}
        onChange={handleChange}
        placeholder="Enter message here"
      />
      {/* Button to send the message */}
      <button onClick={handleClick}>Send Message to Unity</button>
    </div>
  );
};

export default SendMessageToUnityButton;
