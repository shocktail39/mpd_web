pub const CONTROL_PANEL: &str = "<!DOCTYPE html>
<html>
    <head>
        <title>music control panel</title>
        <link rel=\"stylesheet\" href=\"/style.css\" />
        <script src=\"/script.js\"></script>
    </head>
    <body>
        <div id=\"all_songs\"></div>
        <div id=\"controls\">
            <h1 id=\"now_playing\"></h1>
            <input id=\"timer_slider\" type=\"range\" oninput=\"seek_time();\" />
            <h3 id=\"timer_text\"></h3>
            <input type=\"button\" value=\"prev\" onclick=\"prev_song();\" />
            <input type=\"button\" value=\"pause\" onclick=\"toggle_pause();\" />
            <input type=\"button\" value=\"next\" onclick=\"next_song();\" />
            <h2>queue:</h2>
            <ol id=\"queue\"></ol>
        </div>
    </body>
</html>";

pub const STYLE: &str = ".current_song {
    font-weight: bold;
}

#all_songs {
    position: fixed;
    top: 0%;
    left: 0%;
    width: 15%;
    height: 100%;
    overflow: scroll;
    border-right: solid 1px black;
}

#controls {
    position: fixed;
    top: 0%;
    left: 15%;
    width: 85%;
    height: 100%;
    overflow: scroll;
    margin-left: 8px;
}

#timer_slider {
    width: 100%;
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
            list_item.appendChild(document.createTextNode(queue_array[i]));
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

function get_all_songs() {
    fetch(\"/allsongs\").then((response) => response.text()).then((songs) => {
        let song_list = JSON.parse(songs);
        let song_div = document.getElementById(\"all_songs\");
        song_div.innerHTML = \"\";
        for (let i = 0; i < song_list.length; i++) {
            let song = song_list[i];
            let song_p = document.createElement(\"p\");
            song_p.appendChild(document.createTextNode(song[\"artist\"] + \" -- \" + song[\"title\"]));
            song_div.appendChild(song_p);
        }
    })
}

window.onload = function() {
    get_all_songs();
    update_info();
    window.setInterval(update_info, 1000);
};";

