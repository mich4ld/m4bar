## m4bar
Simple status bar written in Rust. Not ready to use - early stage of development (I am planning write it from scratch again in xcb).

![image](https://user-images.githubusercontent.com/43048524/150657947-163fce61-5f61-48f6-968d-78af3450dceb.png)

#### Goals:
- configurable
- support many window managers
- basic modules (clock, pager and hardware indicators)

#### Current status:
- blocks fixed in code
- works with Kwin
- has basic modules (clock, pager)
- has some config parsing


### Usage:
```
$ m4bar <path-to-config> 
```

### Example config
Config file uses `.toml` format 
```toml
[bar]
background = "#ffffff"
color = "#2e3440"
font = "Roboto Mono 10"
height = 26
left-blocks = "clock pager uptime hello"

[clock]
background = "#ebcb8b"
color = "#2e3440"

[pager]
background = "#81a1c1"
color = "#eceff4"

[pager.active]
background = "#5e81ac"
color = "#eceff4"

[block.hello]
background = "#5e81ac"
color = "#eceff4"
text = "Hello world"

[block.uptime]
background = "#5e81ac"
color = "#eceff4"
text = "-"
command = "uptime -p"
```
