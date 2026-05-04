# RBAC Guessing Game

A terminal-based guessing game written in Rust. This project demonstrates strict compile-time state management and access control using the Typestate and Role-Based Access Control (RBAC) design patterns.

## Architecture

The system utilizes three core components:
1. **Typestate Pattern (`typestate.rs`)**: Enforces game states (`Waiting`, `InProgress`, `Finished`) at compile time. A game cannot accept guesses until the secret is securely locked in.
2. **Capabilities (`capabilities.rs`)**: Implements RBAC. The `Dealer` struct is exclusively authorized to set the secret number, while the `Player` struct is exclusively authorized to submit guesses.
3. **Execution (`main.rs`)**: Orchestrates the terminal interface and state transitions.

## Prerequisites

To run this project, you need either:
- Docker
- Rust (Cargo)

## How to Run

### Method 1: Using Docker (Recommended)
This method runs the game in an isolated container without requiring a local Rust toolchain.

1. Build the image:
   ```bash
   docker build -t rbac-game .
2. Run the interactive container:
    ```bash
    docker run -it --rm rbac-game

### Method 2: Using Cargo
If you have Rust installed locally, you can compile and run directly.
1. Build for release:
    ```bash
    cargo build --release
2. Execute the binary:
    ```bash
    ./target/release/guessing-game

### Gameplay Instructions
1. The game starts in the setup phase. The Dealer enters a secret number.
2. The terminal screen clears automatically to hide the input.
3. The Player enters guesses. The system provides "Higher" or "Lower" hints until the correct number is found.

### Live Tunneling Mode (for Showcase purpose use of Tmate)
This project includes **tmate** to allow for remote auditing or multiplayer over a secure tunnel. To launch with a web-broadcast link:

1. Run the container with host networking:
    ```bash
    docker run -it --rm --network host vault-game \
    bash -c "tmate -S /tmp/t.sock new-session -d && sleep 2 && tmate -S /tmp/t.sock display -p '#{tmate_web_ro}' && ./target/release/guessing-game"

2. Copy the generated **tmate.io** link to view the terminal in a browser.