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

class KeyHistory {
    div = null;
    pressed = false;

    start = 0;
    end = 0;
}

class Key {
    text = "";
    counter = 0;

    div = null;
    keytext = null;
    odometer = null;

    history = [];
}

// the list of keys that have been pressed so far
let keysList = [];

function handleKeyPress(data) {
    if (typeof data !== 'string') {
        throw "Attempted to call `handleKeyPress` where `data` was not typeof string";
    }

    if (data == "reset") {
        keysList.forEach(key => {
            key.counter = 0;
            key.odometer.update(key.counter);
        })

        return;
    }

    let pair = JSON.parse(data);

    down = pair[0] == 1;

    if (down) {
        onKeyDown(pair[1]);
    }
    else {
        onKeyUp(pair[1]);
    }
}

function addKeyFromString(text) {
    if (typeof text !== 'string') {
        throw "Attempted to call `addKeyFromString` where `text` was not typeof string"
    }

    let added = new Key();
    added.text = text;

    addNewKeyHTML(added);
    keysList.push(added);

    return added;
}

function findKey(text) {
    if (typeof text !== 'string') {
        throw "Attempted to call `findKey` where `text` was not typeof string"
    }

    // has this key been pressed before?
    let found = null;
    keysList.every(key => {
        if (key.text === text) {
            found = key;
            return false; // array.every breaks on false
        }

        return true; // array.every must have a return false statement
    });

    return found;
}

function onKeyDown(text) {
    let key = findKey(text);

    if (key === null) {
        key = addKeyFromString(text);
    }

    if (!(key instanceof Key)) {
        throw "Attempted to call `onKeyDown` where `key` was not instanceof `Key`";
    }

    // update odometer
    if (Settings.odometerAnimation) {
        key.odometer.update(++key.counter);
    }

    handleKeyHistory(key, true);

    // set background-alpha
    key.div.style = "background-color: var(--fill-color); transition: background-color var(--fill-animation-speed) linear;"
}

function onKeyUp(text) {
    let key = findKey(text);

    if (key === null) {
        return;
    }

    if (!(key instanceof Key)) {
        throw "Attempted to call `onKeyUp` where `key` was not instanceof `Key`";
    }

    handleKeyHistory(key, false);

    // un-set background alpha
    key.div.style = "background-color: transparent; transition: background-color var(--fill-animation-speed) linear;"
}

function addNewKeyHTML(keypress) {
    if (!(keypress instanceof Key)) {
        throw "Attempted to call `addNewKeyHTML` where `keypress` was not instanceof `Key`";
    }

    // create parent div
    keypress.div = document.createElement("div");
    keypress.div.className = "keybox";

    document.getElementById("keys").appendChild(keypress.div);

    // create odometer
    if (Settings.odometerAnimation) {
        keypress.odometer = document.createElement("div");
        keypress.odometer.className = "counter";
        keypress.div.appendChild(keypress.odometer);

        keypress.odometer = new Odometer({
            el: keypress.odometer,
            value: 0,
        });
    }

    // fill text in `keybox-text`
    keypress.keytext = document.createElement("div");
    keypress.keytext.className = "keybox-text";

    keypress.keytext.innerHTML = keypress.text;

    keypress.div.appendChild(keypress.keytext);
}

function handleKeyHistory(keypress, down) {
    if (!Settings.showHistory) {
        return;
    }

    if (!(keypress instanceof Key)) {
        throw "Attempted to call `handleKeyHistory` where `keypress` was not instanceof `Key`";
    }

    if (down) {
        let history = new KeyHistory();

        // add new history div with class #history
        history.div = document.createElement("div");
        history.div.className = "history";
        history.div.style = "--length: 0px;"
        history.pressed = true;

        keypress.div.appendChild(history.div);
        keypress.history.push(history);
    }
    else {
        let history = keypress.history[keypress.history.length - 1];
        history.pressed = false;
    }
    /*
    else {
        let history = keypress.history[keypress.history.length - 1];
        let length = history.div.style.getPropertyValue("--length");
        history.style = "--length: " + length + "; animation: moveUp var(--history-time) linear forwards;";
    }
    */
}

// main
document.querySelector(':root').style.setProperty("--duration", Settings.odometerAnimationSpeed);

let lastUpdate = Date.now();
setInterval(function () {
    let current = Date.now();
    let delta = (current - lastUpdate) / 1000;

    keysList.forEach(key => {
        key.history.forEach(history => {
            history.start += Settings.historyPixelsPerSecond * delta;

            if (history.pressed) {
                history.end = 0;
            }
            else {
                history.end += Settings.historyPixelsPerSecond * delta;
            }

            let height = history.start - history.end;
            history.div.style = "--length: " + height + "px; transform: translateY(-" + history.end + "px);";
        });
    });

    lastUpdate = current;
}, 0);

// https://www.w3schools.com/css/tryit.asp?filename=trycss3_gradient-linear_trans