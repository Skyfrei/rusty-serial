# Rusty-serial

Serial port communication using rust, can read and write (NOT SIMMULTANIOUSLY FOR THE TIME BEING). However, you can read and write with very little delay between each operation.
You can read and write using the serial port. Actually usable and prints out when errors are happening.
Easy to understand and use, although currently the functionalities are a bit limited.

Similar to gtkterm, but without an annoying GUI. Fully terminal based.

# Building
```sh
cargo build --release
```

# Arguments

If you decide to run the app as cargo run then please use the following command for arguments

```sh
cargo run -- {arg here as below}
```

```sh
-h gives the necessary information to run the program properly

-b sets baudrate of the communication

-d set data bits

-s set stop bits

-f set flowcontrol
```

# Functionality

The timeout is currently set to a small amount of around 1000 ms, this is so the reading of the port doesn't take forever in case there is nothing to read.
First reading the port begins, and it shows how many bytes can be read.
After reading the port, we can write a command or just proceed with reading again. You can proceed with the next read operation by simply enter-ing.

You can also type exit during write operation if u want to exit the program.

Simply send an empty (just enter) serial message to read the next messages from the serial port.
