pub const CONTROL_PANEL: &str = "<!DOCTYPE html>
<html>
    <head>
        <title>music control panel</title>
        <link rel=\"stylesheet\" href=\"/style.css\" />
        <script src=\"/script.js\"></script>
    </head>
    <body>
        <div id=\"all_songs\"></div>
        <div id=\"right_side\">
            <div id=\"controls\">
                <h1 id=\"now_playing\"></h1>
                <input id=\"timer_slider\" type=\"range\" oninput=\"seek_time();\" />
                <h3 id=\"timer_text\"></h3>
                <input type=\"button\" value=\"prev\" onclick=\"prev_song();\" />
                <input type=\"button\" value=\"pause\" onclick=\"toggle_pause();\" />
                <input type=\"button\" value=\"next\" onclick=\"next_song();\" />
            </div>
            <div id=\"queue_div\">
                <h2>queue</h2>
                <ol id=\"queue\"></ol>
            </div>
        </div>
    </body>
</html>";

pub const STYLE: &str = "body {
    height: 100vh;
    display: flex;
    margin: 0;
}

#all_songs {
    overflow-y: scroll;
    border-right: solid 1px black;
}

#right_side {
    display: flex;
    flex-direction: column;
    flex-grow: 32;
}

#controls {
    border-bottom: solid 1px black;
}

#queue_div {
    overflow-y: scroll;
}

.current_song {
    font-weight: bold;
}

#timer_slider {
    width: 100%;
}";

pub const SCRIPT: &str = "function song_obj_to_string(song) {
    if (song == null) {
        return \"nothing\";
    }
    return song[\"artist\"] + \" -- \" + song[\"title\"];
}

function to_timer(seconds) {
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

        document.getElementById(\"now_playing\").textContent = \"now playing: \" + song_obj_to_string(response_as_object[\"now_playing\"]);

        let elapsed = response_as_object[\"elapsed\"];
        let duration = response_as_object[\"duration\"];
        document.getElementById(\"timer_text\").textContent = to_timer(elapsed) + \"/\" + to_timer(duration);
        
        let timer_slider = document.getElementById(\"timer_slider\");
        timer_slider.setAttribute(\"max\", duration);
        timer_slider.value = elapsed;

        let queue_element = document.getElementById(\"queue\");
        queue_element.innerHTML = \"\";
        let queue_array = response_as_object[\"queue\"];
        for (let i = 0; i < queue_array.length; i++) {
            let list_item = document.createElement(\"li\");

            let remove_button = document.createElement(\"input\");
            remove_button.setAttribute(\"type\", \"button\");
            remove_button.setAttribute(\"value\", \"x\");
            remove_button.onclick = function() {remove_from_queue(i)};
            list_item.appendChild(remove_button);

            list_item.appendChild(document.createTextNode(song_obj_to_string(queue_array[i])));
            if (response_as_object[\"queue_pos\"] == i) {
                list_item.className = \"current_song\";
            }

            queue_element.appendChild(list_item);
        }
    });
}

function seek_time() {
    let time_to_go_to = document.getElementById(\"timer_slider\").value;
    fetch(\"/seek\", {
        method: \"POST\",
        headers: {\"Content-Type\": \"text/plain\"},
        body: time_to_go_to
    }).then((body) => {update_info();});
}

function prev_song() {
    fetch(\"/prev\", {
        method: \"POST\"
    }).then((body) => {update_info();});
}

function toggle_pause() {
    fetch(\"/pause\", {
        method: \"POST\"
    }).then((body) => {update_info();});
}

function next_song() {
    fetch(\"/next\", {
        method: \"POST\"
    }).then((body) => {update_info();});
}

function add_to_queue(file) {
    fetch(\"/addsong\", {
        method: \"POST\",
        headers: {\"Content-Type\": \"text/plain\"},
        body: file
    }).then((body) => {update_info();});
}

function remove_from_queue(position) {
    fetch(\"/removesong\", {
        method: \"POST\",
        headers: {\"Content-Type\": \"text/plain\"},
        body: position
    }).then((body) => {update_info();});
}

function get_all_songs() {
    fetch(\"/allsongs\").then((response) => response.text()).then((songs) => {
        let song_list = JSON.parse(songs);
        let song_div = document.getElementById(\"all_songs\");
        song_div.innerHTML = \"\";
        for (let i = 0; i < song_list.length; i++) {
            let song = song_list[i];
            let queue_button = document.createElement(\"input\");
            queue_button.setAttribute(\"type\", \"button\");
            queue_button.setAttribute(\"value\", song_obj_to_string(song));
            queue_button.onclick = function() {add_to_queue(song[\"file\"]);};
            let song_p = document.createElement(\"p\");
            song_p.appendChild(queue_button);
            song_div.appendChild(song_p);
        }
    })
}

window.onload = function() {
    get_all_songs();
    update_info();
    window.setInterval(update_info, 1000);
};";

