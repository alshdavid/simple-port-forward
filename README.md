# simple-port-forward:

Simple port forward service

## Usage

Add a config `spf.toml`

```toml
[[TCP]]
target = "192.168.1.10:8080" # Target service
bind = "0.0.0.0:8081"        # Local address to forward from

# TODO
[[UDP]]
target = "localhost:8080"
bind = "0.0.0.0:8081"

# TODO
[[HTTP]]
target = "http://192.168.1.10:3000"
bind = "0.0.0.0:3000"
request_path = "/foo/*"
destination_path = "/*"

# TODO
[[HTTP]]
domains = ["foo.localhost"]
target = "http://192.168.1.10:3000"
bind = "0.0.0.0:3000"
```

```
simple-port-forward -c ./spf.toml
```