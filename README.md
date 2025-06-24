# wordle-rs: Thobias' and Jardar's wordle clone in rust

Purpose: this project aims to serve as a intro to the basics of rust for 2 noobies collaborating on a wordle clone. From different backround (C/C++ vs Python/Javascript), this project will be a stepping block on the way to rust proficiency.

## What makes this project unique

Our goal is to create a "superset" of wordle, to enable customizing your game to allow playing with 3-12 words, also giving anywhere from a 3x3 to 12x12 playing board.

## Setup

Run 'cargo run' for compiling.

## Words

The words will be gotten from a list of top 20,000 words, from [this GitHub repo](https://github.com/first20hours/google-10000-english/blob/master/20k.txt).

## Graphics

This repo uses the wgpu library for rendering UI
