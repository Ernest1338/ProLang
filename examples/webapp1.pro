C #define HTTPSERVER_IMPL
C #include "cmicroweb.h"

export http_server_listen 1
export http_server_init 2
export http_response_init 0
export http_response_status 2
export http_response_header 3
export http_response_body 3
export http_request_path 1
export http_respond 2
export http_quick_response 2

fn hello_world > int {
    var int response () http_response_init

    () http_response_status response 200
    () http_response_header response "Content-Type" "text/html"
    var char* text "Hello,_World!"
    () http_response_body response text strlen(text)

    ret response
}

fn handle_request int request > void {
    var char* url () http_request_path request
    var int response

    if strcmp(url, "/") == 0 {
        = response () hello_world
    }
    else {
        = response () http_quick_response 404 "404_not_found"
    }

    () http_respond request response
}

fn main {
    var int port 8080
C   struct http_server_s* server = http_server_init(port, handle_request);


    () print "Listening on port "
    () println %d port

    () http_server_listen server
}
