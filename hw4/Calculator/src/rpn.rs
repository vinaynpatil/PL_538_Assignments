/*
 * Reverse Polish Notation: rpn.rs
 * See `rpn.md` for the overview.
 */

use std::io;

// Stacks will work with Items, which either either integers or booleans
#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum Item {
    Int(i32),
    Bool(bool),
}

// List of possible errors
#[derive(Debug)]
pub enum Error {
    Empty,         // Tried to pop empty stack
    Extra,         // Stack ended with extra elements
    Type,          // Type mismatch
    Syntax,        // Syntax error, didn't recognize op
    IO(io::Error), // Some kind of IO error
    Quit,          // User quitting
}

// Base operations supported by calculator, see rpn.md
#[derive(Debug)]
pub enum Op {
    Add,
    Eq,
    Neg,
    Swap,
    Rand,
    Cond,
    Quit,
}

// We'll define a result type for our calculator: either a valid value, or a calculator Error
pub type Result<T> = std::result::Result<T, Error>;

// Define a type for Stacks
#[derive(Debug)]
pub struct Stack(Vec<Item>);

// Implement the following functions on Stacks
impl Stack {
    // Make a new Stack
    pub fn new() -> Self {
        let vec: Vec<Item> = Vec::new();
        Stack(vec)
    }

    // Check if a Stack is empty
    pub fn empty(&self) -> bool {
        self.0.is_empty()
    }

    // Push an item onto a stack (should never error)
    pub fn push(&mut self, item: Item) -> Result<()> {
        self.0.push(item);
        Ok(())
    }

    // Pop an item off the Stack; may result in Empty error
    pub fn pop(&mut self) -> Result<Item> {
        let pop_element = self.0.pop();
        match pop_element {
            Some(val) => Ok(val),
            None => Err(Error::Empty),
        }
    }

    /*
     * Main evaluation function: apply an operation to a Stack
     *
     * Complete each of the cases.
     *
     * Hint: You'll probably want to use the "question-mark" syntax quite a bit; see `rpn.md`.
     */
    pub fn eval(&mut self, op: Op) -> Result<()> {
        match op {
            Op::Add => {
                let x = self.pop()?;
                match x {
                    Item::Int(val_x) => {
                        let y = self.pop()?;
                        match y {
                            Item::Int(val_y) => self.push(Item::Int(val_x + val_y)),
                            _ => Err(Error::Type),
                        }
                    }
                    _ => {
                        Err(Error::Type)},
                }
            }
            Op::Eq => {
                let x = self.pop()?;
                match x {
                    Item::Int(val_x) => {
                        let y = self.pop()?;
                        match y {
                            Item::Int(val_y) => self.push(Item::Bool(val_x == val_y)),
                            _ => Err(Error::Type),
                        }
                    }
                    Item::Bool(val_x) => {
                        let y = self.pop()?;
                        match y {
                            Item::Bool(val_y) => self.push(Item::Bool(val_x == val_y)),
                            _ => Err(Error::Type),
                        }
                    }
                }
            }
            Op::Neg => {
                let x = self.pop()?;
                match x {
                    Item::Bool(val_x) => self.push(Item::Bool(!val_x)),
                    _ => Err(Error::Type),
                }
            }
            Op::Swap => {
                let x = self.pop()?;
                let y = self.pop()?;
                self.push(y)?;
                self.push(x)
            }
            Op::Rand => {
                let x = self.pop()?;
                match x {
                    Item::Int(val_x) => {
                        let random_number = rand::random::<i32>().abs();
                        let number_in_range = random_number % val_x;
                        self.push(Item::Int(number_in_range))
                    }
                    _ => Err(Error::Type),
                }
            }
            Op::Cond => {
                let x = self.pop()?;
                let y = self.pop()?;
                let z = self.pop()?;
                match z {
                    Item::Bool(val_z) => {
                        if val_z {
                            self.push(y)
                        } else {
                            self.push(x)
                        }
                    }
                    _ => Err(Error::Type),
                }
            }
            Op::Quit => Err(Error::Quit),
        }
    }
}
