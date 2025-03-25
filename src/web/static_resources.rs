pub const CONTROL_PANEL: &str = "<!DOCTYPE html>
<html>
    <head>
        <title>music control panel</title>
        <link rel=\"stylesheet\" href=\"/style.css\" />
        <script src=\"/script.js\"></script>
    </head>
    <body>
        <h1 id=\"now_playing\"></h1>
        <h3 id=\"timer\"></h3>
        <input type=\"button\" value=\"prev\" onclick=\"prev_song();\" />
        <input type=\"button\" value=\"pause\" onclick=\"toggle_pause();\" />
        <input type=\"button\" value=\"next\" onclick=\"next_song();\" />
        <h2>queue:</h2>
        <ol id=\"queue\"></ol>
    </body>
</html>";

pub const STYLE: &str = ".current_song {
    font-weight: bold;
}";

pub const SCRIPT: &str = "function to_timer(seconds) {
    let mins = Math.floor(seconds / 60);
    if (mins < 10) {
        mins = \"0\" + mins;
    }
    let secs = seconds % 60;
    if (secs < 10) {
        secs = \"0\" + secs;
    }
    return mins + \":\" + secs;
}

function update_info() {
    fetch(\"/update\").then((response) => response.text()).then((json_response) => {
        let response_as_object = JSON.parse(json_response);

        document.getElementById(\"now_playing\").textContent = \"now playing: \" + response_as_object[\"now_playing\"];

        document.getElementById(\"timer\").textContent = to_timer(response_as_object[\"elapsed\"]) + \"/\" + to_timer(response_as_object[\"duration\"]);

        let queue_element = document.getElementById(\"queue\");
        queue_element.innerHTML = \"\";
        let queue_array = response_as_object[\"queue\"];
        for (let i = 0; i < queue_array.length; i++) {
            let list_item = document.createElement(\"li\");
            list_item.appendChild(document.createTextNode(queue_array[i]));
            if (response_as_object[\"queue_pos\"] == i) {
                list_item.className = \"current_song\";
            }
            queue_element.appendChild(list_item);
        }
    });
}

function prev_song() {
    fetch(\"/prev\");
    update_info();
}

function toggle_pause() {
    fetch(\"/pause\");
    update_info();
}

function next_song() {
    fetch(\"/next\");
    update_info();
}

window.onload = function() {
    update_info();
    window.setInterval(update_info, 1000);
};";

pub const NOT_FOUND: &str = "<!DOCTYPE html>
<html>
    <head>
        <title>404 Not Found</title>
    </head>
    <body>
        <h1>404 Not Found</h1>
    </body>
</html>";

