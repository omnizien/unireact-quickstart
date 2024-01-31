using UnityEngine;
using UnityEngine.UI; // Import the UI namespace to use Text components

public class DisplayMessage : MonoBehaviour
{
    public Text messageText; // Assign this in the inspector

    // Method called from React
    public void UpdateMessage(string message)
    {
        if (messageText != null)
        {
            messageText.text = message;
        }
    }
}
