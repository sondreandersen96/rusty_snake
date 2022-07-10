# Rusty Snake 

This is a recreating of the classic snake game. The game is written in Rust using the [macroquad](https://github.com/not-fl3/macroquad) framework (crate). 

# Story 

This is my first real project using the Rust language, and has so far been a fun and great way to learn the language. 

# TODO

- Prevent turning in the exact oposite direction 
- Tune food spawining
- Make snake speed dynamic (increase with points)


# Nice to know 

Create and view documentation for this project (I have not written any as of yet) and all its dependencies with: `cargo doc --open`. 

Run with: `cargo run`.

Run live development server with: `cargo watch -c -x run`.

# Architecture 

I have had the MVC-pattern in mind when coding this game. However I have deviated somewhat, the controller is also storing data, so the controller is kind of both controller and model.

## MVC

### Model 

- Snake position 
- Food position 
- Score 
- Frame 

### View

- Render screen 

### Controller 

- Take in user input







