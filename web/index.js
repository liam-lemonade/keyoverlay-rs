class Settings {
    static port = 7685; // check the settings.json file for this
    static odometerAnimation = true; // should there be a keypress counter
    static odometerAnimationSpeed = "100ms"; // how fast should the animation for the counter play. set to 0 to disable animation
    static overrideKeyDisplay = [];
}

let socket = new ReconnectingWebSocket('ws://127.0.0.1:' + Settings.port + '/ws');

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

let lastPressed = [];
function handleKeyPress(data) {
    if (typeof data !== 'string') {
        throw "Attempted to call `handleKeyPress` where `data` was not typeof string";
    }

    let pressed = JSON.parse(data);

    let newKeysPressed = [];
    let keysReleased = [];

    // populate newKeysPressed and keysReleased
    {
        pressed.forEach(press => {
            if (!lastPressed.includes(press)) {
                newKeysPressed.push(press);
            }
        });

        lastPressed.forEach(press => {
            if (!pressed.includes(press)) {
                keysReleased.push(press);
            }
        });
    }

    // call respective handlers
    {
        newKeysPressed.forEach(key => {
            onKeyDown(findKey(key));
        });

        keysReleased.forEach(key => {
            onKeyUp(findKey(key));
        });
    }

    lastPressed = pressed;
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

    if (found === null) {
        // key has never been pressed before
        let added = new Key();
        added.text = text;
        
        addNewKeyHTML(added);
        keysList.push(added);

        return added;
    }
    else {
        // key has been pressed before
        return found;
    }
}

function onKeyDown(key) {
    if (!(key instanceof Key)) {
        throw "Attempted to call `onKeyDown` where `key` was not instanceof `Key`";
    }

    // update odometer
    if (Settings.odometerAnimation) {
        key.odometer.update(++key.counter);
    }

    // set background-alpha
    key.div.style = "background-color: var(--fill-color); transition: background-color var(--fill-animation-speed) linear;"
}

function onKeyUp(key) {
    if (!(key instanceof Key)) {
        throw "Attempted to call `onKeyUp` where `key` was not instanceof `Key`";
    }

    // un-set background alpha
    key.div.style = "background-color: transparent; transition: background-color var(--fill-animation-speed) linear;"
}

function addNewKeyHTML(keypress) {
    if (!(keypress instanceof Key)) {
        throw "Attempted to call `addNewKeyHTML` where `keypress` was not instanceof `Key`";
    }

    // create parent div
    keypress.div = document.createElement("div");
    keypress.div.id = "keybox-" + keypress.text;
    keypress.div.className = "keybox";
    
    document.getElementById("keys").appendChild(keypress.div);

    // create odometer
    keypress.odometer = document.createElement("div");
    keypress.odometer.id = "odometer";
    keypress.div.appendChild(keypress.odometer);
    
    keypress.odometer = new Odometer({
        el: keypress.odometer,
        value: 0,
    });

    // fill text in `keybox-text`
    keypress.keytext = document.createElement("div");
    keypress.keytext.id = "keybox-text";
    keypress.keytext.innerHTML = keypress.text;
    keypress.div.appendChild(keypress.keytext);
}

// main
document.querySelector(':root').style.setProperty("--duration", Settings.odometerAnimationSpeed);



// https://www.w3schools.com/css/tryit.asp?filename=trycss3_gradient-linear_trans