use probe_rs::{config::TargetSelector, Probe, Target, WireProtocol};
use std::convert::From;

fn main() {
    println!("Attempt to connect to SAME54P20A");

    // Get a list of all available debug probes.
    let probes = Probe::list_all();

    println!("Probes {:?}", probes);

    // Use the first probe found.
    let mut probe = probes[0].open().unwrap();

    probe
        .select_protocol(WireProtocol::Swd)
        .expect("SWD failed");

    let target_selector: TargetSelector = "ATSAME54P20A".into();
    println!("target {:?}", target_selector);
    // let target: Target = target_selector.into();

    // Attach to a chip.
    let mut session = probe.attach(target_selector).unwrap(); // .expect("failure");

    // Select a core.
    let mut core = session.core(0).unwrap();

    // Halt the attached core.
    // core.halt().unwrap();
}
