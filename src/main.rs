use anyhow::Result;
use wasmtime::*;

fn main() -> Result<()> {
    let engine = Engine::default();
    let wat = r#"
        (module
            (import "host" "hello" (func $host_hello (param i32)))

            (func (export "hello")
                i32.const 3
                call $host_hello)
        )
    "#;
    let module = Module::new(&engine, wat)?;

    // Create a `Linker` and define our host function in it:
    let mut linker = Linker::new(&engine);
    linker.func_wrap("host", "hello", |caller: Caller<'_, u32>, param: i32| {
        println!("Got {} from WebAssembly", param);
        println!("my host state is: {}", caller.data());
    })?;

    // Use the `linker` to instantiate the module, which will automatically
    // resolve the imports of the module using name-based resolution.
    let mut store = Store::new(&engine, 0);
    let instance = linker.instantiate(&mut store, &module)?;
    let hello = instance.get_typed_func::<(), ()>(&mut store, "hello")?;
    hello.call(&mut store, ())?;

    Ok(())
}