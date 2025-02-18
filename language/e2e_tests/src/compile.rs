// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

//! Support for compiling scripts and modules in tests.

use compiler::Compiler;
use types::{
    account_address::AccountAddress,
    transaction::{Program, TransactionArgument},
};
use bytecode_verifier::VerifiedModule;
use vm::CompiledModule;

/// Compile the provided Move code into a blob which can be used as the code for a [`Program`] or
/// a [`Script`].
///
/// The script is compiled with the default account address (`0x0`).
pub fn compile_script(code: &str) -> Vec<u8> {
    let compiler = Compiler {
        code,
        ..Compiler::default()
    };
    compiler.into_script_blob().unwrap()
}

/// Compile the provided Move code into a blob which can be used as a [`Script`].
pub fn compile_script_with_address(address: &AccountAddress, code: &str) -> Vec<u8> {
    let compiler = Compiler {
        address: *address,
        code,
        ..Compiler::default()
    };
    compiler.into_script_blob().unwrap()
}

/// Compile the provided Move code and arguments into a `Program` using `address` as the
/// self address for any modules in `code`.
pub fn compile_program_with_address(
    address: &AccountAddress,
    code: &str,
    args: Vec<TransactionArgument>,
) -> Program {
    let compiler = Compiler {
        address: *address,
        code,
        ..Compiler::default()
    };
    compiler.into_program(args).unwrap()
}

pub fn compile_program_with_address_return_deps (
    address: &AccountAddress,
    code: &str,
    args: Vec<TransactionArgument>,
) -> (Program,Vec<VerifiedModule>,Vec<CompiledModule>) {
    let compiler = Compiler {
        address: *address,
        code,
        ..Compiler::default()
    };
    compiler.into_program_and_deps(args).unwrap()
}

pub fn compile_program_with_address_with_deps(
    address: &AccountAddress,
    code: &str,
    args: Vec<TransactionArgument>,
    mut deps:Vec<CompiledModule>
) -> Program {
    let depsv = VerifiedModule::constract(deps[0].clone());
    let mut compiler = Compiler {
        address: *address,
        skip_stdlib_deps:false,
        code,
        extra_deps:vec![depsv],
        ..Compiler::default()
    };
    //compiler.add_deps(deps);
    compiler.into_program_2(args,deps).unwrap()
}


/// Compile the provided Move code and arguments into a `Program`.
///
/// This supports both scripts and modules defined in the same Move code. The code is compiled with
/// the default account address (`0x0`).
pub fn compile_program(code: &str, args: Vec<TransactionArgument>) -> Program {
    let compiler = Compiler {
        code,
        ..Compiler::default()
    };
    compiler.into_program(args).unwrap()
}

/// Compile the provided Move code into a blob which can be used as the code to be published
/// (a Module).
///
/// The code is compiled with the default account address (`0x0`).
pub fn compile_module(code: &str) -> Vec<u8> {
    let compiler = Compiler {
        code,
        ..Compiler::default()
    };
    compiler.into_module_blob().unwrap()
}

/// Compile the provided Move code into a blob which can be used as the code to be published
/// (a Module).
pub fn compile_module_with_address(address: &AccountAddress, code: &str) -> Vec<u8> {
    let compiler = Compiler {
        address: *address,
        code,
        ..Compiler::default()
    };
    compiler.into_module_blob().unwrap()
}
