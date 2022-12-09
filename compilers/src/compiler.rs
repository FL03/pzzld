/*
   Appellation: compiler <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use crate::states::CompilerState;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Compiler {
    state: CompilerState,
}

impl Compiler {
    pub fn set_state(&mut self, state: CompilerState) -> &Self {
        self.state = state;
        self
    }
    pub fn init(&mut self) -> &Self {
        self.set_state(CompilerState::Init);
        self
    }

    pub fn read_input(&mut self) -> &Self {
        self.set_state(CompilerState::read());
        // read the input WebAssembly code
        // ...
        // return the next state
        self
    }

    pub fn compile(&mut self) -> &Self {
        self.set_state(CompilerState::compile());
        // compile the input WebAssembly code using Wasmer
        // ...
        // return the next state
        self
    }

    pub fn write_output(&mut self) -> &Self {
        self.set_state(CompilerState::write());
        // write the compiled WebAssembly code to the output
        // ...
        // return the next state
        self
    }

    pub fn finish(&mut self) -> &Self {
        self.set_state(CompilerState::complete());
        // clean up and finalize the compilation process
        // ...
        // return the final state
        self
    }

    pub fn run(&mut self) {
        // update the current state of the state-machine
        // by calling the appropriate state method based
        // on the current state
        match self.state.into() {
            0 => self.state = self.init().state,
            1 => self.state = self.read_input().state,
            2 => self.state = self.compile().state,
            3 => self.state = self.write_output().state,
            4 => self.state = self.finish().state,
            _ => {}
        }
    }
}
