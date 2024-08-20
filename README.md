# WavConv

Converts *.mp3* and *.ogg* files of any sample rate to a 41kHz *.wav* file.

## How To Use

To build wavconv, you can run

```
$ cargo build --release
```

to build a wavconv bin file in the `target/` directory.

Simply add the `wavconv` binary to your current directory, or add it to your path and run

```
$ wavconv &> /dev/null
```

```
C:\> wavconv.exe
```