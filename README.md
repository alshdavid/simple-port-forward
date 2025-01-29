# simple-port-forward:

Simple port forward service

## Usage

Add a config `spf.toml`

```toml
[[binding]]
target = "localhost:8080"
bind = "0.0.0.0:8081"
protocol = "TCP"

[[binding]]
target = "localhost:8080"
bind = "0.0.0.0:8082"
protocol = "TCP"
```

```
simple-port-forward
```