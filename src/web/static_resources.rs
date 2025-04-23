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
                <button type=\"button\" class=\"control\" onclick=\"prev_song();\"><img src=\"/prev.svg\" /></button>
                <button type=\"button\" class=\"control\" onclick=\"toggle_pause();\"><img id=\"pause_button_image\" src=\"/play.svg\" /></button>
                <button type=\"button\" class=\"control\" onclick=\"next_song();\"><img src=\"/next.svg\" /></button>
            </div>
            <div id=\"queue_div\">
                <h2>queue</h2>
                <ol id=\"queue\"></ol>
            </div>
        </div>
    </body>
</html>";

pub const STYLE: &str = "body {
    background-color: #306;
    color: #f0c;
    height: 100vh;
    display: flex;
    margin: 0;
}

.control {
    background: none;
    border: none;
    padding: 0;
    cursor: pointer;
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

pub const SCRIPT: &str = "\"use strict\";

let now_playing_name = \"\";

function song_obj_to_string(song) {
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

function update_queue() {
    fetch(\"/queue\").then((response) => response.text()).then((json_response) => {
        let response_as_object = JSON.parse(json_response);

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

function update_now_playing() {
    fetch(\"/nowplaying\").then((response) => response.text()).then((json_response) => {
        let response_as_object = JSON.parse(json_response);

        let np = song_obj_to_string(response_as_object[\"now_playing\"]);
        let now_playing_message = \"now playing \" + np;
        document.getElementById(\"now_playing\").textContent = now_playing_message;
        document.title = now_playing_message;
        if (np != now_playing_name) {
            now_playing_name = np;
            update_queue();
        }

        let elapsed = response_as_object[\"elapsed\"];
        let duration = response_as_object[\"duration\"];
        document.getElementById(\"timer_text\").textContent = to_timer(elapsed) + \"/\" + to_timer(duration);
        
        let timer_slider = document.getElementById(\"timer_slider\");
        timer_slider.setAttribute(\"max\", duration);
        timer_slider.value = elapsed;

        document.getElementById(\"pause_button_image\").src = response_as_object[\"is_playing\"] ? \"/pause.svg\" : \"/play.svg\";
    });
}

function update_right_side() {
    update_queue();
    update_now_playing();
}

function seek_time() {
    let time_to_go_to = document.getElementById(\"timer_slider\").value;
    fetch(\"/seek\", {
        method: \"POST\",
        headers: {\"Content-Type\": \"text/plain\"},
        body: time_to_go_to
    }).then((body) => {update_now_playing();});
}

function prev_song() {
    fetch(\"/prev\", {
        method: \"POST\"
    }).then((body) => {update_now_playing();});
}

function toggle_pause() {
    fetch(\"/pause\", {
        method: \"POST\"
    }).then((body) => {update_now_playing();});
}

function next_song() {
    fetch(\"/next\", {
        method: \"POST\"
    }).then((body) => {update_now_playing();});
}

function add_to_queue(file) {
    fetch(\"/addsong\", {
        method: \"POST\",
        headers: {\"Content-Type\": \"text/plain\"},
        body: file
    }).then((body) => {update_queue();});
}

function remove_from_queue(position) {
    fetch(\"/removesong\", {
        method: \"POST\",
        headers: {\"Content-Type\": \"text/plain\"},
        body: position
    }).then((body) => {update_queue();});
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
    update_right_side();
    window.setInterval(update_now_playing, 1000);
};";

pub const PREV_SVG: &str = "<?xml version=\"1.0\" standalone=\"no\"?>
<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 20010904//EN\" \"http://www.w3.org/TR/2001/REC-SVG-20010904/DTD/svg10.dtd\">
<!-- Created using Krita: https://krita.org -->
<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"32\" height=\"32\">
<path fill=\"#ff00cc\" d=\"M 16 0 L 16 32 L 0 16 L 16 0\"/>
<path fill=\"#ff00cc\" d=\"M 32 0 L 32 32 L 16 16 L 32 0\"/>
</svg>";

pub const PAUSE_SVG: &str = "<?xml version=\"1.0\" standalone=\"no\"?>
<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 20010904//EN\" \"http://www.w3.org/TR/2001/REC-SVG-20010904/DTD/svg10.dtd\">
<!-- Created using Krita: https://krita.org -->
<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"32\" height=\"32\">
<rect fill=\"#ff00cc\" x=\"0\" y=\"0\" width=\"12\" height=\"32\"/>
<rect fill=\"#ff00cc\" x=\"20\" y=\"0\" width=\"12\" height=\"32\"/>
</svg>";

pub const PLAY_SVG: &str = "<?xml version=\"1.0\" standalone=\"no\"?>
<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 20010904//EN\" \"http://www.w3.org/TR/2001/REC-SVG-20010904/DTD/svg10.dtd\">
<!-- Created using Krita: https://krita.org -->
<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"32\" height=\"32\">
<path fill=\"#ff00cc\" d=\"M 0 0 L 32 16 L 0 32 L 0 0\"/>
</svg>";

pub const NEXT_SVG: &str = "<?xml version=\"1.0\" standalone=\"no\"?>
<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 20010904//EN\" \"http://www.w3.org/TR/2001/REC-SVG-20010904/DTD/svg10.dtd\">
<!-- Created using Krita: https://krita.org -->
<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"32\" height=\"32\">
<path fill=\"#ff00cc\" d=\"M 0 0 L 16 16 L 0 32 L 0 0\"/>
<path fill=\"#ff00cc\" d=\"M 16 0 L 32 16 L 16 32 L 16 0\"/>
</svg>";

