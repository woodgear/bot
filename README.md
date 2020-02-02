# what it is
the aim of bot is provid a way that control you windows manchine (like a ssh).
# how it work
bot has client mode and server mode.  
under server mode,it receive command and envloe it 
under client mode,it connect to bot server and you could send command via it.
the whole work behavior is just like the behavior between ssh server and ssh client
# how to use
## server mode
```
bot server PORT
```
## client mode
```
bot client URL
```
url is the ws://IP:PORT,ip is ip of the machine which bot run in server mode,
### cmd syntax
there is only two cmd here
```
call xxxx
spawn xxxx
```
when server receiver cmd like cmd A, it will call create process `cmd /c A` wait the end of process and send all output back. similarly spawn do the same, but will not wait end of process.

# how to install
```
cargo install --git https://github.com/woodgear/bot.git
```