const rust = import ('./pkg/wasm_hello_world');

rust.then((func)=>{
    func.create_stuff();
    // func.run_alert("javascript")
    const ctx = func.instantiate_context();

    // console.log(new AudioContext())
    console.log(ctx);
})
