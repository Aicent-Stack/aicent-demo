//! Aicent Stack | The Sovereign AI Nervous System
// Domain: http://aicent.com
//! [MASTER COMMANDER] - aicent-organism.rs
//! This binary orchestrates the full biological reflex arc: 
//! GTIOT (Senses) -> RTTP (Nerves) -> RPKI (Immunity) -> Aicent (Brain) -> ZCMK (Blood) -> GTIOT (Action)
// Specification: Unified Workspace for RFC-001/002/003/004/005
// Licensed under Apache-2.0 via Aicent.com Organization.
// [RFC-001] AICENT: The Brain
// [RFC-002] RTTP:   The Nerves
// [RFC-003] RPKI:   The Immunity
// [RFC-004] ZCMK:   The Blood
// [RFC-005] GTIOT:  The Body

use std::time::{Duration, Instant};
use std::thread;

macro_rules! log_organ {
    ($color:expr, $organ:expr, $msg:expr) => {
        // 增加微秒级精确时间戳，模拟实时遥测数据
        let now = Instant::now();
        println!("\x1b[1;30m[{:?}]\x1b[0m \x1b[1;{}m[{}]\x1b[0m {}", now, $color, $organ, $msg);
    };
}

fn run_reflex_arc(is_under_attack: bool) {
    let total_start = Instant::now();

    // --- STEP 1: GTIOT (Senses) ---
    let step_start = Instant::now();
    log_organ!("33", "GTIOT", "Sensory input: Edge-882 vibration [142Hz]");
    thread::sleep(Duration::from_micros(100));
    let gtiot_sense = step_start.elapsed();

    // --- STEP 2: RTTP (Nerves) ---
    let step_start = Instant::now();
    log_organ!("36", "RTTP ", "Pulse Frame broadcasted. 420µs KV Sync initiated.");
    thread::sleep(Duration::from_micros(420));
    let rttp_time = step_start.elapsed();

    // --- STEP 3: RPKI (Immunity) ---
    let step_start = Instant::now();
    if is_under_attack {
        log_organ!("31", "RPKI ", "🚨 HIJACK DETECTED! Initiating Quarantine Pulse...");
        thread::sleep(Duration::from_micros(850));
        log_organ!("31", "RPKI ", "Node 882 ISOLATED. Rescheduling through ZCMK...");
    } else {
        log_organ!("31", "RPKI ", "Parallel Watermark verified ✅ | Identity Secure.");
        thread::sleep(Duration::from_micros(300));
    }
    let rpki_time = step_start.elapsed();

    // --- STEP 4: AICENT (Brain) ---
    let step_start = Instant::now();
    let action_msg = if is_under_attack { "Reschedule via Node-883" } else { "Active Damping" };
    log_organ!("37", "AICENT", &format!("Cognitive Decision: {}", action_msg));
    thread::sleep(Duration::from_millis(1));
    let aicent_time = step_start.elapsed();

    // --- STEP 5: ZCMK (Blood) ---
    let step_start = Instant::now();
    log_organ!("32", "ZCMK ", "Nanosecond RTBA cleared. micro-settlement: $0.00008");
    thread::sleep(Duration::from_micros(200));
    let zcmk_time = step_start.elapsed();

    // --- STEP 6: GTIOT (Action) ---
    log_organ!("33", "GTIOT", "Action-Collapse: Physical actuation complete.");

    // --- 性能分析报告 ---
    if !is_under_attack {
        println!("\x1b[1;35m[HOMEOTASIS REPORT] E2E: {:?} | RTTP: {:?} | RPKI: {:?}\x1b[0m\n", 
                 total_start.elapsed(), rttp_time, rpki_time);
    } else {
        println!("\x1b[1;31m[DEFENSE REPORT] Threat Isolated in {:?} | System Self-Healed.\x1b[0m\n", 
                 total_start.elapsed());
    }
}

fn main() {
    println!("\n\x1b[1;37m🧬 [AICENT ORGANISM] Genesis v0.2.0 - Pulsing...\x1b[0m");
    println!("--------------------------------------------------------------------");

    let mut cycle_count = 0;

    loop {
        cycle_count += 1;
        // 每 5 次循环模拟一次攻击，展示 RPKI 的防御力
        let attack = cycle_count % 5 == 0;
        
        run_reflex_arc(attack);

        // 模拟代谢间隔
        thread::sleep(Duration::from_secs(2));
        
        if cycle_count > 10 { 
            println!("Demo loop complete. System status: HOMEOTASIS.");
            break; 
        }
    }
}
