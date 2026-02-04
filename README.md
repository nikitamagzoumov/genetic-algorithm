# Autonomous Agent Navigation via Genetic Algorithm (Rust)

[![Language](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Library](https://img.shields.io/badge/Library-Macroquad-blue.svg)](https://macroquad.rs/)

Inspired by [CodeBullet's](https://youtube.com/@codebullet?si=-xedZHoe2U4fbZCw) work. 

An implementation of a Genetic Algorithm (GA) designed to evolve autonomous agents capable of navigating a track. This project explores high-performance simulation and neuroevolution concepts using **Rust**.

![Example](assets/Algorithm_example.gif)

## Project Goals
The objective is to evolve a population of 500 agents to reach a finish line by navigating around obstacles. Instead of hard-coded paths, the agents "learn" through iterative selection and mutation.

## The Genetic Engine
This implementation follows a classic GA lifecycle:

1. Population Initialization: 500 agents are spawned with a random "DNA" consisting of 1,000 movement vectors.
2. Fitness Evaluation: Survival is not enough. Fitness is calculated as:
   - `Fitness = (Checkpoints * Multiplier) + Distance_to_Next_Checkpoint`
3. Selection: Using Elitism by selecting the top 10% of the population to seed the next generation.
4. Mutation: To ensure genetic diversity, non-elite agents have a 1% probability of gene mutation.

## Why Rust?
I chose Rust for this simulation to ensure:
* Concurrency Ready: The architecture is designed to eventually scale to parallelized fitness calculations using `Rayon`.
* Memory Safety: Eliminating memory leaks in long-running evolutionary simulations.

## Installation & Usage
Ensure you have [Rust](https://rustup.rs/) installed.

```bash
# Clone the repository
git clone https://github.com/nikitamagzoumov/genetic-algorithm.git

# Run the program
cargo run 