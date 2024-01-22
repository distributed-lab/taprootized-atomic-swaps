use eyre::Result;
use fnv::FnvHasher;
use num::BigInt;
use num::ToPrimitive;
use num_traits::Zero;
use std::hash::Hasher;
use std::io::Write;
use wasmer::{imports, Function, Instance, Memory, MemoryType, Module, RuntimeError, Store};

mod wasm;
use crate::wasm::{CircomWasm, Wasm};

#[derive(thiserror::Error, Debug, Clone, Copy)]
#[error("{0}")]
struct ExitCode(u32);

#[derive(Debug)]
pub struct WitnessCalculator {
    pub instance: Wasm,
    pub store: Store,
}

impl WitnessCalculator {
    pub fn new(path: impl AsRef<std::path::Path>) -> Result<Self> {
        Self::from_file(path)
    }

    pub fn from_file(path: impl AsRef<std::path::Path>) -> Result<Self> {
        let store = Store::default();
        let module = Module::from_file(&store, path)?;

        Self::from_module(module, store)
    }

    pub fn from_module(module: Module, mut store: Store) -> Result<Self> {
        let memory = Memory::new(&mut store, MemoryType::new(2000, None, false)).unwrap();
        let import_object = imports! {
            "env" => {
                "memory" => memory.clone(),
            },
            "runtime" => {
                "error" => runtime::error(&mut store),
                "logSetSignal" => runtime::log_signal(&mut store),
                "logGetSignal" => runtime::log_signal(&mut store),
                "logFinishComponent" => runtime::log_component(&mut store),
                "logStartComponent" => runtime::log_component(&mut store),
                "log" => runtime::log_component(&mut store),
                "exceptionHandler" => runtime::exception_handler(&mut store),
                "showSharedRWMemory" => runtime::show_memory(&mut store),
                "printErrorMessage" => runtime::print_error_message(&mut store),
                "writeBufferMessage" => runtime::write_buffer_message(&mut store),
            }
        };

        let instance = Wasm::new(Instance::new(&mut store, &module, &import_object)?);

        Ok(WitnessCalculator { instance, store })
    }

    pub fn calculate_witness<I: IntoIterator<Item = (String, Vec<BigInt>)>>(
        &mut self,
        inputs: I,
        sanity_check: bool,
    ) -> Result<Vec<u8>> {
        self.instance.init(sanity_check, &mut self.store)?;

        let n32 = self.instance.get_field_num_len32(&mut self.store)?;

        // allocate the inputs
        for (name, values) in inputs.into_iter() {
            let (msb, lsb) = fnv(&name);

            for (i, value) in values.into_iter().enumerate() {
                let f_arr = to_array32(&value, n32 as usize);
                for j in 0..n32 {
                    self.instance.write_shared_rw_memory(
                        j,
                        f_arr[(n32 as usize) - 1 - (j as usize)],
                        &mut self.store,
                    )?;
                }
                self.instance
                    .set_input_signal(msb, lsb, i as u32, &mut self.store)?;
            }
        }

        let mut binary_witnes = Vec::new();

        // wtns
        binary_witnes.write_all(&[b'w', b't', b'n', b's']).unwrap();

        // version - 2
        binary_witnes.write_all(&u32::to_le_bytes(2)).unwrap();

        // number of sections - 2
        binary_witnes.write_all(&u32::to_le_bytes(2)).unwrap();

        // id section - 1
        binary_witnes.write_all(&u32::to_le_bytes(1)).unwrap();

        let n8 = n32 * 4;

        // id section 1 length in 64bytes
        binary_witnes
            .write_all(&u64::to_le_bytes((n8 + 8) as u64))
            .unwrap();

        // n8
        binary_witnes.write_all(&u32::to_le_bytes(n8)).unwrap();

        // prime number
        self.instance.get_raw_prime(&mut self.store)?;

        for i in 0..n32 {
            let val = self.instance.read_shared_rw_memory(i, &mut self.store)?;
            binary_witnes.write_all(&u32::to_le_bytes(val)).unwrap();
        }

        // witness size
        let witness_size = self.instance.get_witness_size(&mut self.store)?;
        binary_witnes
            .write_all(&u32::to_le_bytes(witness_size))
            .unwrap();

        // id section - 2
        binary_witnes.write_all(&u32::to_le_bytes(2)).unwrap();

        // id section 2 length in 64bytes
        binary_witnes
            .write_all(&u64::to_le_bytes((n8 * witness_size) as u64))
            .unwrap();

        for i in 0..witness_size {
            self.instance.get_witness(i, &mut self.store)?;

            for j in 0..n32 {
                let val = self.instance.read_shared_rw_memory(j, &mut self.store)?;
                binary_witnes
                    .write_all(&u32::to_le_bytes(val as i32 as u32))
                    .unwrap();
            }
        }

        Ok(binary_witnes)
    }
}

mod runtime {
    use super::*;

    pub fn error(store: &mut Store) -> Function {
        #[allow(unused)]
        #[allow(clippy::many_single_char_names)]
        fn func(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> Result<(), RuntimeError> {
            println!("runtime error, exiting early: {a} {b} {c} {d} {e} {f}",);
            Err(RuntimeError::user(Box::new(ExitCode(1))))
        }
        Function::new_typed(store, func)
    }

    // Circom 2.0
    pub fn exception_handler(store: &mut Store) -> Function {
        #[allow(unused)]
        fn func(a: i32) {}
        Function::new_typed(store, func)
    }

    // Circom 2.0
    pub fn show_memory(store: &mut Store) -> Function {
        #[allow(unused)]
        fn func() {}
        Function::new_typed(store, func)
    }

    // Circom 2.0
    pub fn print_error_message(store: &mut Store) -> Function {
        #[allow(unused)]
        fn func() {}
        Function::new_typed(store, func)
    }

    // Circom 2.0
    pub fn write_buffer_message(store: &mut Store) -> Function {
        #[allow(unused)]
        fn func() {}
        Function::new_typed(store, func)
    }

    pub fn log_signal(store: &mut Store) -> Function {
        #[allow(unused)]
        fn func(a: i32, b: i32) {}
        Function::new_typed(store, func)
    }

    pub fn log_component(store: &mut Store) -> Function {
        #[allow(unused)]
        fn func(a: i32) {}
        Function::new_typed(store, func)
    }
}

fn fnv(inp: &str) -> (u32, u32) {
    let mut hasher = FnvHasher::default();
    hasher.write(inp.as_bytes());
    let h = hasher.finish();

    ((h >> 32) as u32, h as u32)
}

fn to_array32(s: &BigInt, size: usize) -> Vec<u32> {
    let mut res = vec![0; size];
    let mut rem = s.clone();
    let radix = BigInt::from(0x100000000u64);
    let mut c = size;
    while !rem.is_zero() {
        c -= 1;
        res[c] = (&rem % &radix).to_u32().unwrap();
        rem /= &radix;
    }

    res
}
