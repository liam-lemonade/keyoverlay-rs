class Settings {
    static port = 7686; // check the settings.json file for this
    static odometerAnimation = true; // should there be a keypress counter
    static odometerAnimationSpeed = "100ms"; // how fast should the animation for the counter play. set to 0 to disable animation

    static showHistory = true;
    static historyPixelsPerSecond = 1000;
}

let socket = new ReconnectingWebSocket("ws://127.0.0.1:" + Settings.port + "/ws");

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
    div;
    pressed;

    start;
    end;

    constructor() {
        this.div = null;
        this.pressed = true;

        this.start = 0;
        this.end = 0;
    }
}

class Key {
    text;
    index;
    counter;

    div;
    keytext;
    odometer;

    history;

    constructor(text, index) {
        this.text = text;
        this.index = index;
        this.counter = 0;

        this.div = null;
        this.keytext = null;
        this.odometer = null;

        this.history = [];
    }
}

// the list of keys that have been pressed so far
let keyList = [];

function handleKeyPress(data) {
    if (typeof data !== 'string') {
        throw "Attempted to call `handleKeyPress` where `data` was not typeof string";
    }

    if (data == "reset") {
        keyList.forEach(key => {
            key.counter = 0;

            if (key.odometer !== null) {
                key.odometer.update(key.counter);
            }
        })

        return;
    }

    let json = JSON.parse(data);

    let key = json[0];
    let down = json[1];

    if (down) {
        onKeyDown(key, json[2]); // json[2] == index
    }
    else {
        onKeyUp(key);
    }
}

function onKeyDown(text, index) {
    if (typeof text !== 'string') {
        throw "Attempted to call `onKeyDown` where `text` was not typeof `string`";
    }

    if (typeof index !== 'number') {
        throw "Attempted to call `addKeyFromString` where `index` was not typeof `number`"
    }

    let key = findKey(text);

    if (key === null) {
        console.log("Adding key: " + text);
        key = addKeyFromString(text, index);
    }

    // update odometer
    if (Settings.odometerAnimation && key.odometer !== null) {
        key.odometer.update(++key.counter);
    }

    handleKeyHistory(key, true);

    // set background-alpha
    key.div.style = "background-color: var(--fill-color); transition: background-color var(--fill-animation-speed) linear;"
}

function onKeyUp(text) {
    if (typeof text !== 'string') {
        throw "Attempted to call `onKeyDown` where `text` was not typeof `string`";
    }

    let key = findKey(text);

    if (key === null) {
        return;
    }

    handleKeyHistory(key, false);

    // un-set background alpha
    key.div.style = "background-color: transparent; transition: background-color var(--fill-animation-speed) linear;"
}

function handleKeyHistory(key, down) {
    if (!Settings.showHistory) {
        return;
    }

    if (!(key instanceof Key)) {
        throw "Attempted to call `handleKeyHistory` where `keypress` was not instanceof `Key`";
    }

    if (down) {
        let history = new KeyHistory();

        // add new history div with class #history
        history.div = document.createElement("div");
        history.div.className = "history";
        history.div.style = "--length: 0px;"

        key.div.appendChild(history.div);
        key.history.push(history);
    }
    else {
        let history = key.history[key.history.length - 1];
        history.pressed = false;
    }
}

function findKey(text) {
    if (typeof text !== 'string') {
        throw "Attempted to call `findKey` where `text` was not typeof string"
    }

    // has this key been pressed before?
    let found = null;
    keyList.every(key => {
        if (key.text === text) {
            found = key;
            return false; // array.every breaks on false
        }

        return true; // array.every must have a return false statement
    });

    return found;
}

function addKeyFromString(text, index) {
    if (typeof text !== 'string') {
        throw "Attempted to call `addKeyFromString` where `text` was not typeof string"
    }

    if (typeof index !== 'number') {
        throw "Attempted to call `addKeyFromString` where `index` was not typeof `number`"
    }

    let added = new Key(text, index);

    addNewKeyHTML(added);
    keyList.push(added);

    return added;
}

function addNewKeyHTML(key) {
    if (!(key instanceof Key)) {
        throw "Attempted to call `addNewKeyHTML` where `key` was not instanceof `Key`";
    }

    // create parent div
    key.div = document.createElement("div");
    key.div.className = "keybox";

    let addAbove = null;
    keyList.forEach(checking => {
        if (checking.index > key.index) {
            addAbove = checking;
        }
    });

    if (addAbove === null) {
        document.getElementById("keys").appendChild(key.div);
    }
    else {
        //console.log(addAbove.div.className);
        document.getElementById("keys").insertBefore(key.div, addAbove.div);
    }

    // create odometer
    if (Settings.odometerAnimation) {
        key.odometer = document.createElement("div");
        key.odometer.className = "counter";
        key.div.appendChild(key.odometer);

        key.odometer = new Odometer({
            el: key.odometer,
            value: 0,
        });
    }

    // fill text in `keybox-text`
    key.keytext = document.createElement("div");
    key.keytext.className = "keybox-text";

    key.keytext.innerHTML = key.text;

    key.div.appendChild(key.keytext);
}

function isVisible(element) {
    return element.getBoundingClientRect().bottom > 0;
}

// main
document.querySelector(':root').style.setProperty("--duration", Settings.odometerAnimationSpeed);

let lastUpdate = Date.now();
setInterval(function () {
    let current = Date.now();
    let delta = (current - lastUpdate) / 1000;

    keyList.forEach(key => {
        for (var i = 0; i < key.history.length; i++) {
            let history = key.history[i];

            history.start += Settings.historyPixelsPerSecond * delta;

            if (history.pressed) {
                history.end = 0;
            }
            else {
                history.end += Settings.historyPixelsPerSecond * delta;
            }

            let height = history.start - history.end;
            history.div.style = "--length: " + height + "px; transform: translateY(-" + history.end + "px);";

            if (!isVisible(history.div)) {
                history.div.remove();
                key.history.splice(i--, 1);
            }
        }
    });

    lastUpdate = current;
}, 0);


// https://www.w3schools.com/css/tryit.asp?filename=trycss3_gradient-linear_trans