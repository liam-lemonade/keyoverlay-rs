class Settings {
    static port = 7686; // check the settings.json file for this
    static odometerAnimation = true; // should there be a keypress counter
    static odometerAnimationSpeed = "100ms"; // how fast should the animation for the counter play. set to 0 to disable animation

    static showHistory = true;
    static historyPixelsPerSecond = 1000;
}

let socket = new ReconnectingWebSocket("ws://127.0.0.1:" + Settings.port);

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
    handleKeyPress(event.data);
}

function handleKeyPress(data) {
    if (typeof data !== 'string') {
        throw "Attempted to call `handleKeyPress` where `data` was not typeof string";
    }

    let pair = JSON.parse(data);

    down = pair[0] == 1;

    if (down) {
        onKeyDown(pair[1]);
    }
}

let animation = new CountUp('bpm', 0, 0, 0, .5, {
    useEasing: true,
    useGrouping: true,
    separator: " ",
    decimal: "."
});

let hasStarted = false;
let timeStarted = -1;
let presses = 0;

function onKeyDown(text) {
    presses++;
}

setInterval(function () {
    let timeElapsed = Date.now() - timeStarted;

    if (timeElapsed > 500) {
        let bpm = Math.ceil(((presses / (timeElapsed / 1000)) / 4) * 60)
        animation.update(bpm);
        //odometer.update(bpm);

        timeStarted = Date.now();
        presses = 0;
    }
}, 10)

// https://www.w3schools.com/css/tryit.asp?filename=trycss3_gradient-linear_trans