# Injection dll example

Use [syringe](https://github.com/OpenByteDev/dll-syringe) behind the scene.

## How to Build

Simply run `cargo build --release` to build all the binaries.

You will have the following output in `target/release` :

- `payload.dll` : the dll with the new symbols you wish to insert into your target process.
- `runner.exe` : the runner to inject the `payload.dll` in your target process.
- `test_process.exe` : a simple process that wait 120s before exiting to provide you an example target **unprivileged**
  process.

## How to Use

Run `.\target\release\test_process.exe` in a terminal.

Run `.\target\release\runner.exe` in another terminal. This will recompile payload.dll currently (you can switch to a
full path or embed your dll in the runner if you want) so you need to run your terminal from inside this code
repository (it runs `cargo build . --release --lib`).

Observe output of runner.exe.

