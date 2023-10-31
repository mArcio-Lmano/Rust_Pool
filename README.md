# Rust Pool Game

## Description

Rust Pool Game is a simple 2D pool (billiards) game implemented in the Rust programming language using the ggez game framework. The game features a white cue ball and a set of numbered red balls. The objective of the game is to pot the red balls into the pockets while following the rules of pool.

## Features

- Realistic ball physics and collision handling.
- Broad-phase collision culling for improved performance.
- Two-player support with a turn-based system.
- Scoring system to keep track of each player's points.
- Game over condition when all red balls are potted.

## How to Play


1. **Player 1:** The Player 1 goes first. Click the left mouse button to shoot the ball.

2. **Player 2:** If Player 1 fails to pot a red ball into the pocket, it's your turn to play.

3. **Scoring:** Players score points by potting the red balls into the pockets. If a player successfully pots a red ball, they can take another shot. The game keeps track of each player's points.

4. **Turns:** Players take turns, and the game indicates whose turn it is.

5. **Game Over:** The game ends when all red balls are potted. The player with the most points wins.

## Installation

1. Ensure you have Rust and Cargo installed. You can download them from [https://www.rust-lang.org/](https://www.rust-lang.org/).

2. Clone this repository or download the source code.

3. Open your terminal and navigate to the project's directory.

4. Run the game using the command:

   ```shell
   cargo run
    ```

## Controls

* Left Mouse Button: To strike the cue ball.

## Known Bugs

* Occasionally, when a ball travels at high speed, it may pass through other balls due to collision handling limitations (Work in the Feature).

## Future Work

- **Implement an AI:** Enhance the game by adding an artificial intelligence (AI) opponent, allowing players to challenge computer-controlled opponents.

- **Menu System:** Create a user-friendly menu system that provides options for playing against another player or challenging the AI. This menu can enhance the overall user experience and make the game more accessible.

