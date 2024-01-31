1) Install rust
```
sudo apt install build-essential
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2) create a new project using cargo
```
cargo new projectname
```
OR to use an existing folder
```
cargo init projectname
```


3) build the project
```
cargo build
```
then
```
cargo run to execute the program
```

4) to debug inside VSCode install the extension CodeLLDB
once the extension is installed just press F5 and it will create the launch.json file
customize the file as required and fix the program property like this:
```
"program": "${workspaceRoot}/target/debug/${workspaceRootFolderName}",
```
and the press F5 again and the debugger will run the program

