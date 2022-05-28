// we define a function call add(f64,f64)->f64 that can be called when injected in the process
dll_syringe::payload_procedure! {
    fn add(a: f64, b: f64) -> f64 {
        a + b
    }
}
