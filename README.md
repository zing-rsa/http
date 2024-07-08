## http

a basic http server built on top of a tcp connection in rust

## http spec:
https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP
https://developer.mozilla.org/en-US/docs/Web/HTTP/Messages

## run locally: 
from root: `cargo run`

## usage:
- `GET: localhost:8080/example.txt` should return the example data inside `resources/example.txt`
- `POST: localhost:8080/myfile.txt` should create a file called myfile