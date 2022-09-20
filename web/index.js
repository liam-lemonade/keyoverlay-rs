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
    html = null;

    history = [];
}

// the list of keys that have been pressed so far
let keysList = [];
let keyHistory = [];

function handleKeyPress(data) {
    if (typeof data !== 'string')
        throw "Attempted to call `handleKeyPress` where `data` was not typeof string";

    if (data != "[]") {
        let pressedKeys = [];
        
        // Parse JSON that looks like [ "X", "Z" ]
        let parsed = JSON.parse(data);

        // For each key currently pressed
        parsed.forEach(jsonKey => {
            let found = null;
            
            keysList.every(storedKey => {
                // if this key (keyString) has been pressed before
                // don't use .includes because of `counter` variable
                if (storedKey.text === jsonKey) {
                    found = storedKey;

                    return false; // .every breaks on return false
                }

                return true; // .every will also break if `true` is not returned at some point
            });

            // key is being pressed for the first time
            if (found == null) {
                let added = new Key();
                added.text = jsonKey;
                added.html = addNewKeyHTML(added);
                
                keysList.push(added);
                pressedKeys.push(added);
            }
            else {
                // key exists in list
                pressedKeys.push(found);
            }
        });

        // increase pressed key counter by 1
        pressedKeys.forEach(key => {
            key.counter++;
        });

        // update animation
        if (Settings.odometerAnimation) {
            let parent = document.getElementById("keys");
            parent.children.forEach(child => {
                child.odometer.innerHTML++;
            })
        }
    }
}

function addNewKeyHTML() {
    element = document.createElement("div");
    element.id = "keybox";
    element.innerHTML = "<div class=\"odometer\" id=\"odometer\">0</div>\n<div id=\"keybox-text\"></div>";
    document.getElementById("keys").appendChild(element);
}

// main
document.querySelector(':root').style.setProperty("--duration", Settings.odometerAnimationSpeed);



// https://www.w3schools.com/css/tryit.asp?filename=trycss3_gradient-linear_trans