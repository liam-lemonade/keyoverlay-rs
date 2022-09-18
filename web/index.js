class Settings {
    static port = 7685;
    static odometerAnimation = true;
    static odometerAnimationSpeedMs = 100;
}

let socket = new ReconnectingWebSocket('ws://127.0.0.1:' + Settings.port + '/ws');

socket.onopen = () => {
    odometer.innerHTML = 0;
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
    //console.log(event.data);
}

class Key {
    text = "";
    counter = 0;
}

let keysList = [];

function handleKeyPress(data) {
    if (data !== "[]") {
        let pressedKeys = [];
        
        // Parse JSON that looks like ["X", "Z"]
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
            if (found === null) {
                let added = new Key();
                added.text = jsonKey;
                
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
            key.counter++
        });

        // update animation
        if (Settings.odometerAnimation) {
            // TODO: implement something that adds new odometers for every key in `keysList`
            odometer.innerHTML = pressedKeys[0].counter;
        }
    }
}

// Main
document.querySelector(':root').style.setProperty("--duration", Settings.odometerAnimationSpeedMs + "ms");