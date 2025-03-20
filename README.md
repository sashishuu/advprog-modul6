# advprog-modul6
# Commit 1: Reflection Notes

## Handle Connection & Check Response

The function `handle_connection` in `src/main.rs` reads incoming HTTP requests using a `BufReader`. It processes the request by:
- Reading the stream line by line using `.lines()`
- Mapping the result with `.map(|result| result.unwrap())`
- Stopping when it encounters an empty line with `.take_while(|line| !line.is_empty())`
- Storing the request as a `Vec<String>`, which contains each line of the HTTP request

