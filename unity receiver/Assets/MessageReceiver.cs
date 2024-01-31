using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class MessageReceiver : MonoBehaviour
{
    public void ReceiveMessage(string message)
    {
        Debug.Log($"Received message: {message}");
        // Add your logic here to handle the message
    }
}