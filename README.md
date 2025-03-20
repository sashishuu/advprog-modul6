# advprog-modul6
# Commit 1: Reflection Notes

## Handle Connection & Check Response

The function `handle_connection` in `src/main.rs` reads incoming HTTP requests using a `BufReader`. It processes the request by:
- Reading the stream line by line using `.lines()`
- Mapping the result with `.map(|result| result.unwrap())`
- Stopping when it encounters an empty line with `.take_while(|line| !line.is_empty())`
- Storing the request as a `Vec<String>`, which contains each line of the HTTP request


# Commit 2: Reflection Notes

## Returning HTML

In this update, we modified `handle_connection` to return an HTML page (`hello.html`) as an HTTP response.  
The response is sent with a **200 OK** status, and the content is read from a file.  
The important changes in `handle_connection`:
- Reads `hello.html` using `fs::read_to_string("hello.html")`
- Formats HTTP response with **Content-Length**
- Sends the response to the client using `stream.write_all(response.as_bytes()).unwrap();`

Example screenshot of the output:

