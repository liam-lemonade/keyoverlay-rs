class Settings {
    static port = 7686; // check the settings.json file for this
    static timeWindowMs = 1000;
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

let timestamps = []

function onKeyDown(text) {
    timestamps.push(Date.now());
}

let lastBpm = 0; // it starts at 0 anyway so no issues here

setInterval(function () {
    timestamps.forEach(function (value, index) {
        let delta = Date.now() - value

        if (delta > Settings.timeWindowMs) {
            timestamps.splice(index, 1);
        }
    });

    // calculate average
    let bpm = Math.ceil(((timestamps.length / (Settings.timeWindowMs / 1000)) / 4) * 60)

    if (lastBpm != bpm) {
        animation.update(bpm); // countUp.js does not enjoy being spammed
        lastBpm = bpm;
    }
}, 10)

// https://www.w3schools.com/css/tryit.asp?filename=trycss3_gradient-linear_trans