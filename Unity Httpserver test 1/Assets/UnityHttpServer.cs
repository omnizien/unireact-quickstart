using System.Net;
using System.Threading;
using UnityEngine;
using System.IO; // Make sure to include this for StreamReader


public class UnityHttpServer : MonoBehaviour
{
    private HttpListener listener;
    private Thread listenerThread;

    void Start()
    {
        listener = new HttpListener();
        listener.Prefixes.Add("http://localhost:8080/unity-endpoint/");
        listener.Start();
        listenerThread = new Thread(StartListening);
        listenerThread.Start();
        Debug.Log("Unity HTTP Server started.");
    }

void StartListening()
{
    while (listener.IsListening)
    {
        var context = listener.GetContext(); // Blocks until a request is received
        var request = context.Request;
        var response = context.Response;

        // Read the request body
        string requestBody;
        using (var reader = new StreamReader(request.InputStream, request.ContentEncoding))
        {
            requestBody = reader.ReadToEnd();
        }

        // Print the received message to the Unity console
        Debug.Log($"Received message: {requestBody}");

        // Send a response back to the client
        var responseString = "Message received";
        var buffer = System.Text.Encoding.UTF8.GetBytes(responseString);
        response.ContentLength64 = buffer.Length;
        var responseOutput = response.OutputStream;
        responseOutput.Write(buffer, 0, buffer.Length);
        responseOutput.Close();
    }
}

    void OnApplicationQuit()
    {
        if (listener != null)
        {
            listener.Stop();
            listener.Close();
        }
        if (listenerThread != null)
        {
            listenerThread.Abort();
        }
    }
}
