// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkVM library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod argument;
pub use argument::Argument;

mod find;
mod to_bits;
mod to_fields;

use crate::{
    Access,
    Aleo,
    Boolean,
    Debug,
    Eject,
    Identifier,
    Inject,
    Mode,
    Plaintext,
    ProgramID,
    Result,
    ToBits,
    ToFields,
    Value,
};

use snarkvm_circuit_types::Field;

/// A future.
#[derive(Clone)]
pub struct Future<A: Aleo> {
    /// The program ID.
    program_id: ProgramID<A>,
    /// The name of the function.
    function_name: Identifier<A>,
    /// The arguments.
    arguments: Vec<Argument<A>>,
}

// TODO (d0cd) Mode consistency

impl<A: Aleo> Inject for Future<A> {
    type Primitive = console::Future<A::Network>;

    /// Initializes a circuit of the given mode and future.
    fn new(_: Mode, value: Self::Primitive) -> Self {
        todo!()
    }
}

impl<A: Aleo> Eject for Future<A> {
    type Primitive = console::Future<A::Network>;

    /// Ejects the mode of the circuit future.
    fn eject_mode(&self) -> Mode {
        todo!()
    }

    /// Ejects the circuit value.
    fn eject_value(&self) -> Self::Primitive {
        todo!()
    }
}

impl<A: Aleo> Future<A> {
    /// Returns a future from the given program ID, function name, and arguments.
    #[inline]
    pub fn from(program_id: ProgramID<A>, function_name: Identifier<A>, arguments: Vec<Argument<A>>) -> Self {
        Self { program_id, function_name, arguments }
    }

    /// Returns the program ID.
    #[inline]
    pub const fn program_id(&self) -> &ProgramID<A> {
        &self.program_id
    }

    /// Returns the name of the function.
    #[inline]
    pub const fn function_name(&self) -> &Identifier<A> {
        &self.function_name
    }

    /// Returns the inputs.
    #[inline]
    pub fn inputs(&self) -> &[Argument<A>] {
        &self.arguments
    }
}
