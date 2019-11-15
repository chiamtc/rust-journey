const rust = import ('./pkg/wasm_hello_world');

rust.then((func)=>{
    func.create_stuff();
    func.run_alert("javascript")
})
