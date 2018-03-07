const wasm = require("./main.rs")

wasm.initialize({noExitRuntime: true}).then(module=>{
    const add = module.cwrap('add','number', ['number', 'number'])
    const multiply = module.cwrap('multiply','number', ['number', 'number'])
    console.log("calling functions from javascript")
    console.log(add(3,2))
    console.log(multiply(3,2))
})

