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

![Screenshot (1561)](https://github.com/user-attachments/assets/dae53b99-7d03-4322-9378-d5e6322eaffb)

![Screenshot (1556)](https://github.com/user-attachments/assets/0aabc790-a4d4-4b37-a2af-986f9235a0e1)

# Commit 3: Reflection Notes

## Validating Request and Selectively Responding

In this milestone, we implemented request validation and selective responses:
- If the request is `"GET / HTTP/1.1"`, it serves `hello.html`.
- If the request is anything else, it serves `404.html` (or a default error message).
- The function `serve_file` reads the requested file and returns either **200 OK** or **404 NOT FOUND**.

Example screenshot:
![Screenshot (1564)](https://github.com/user-attachments/assets/ccc93bfa-f74f-43c8-95a2-3edc3235431c)
![Screenshot (1562)](https://github.com/user-attachments/assets/521b366a-37a6-424d-ba20-7af3236723c0)


# Commit 4: Reflection Notes

## Simulation of Slow Request

In this milestone, we simulated a slow server response by introducing a delay:
- If a request is `"GET / HTTP/1.1"`, it serves `hello.html` instantly.
- If a request is `"GET /sleep HTTP/1.1"`, it **waits 10 seconds** before responding.
- If a request is invalid, it serves `404.html`.

**Opinions:**
- When `/sleep` is requested, all other requests to the server are delayed.The server is **single-threaded**, meaning it can only handle one request at a time. This demonstrates the need for **multi-threading** in real web servers.

Example screenshot: (it's loading when we opened http://127.0.0.1:7878/sleep from http://127.0.0.1:7878/unknown)
![Screenshot (1566)](https://github.com/user-attachments/assets/57ae772f-9b07-4d1f-8d48-a137ba8efb4e)


# Commit 5: Reflection Notes

## Multithreaded Server using ThreadPool

In this milestone, we improved the server by implementing **ThreadPool**:
- The server now spawns **multiple worker threads** to handle requests.
- If one request takes time (e.g., `/sleep`), it does **not block** other requests.
- We used **Rust's `mpsc` (multi-producer, single-consumer) channel** and **`Arc<Mutex<T>>`** to share the queue of incoming jobs across threads.

**Opinions:**
Previously, one slow request (e.g., `/sleep`) would **block all other requests**. Now, multiple requests **can be processed concurrently** using **4 worker threads**.

Example screenshot:
![Screenshot (1575)](https://github.com/user-attachments/assets/a3da0b1e-cd73-4020-bc58-a658bb072c1b)


