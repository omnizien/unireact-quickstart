import React, { useRef } from 'react';
import SendMessageToUnityButton from './SendMessageToUnityButton';
const UnityComponent = () => {
    const iframeRef = useRef(null); // Reference to the iframe
  
    const sendMessage = (message) => {
      console.log(`Sending message to Unity: ${message}`);
      const unityIframe = iframeRef.current;
      if (unityIframe && unityIframe.contentWindow) {
        unityIframe.contentWindow.postMessage(message, '*'); // Use specific domain instead of '*' for production
      }
    };
  
    return (
      <div>
        <iframe 
          ref={iframeRef}
          src={`/receiver/index.html`} 
          width="800" 
          height="600"
          title="Unity Game"
        ></iframe>
        <SendMessageToUnityButton sendMessage={sendMessage} />
      </div>
    );
  };
  

export default UnityComponent;
