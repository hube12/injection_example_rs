mod helpers;

use crate::helpers::build_payload_x64;
use dll_syringe::process::Process;
use dll_syringe::{process::OwnedProcess, Syringe};

fn main() {
    // find target process by name
    let target_process = OwnedProcess::find_first_by_name("test_process").unwrap();
    dbg!(target_process.base_name());
    dbg!(&target_process);
    // create a new syringe for the target process
    let syringe = Syringe::for_process(target_process);
    dbg!(&syringe);

    let dll = build_payload_x64().unwrap();
    dbg!(&dll);

    // inject the payload into the target process
    let injected_payload = syringe.inject(dll).unwrap();

    /// call the function defined in our dll
    let remote_add =
        unsafe { syringe.get_payload_procedure::<fn(f64, f64) -> f64>(injected_payload, "add") }
            .unwrap()
            .unwrap();
    let result = remote_add.call(&2.0, &4.0).unwrap();
    println!("{}", result); // prints 6

    // eject the payload from the target (optional)
    syringe.eject(injected_payload).unwrap();
}
