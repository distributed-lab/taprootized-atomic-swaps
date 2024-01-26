use eyre::Result;
use wasmer::{Function, Instance, Store, Value};

#[derive(Clone, Debug)]
pub struct Wasm(Instance);

pub trait CircomWasm {
    fn init(&self, sanity_check: bool, store: &mut Store) -> Result<()>;
    fn func(&self, name: &str) -> &Function;
    fn set_signal(
        &self,
        c_idx: u32,
        component: u32,
        signal: u32,
        p_val: u32,
        store: &mut Store,
    ) -> Result<()>;
    fn get_u32(&self, name: &str, store: &mut Store) -> Result<u32>;
    fn get_field_num_len32(&self, store: &mut Store) -> Result<u32>;
    fn get_raw_prime(&self, store: &mut Store) -> Result<()>;
    fn read_shared_rw_memory(&self, i: u32, store: &mut Store) -> Result<u32>;
    fn write_shared_rw_memory(&self, i: u32, v: u32, store: &mut Store) -> Result<()>;
    fn set_input_signal(&self, hmsb: u32, hlsb: u32, pos: u32, store: &mut Store) -> Result<()>;
    fn get_witness(&self, i: u32, store: &mut Store) -> Result<()>;
    fn get_witness_size(&self, store: &mut Store) -> Result<u32>;
}

impl CircomWasm for Wasm {
    fn init(&self, sanity_check: bool, store: &mut Store) -> Result<()> {
        let func = self.func("init");
        func.call(store, &[Value::I32(sanity_check as i32)])?;
        Ok(())
    }

    fn func(&self, name: &str) -> &Function {
        self.0
            .exports
            .get_function(name)
            .unwrap_or_else(|err| panic!("function {} not found: {}", name, err))
    }

    fn set_signal(
        &self,
        c_idx: u32,
        component: u32,
        signal: u32,
        p_val: u32,
        store: &mut Store,
    ) -> Result<()> {
        let func = self.func("setSignal");
        func.call(
            store,
            &[c_idx.into(), component.into(), signal.into(), p_val.into()],
        )?;

        Ok(())
    }

    fn get_u32(&self, name: &str, store: &mut Store) -> Result<u32> {
        let func = self.func(name);
        let result = func.call(store, &[])?;
        Ok(result[0].unwrap_i32() as u32)
    }

    fn get_field_num_len32(&self, store: &mut Store) -> Result<u32> {
        self.get_u32("getFieldNumLen32", store)
    }

    fn get_raw_prime(&self, store: &mut Store) -> Result<()> {
        let func = self.func("getRawPrime");
        func.call(store, &[])?;
        Ok(())
    }

    fn read_shared_rw_memory(&self, i: u32, store: &mut Store) -> Result<u32> {
        let func = self.func("readSharedRWMemory");
        let result = func.call(store, &[i.into()])?;
        Ok(result[0].unwrap_i32() as u32)
    }

    fn write_shared_rw_memory(&self, i: u32, v: u32, store: &mut Store) -> Result<()> {
        let func = self.func("writeSharedRWMemory");
        func.call(store, &[i.into(), v.into()])?;
        Ok(())
    }

    fn set_input_signal(&self, hmsb: u32, hlsb: u32, pos: u32, store: &mut Store) -> Result<()> {
        let func = self.func("setInputSignal");
        func.call(store, &[hmsb.into(), hlsb.into(), pos.into()])?;
        Ok(())
    }

    fn get_witness(&self, i: u32, store: &mut Store) -> Result<()> {
        let func = self.func("getWitness");
        func.call(store, &[i.into()])?;
        Ok(())
    }

    fn get_witness_size(&self, store: &mut Store) -> Result<u32> {
        self.get_u32("getWitnessSize", store)
    }
}

impl Wasm {
    pub fn new(instance: Instance) -> Self {
        Self(instance)
    }
}
