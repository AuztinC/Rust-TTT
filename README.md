# Welcome to another Tic-Tac-Toe!
## Running this project

## Run (prebuilt binary)

1. Download the binary from the GitHub Release page (Place it in your /Downloads folder)

3. Make it executable and run: Open a new terminal window and paste this either as an entire block or in this order.
   ``chmod +x ~/Downloads/arc-rust-ttt``
   
   ``xattr -d com.apple.quarantine ~/Downloads/arc-rust-ttt``
   
   ``~/Downloads/arc-rust``

## Run (run in repository)
###Requirements: Rust

Check for Rust / Verify install 
 - ``rustc --version``

Rust Install command 
 - ``curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh``

 - After install, either restart your terminal, or run this command 
    - ``source ~/.cargo/env``

Once you have Rust installed(verified by the version command)
Use the following commands to run and test.

- ``cargo run``             - run the project; can be followed with `` -- -HvH`` or `` -- -AIvAI``
- ``cargo test``            - run tests a single time

### Developer Convience
An auto runner for tests must be installed, this can be done simply using cargo!
- ``cargo install cargo-watch`` - install the auto runner
- ``cargo watch -x test``       - run and watch tests
