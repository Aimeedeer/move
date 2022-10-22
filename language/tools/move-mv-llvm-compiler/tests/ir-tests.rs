//! Tests of compilation from MVIR to LLVM IR.
//!
//! - Run `move-ir-compiler` to convert MVIR to MV bytecode.
//! - Run `move-mv-llvm-compiler` to convert MV bytecode to LLVM IR.
//! - Compare the actual IR to an existing expected IR.
//!
//! If the `APPROVE_LLVM_IR` env var is set, the actual IR is promoted to the
//! expected IR.

use std::path::{Path, PathBuf};

pub const TEST_DIR: &str = "tests/testdata";

datatest_stable::harness!(run_test, TEST_DIR, r".*\.mvir$");

fn run_test(test_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    Ok(run_test_inner(test_path)?)
}

fn run_test_inner(test_path: &Path) -> anyhow::Result<()> {
    let harness_paths = get_harness_paths();
    let test_plan = get_test_plan(test_path);

    compile_mvir_to_mvbc(&harness_paths, &test_plan)?;
    compile_mvbc_to_llvmir(&harness_paths, &test_plan)?;
    maybe_promote_actual_llvmir_to_expected(&test_plan)?;
    compare_actual_llvmir_to_expected(&test_plan)?;

    Ok(())
}

#[derive(Debug)]
struct HarnessPaths {
    move_ir_compiler: PathBuf,
    move_mv_llvm_compiler: PathBuf,
}

fn get_harness_paths() -> HarnessPaths {
    // Cargo will tell us the location of move-mv-llvm-compiler.
    let move_mv_llvm_compiler = env!("CARGO_BIN_EXE_move-mv-llvm-compiler");
    let move_mv_llvm_compiler = PathBuf::from(move_mv_llvm_compiler);
    eprintln!("{:?}", move_mv_llvm_compiler);

    // We have to guess where move-ir-compiler is
    let move_ir_compiler = if !cfg!(windows) {
        move_mv_llvm_compiler.with_file_name("move-ir-compiler")
    } else {
        move_mv_llvm_compiler.with_file_name("move-ir-compiler.exe")
    };

    HarnessPaths {
        move_ir_compiler,
        move_mv_llvm_compiler,
    }
}

#[derive(Debug)]
struct TestPlan {
    /// The mvir file to be compiled to LLVM IR
    mvir_file: PathBuf,
    /// The move bytecode file, compiled from mvir, compiled to LLVM
    mvbc_file: PathBuf,
    /// The final LLVM IR file
    llir_file: PathBuf,
    /// The existing LLVM IR file to compare to
    llir_file_expected: PathBuf,
}

fn get_test_plan(test_path: &Path) -> TestPlan {
    let mvir_file = test_path.to_owned();
    let mvbc_file = mvir_file.with_extension("actual.mv");
    let llir_file = mvir_file.with_extension("actual.ll");
    let llir_file_expected = mvir_file.with_extension("expected.ll");

    TestPlan {
        mvir_file,
        mvbc_file,
        llir_file,
        llir_file_expected,
    }
}

/// Run `move-ir-compiler` to produce Move bytecode, `mvbc_file`.
fn compile_mvir_to_mvbc(harness_paths: &HarnessPaths, test_plan: &TestPlan) -> anyhow::Result<()> {
    todo!()
}

/// Run `move-mv-llvm-compiler` to produce LLVM IR, `llir_file`.
fn compile_mvbc_to_llvmir(harness_paths: &HarnessPaths, test_plan: &TestPlan) -> anyhow::Result<()> {
    todo!()
}

/// Copy actual LLVM IR, `llir_file`, to expected IR, `llir_file_expected`, if `PROMOTE_LLVM_IR` env var is set.
fn maybe_promote_actual_llvmir_to_expected(test_plan: &TestPlan) -> anyhow::Result<()> {
    todo!()
}

/// Compare `llir_file` to `llir_file_expected`.
///
/// If different, print a diff.
fn compare_actual_llvmir_to_expected(test_plan: &TestPlan) -> anyhow::Result<()> {
    // todo print a diff with some existing crate - see what dada uses
    todo!()
}
