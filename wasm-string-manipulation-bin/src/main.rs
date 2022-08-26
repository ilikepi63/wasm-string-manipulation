use anyhow::{anyhow, Result};
use wasmtime::{Engine, Instance, Module, Store};

fn main() -> Result<()> {
    // generate a valid wasmtime instance
    let engine = Engine::default();

    let module = Module::from_file(&engine, "<INSERT WASM FILE HANDLE HERE>.wasm")?;

    let mut store = Store::new(&engine, 4);

    let instance = Instance::new(&mut store, &module, &[])?;

    let memory = instance
        .get_memory(&mut store, "memory")
        .ok_or(anyhow!("Error with getting memory"))?;

    // get all of the needed internal functions from the wasm module
    let wasm_malloc_fn = instance.get_typed_func::<u32, u32, _>(&mut store, "_malloc")?;
    let say_hello_fn = instance.get_typed_func::<(u32, u32), u32, _>(&mut store, "say_hello")?;
    let get_last_len_fn = instance.get_typed_func::<(), u32, _>(&mut store, "_get_last_length")?;

    // create a byte array representation of a string.
    let name = b"Cameron";

    // we allocate a memory block inside of the module's memory, returning the mem address
    let ptr = wasm_malloc_fn.call(&mut store, name.len().try_into()?)?;

    // write the byte array to that pointer in memory
    memory.write(&mut store, ptr.try_into().unwrap(), name)?;

    // call the "say_hello" function to create the result, which is allocated back onto the heap(or wasm memory)
    let result = say_hello_fn.call(&mut store, (ptr, name.len().try_into()?))?;

    // NOTE: at this point, we could call a "free" function to clear the initial byte array memory address. 

    // get the length of the upper memory address 
    let len_of_string = get_last_len_fn.call(&mut store, ())?;

    // create a buffer to hold the bytes
    let mut buf = vec![0; len_of_string.try_into()?];

    // read the bytes from the wasm memory into our buffer
    memory
        .read(store, result.try_into().unwrap(), &mut buf)
        .expect("Something is not lekker here");

    // print the result!
    println!("{}", String::from_utf8(buf)?);

    Ok(())
}
