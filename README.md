# my_echo
Toy echo command by Rust.

This program is adapted from the following excellent books😍.

- [Command-Line Rust](https://www.oreilly.com/library/view/command-line-rust/9781098109424/)
- [Rustの練習帳](https://www.oreilly.co.jp//books/9784814400584/)


文字列を渡すと、そのまま表示させる。複数渡してもOK。

```shell
$ cargo run hello world
hello world
```

ダブルクオテーションで囲えば、スペースを空けられる。

```shell
$ cargo run "A Day In    The      Life"
A Day In    The      Life
```

`-e [environment variable]`で環境変数を表示。`-e`の前には`--`を付ける。

```shell
$ cargo run -- -e path
    C:\github\my_echo\target\debug\deps
    C:\github\my_echo\target\debug
    C:\Users\my_name\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib
    C:\Users\my_name\.rustup\toolchains\stable-x86_64-pc-windows-msvc\bin
    C:\Users\my_name\bin
    C:\Program Files\Git\mingw64\bin
    ...
```

存在しない環境変数が与えられると異常終了。

```shell
$ cargo run -- -e hello_world
    err! environment variable not found
    error: process didn't exit successfully: `target\debug\my_echo.exe -e hello_world` (exit code: 1)
```

引数が与えられなかったとき、ECHOのAAを表示。

```shell
$ cargo run

#######      #######      #     #      #######
#            #            #     #      #     #
######       #            #######      #     #
#            #            #     #      #     #
#######      #######      #     #      #######


🦀 << Enter characters or use -v to specify environment variables!
```
