/*
   Appellation: compiler <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Compiler {
    current_state: i32,
}

impl Compiler {
    pub fn init(&mut self) -> i32 {
        // initialize Wasmer and any other dependencies
        // ...
        // return the next state
        1
    }

    pub fn read_input(&mut self) -> i32 {
        // read the input WebAssembly code
        // ...
        // return the next state
        2
    }

    pub fn compile(&mut self) -> i32 {
        // compile the input WebAssembly code using Wasmer
        // ...
        // return the next state
        3
    }

    pub fn write_output(&mut self) -> i32 {
        // write the compiled WebAssembly code to the output
        // ...
        // return the next state
        4
    }

    pub fn finish(&mut self) -> i32 {
        // clean up and finalize the compilation process
        // ...
        // return the final state
        5
    }

    pub fn run(&mut self) {
        // update the current state of the state-machine
        // by calling the appropriate state method based
        // on the current state
        match self.current_state {
            0 => self.current_state = self.init(),
            1 => self.current_state = self.read_input(),
            2 => self.current_state = self.compile(),
            3 => self.current_state = self.write_output(),
            4 => self.current_state = self.finish(),
            _ => {}
        }
    }
}
