# Sudoku

This project is Group 16's Sudoku project. It consists of a Flutter frontend (/lib), a solver (/solver), a bridge between the solver and frontend (/rust) and a server (/server).

There are binaries avalable for Windows, mac and Linux in /binaries. Note that these binaries are NOT portable and MUST be run from the folder they are in.

To run on phone see @Running. 


## Compilation
In order to compile and run it yourself, rust, flutter and 'flutter-rust-bridge' are required.


### Flutter
https://docs.flutter.dev/get-started/install

### Rust
Windows:
https://www.rust-lang.org/tools/install

Linux and MacOS
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```


### Flutter-rust-bridge
Installing the bridge requires both flutter and rust (cargo) to be install. Then run:
```
cargo install 'flutter_rust_bridge_codegen@^2.0.0-dev.0' && \
    flutter_rust_bridge_codegen create my_app && cd my_app && flutter run
```

Note that the ``` ' ``` might need to be replaced on Windows. More info at https://github.com/fzyzcjy/flutter_rust_bridge

## Running
Run the app with ```flutter run``` or ```flutter run --release``` for optimized build.

To run on a android phone enable developer mode on your phone, this step is different for every phone. Then plug a USB cable from the computer to the phone and run ```flutter run``` or ```flutter run --release```. 

To run on an IPhone, well, good luck...