mod execution;

fn main() {
    println!("🚀 Booting Nexus HFT Bare-Metal Pipeline...");
    println!("⚠️ [SECURITY NOTICE]: Proprietary AWS DPDK network drivers and live exchange API keys have been scrubbed from this public build.");

    // Step 1: Fetch available physical CPU cores on the bare-metal server
    let core_ids = core_affinity::get_core_ids().expect("Failed to fetch CPU cores");
    
    // Step 2: Isolate the critical path to a specific core (e.g., Core 1)
    let execution_core = core_ids[1];

    // Step 3: Spawn the high-frequency trading thread
    let handle = std::thread::spawn(move || {
        
        // PIN THE THREAD: Lock this process to the isolated CPU core.
        // This prevents the OS hypervisor from context-switching, eliminating micro-jitter.
        let success = core_affinity::set_for_current(execution_core);
        if success {
            println!("✅ Critical execution thread pinned securely to CPU Core: {:?}", execution_core.id);
        } else {
            eprintln!("❌ Failed to pin thread. Ensure you are running on an isolated Bare-Metal environment.");
        }

        // Boot the deterministic routing loop
        execution::start_routing_engine();
    });

    handle.join().unwrap();
}
