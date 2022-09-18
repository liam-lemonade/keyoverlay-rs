const daemonPort = 7685;
let socket = new ReconnectingWebSocket('ws://127.0.0.1:' + daemonPort + '/ws');

socket.onopen = () => {
    console.log("Successfully Connected");
};

socket.onclose = event => {
    console.log("Socket Closed Connection: ", event);
};

socket.onerror = error => {
    console.log("Socket Error: ", error);
};

socket.onmessage = event => {
    console.log(event.data);
}