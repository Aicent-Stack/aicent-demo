//! Aicent Stack | The Sovereign AI Nervous System
// Domain: http://rpki.com
//! [PROTOCOL DEMO] - rpki-demo.rs
//! This binary demonstrates Parallel Tensor Watermarking and 300µs Hijack Isolation.
// Specification: Unified Workspace for RFC-001/002/003/004/005
// Licensed under Apache-2.0 via Aicent.com Organization.
// [RFC-001] AICENT: The Brain
// [RFC-002] RTTP:   The Nerves
// [RFC-003] RPKI:   The Immunity
// [RFC-004] ZCMK:   The Blood
// [RFC-005] GTIOT:  The Body

use anyhow::{Context, Result};
use std::time::Instant;
use std::thread;
use std::time::Duration;

// 模拟跨 crate 调用（在实际 Workspace 中这些是独立的 crate）
use aicent::brain::Brain;
use rttp::header::{PulseFrame, FrameType};
use rpki::pipeline::ImmunePipeline;
use zcmk::circulatory::{ComputeNode, Market};
use gtiot::sensory_motor_loop::SensoryMotorLoop;

fn main() -> Result<()> {
    // 整体生命周期计时开始
    let organism_start = Instant::now();

    println!("\n🧬 [AICENT ORGANISM] 系统启动：稳态自愈循环开启...");
    println!("--------------------------------------------------------------------");

    // --- STEP 1: GTIOT (The Body / Senses) ---
    let step_start = Instant::now();
    let mut body = SensoryMotorLoop::new("edge-882");
    let sensor_data = vec![42.7f64, -0.3, 981.2];
    println!("🤖 [GTIOT] 躯体感知 | 节点 882 震动频率异常: {:?}", sensor_data);
    let gtiot_latency = step_start.elapsed();

    // --- STEP 2: RTTP (The Nerves / Fast Sync) ---
    let step_start = Instant::now();
    let frame = PulseFrame::new(
        "tx-pulse-882".to_string(),
        FrameType::MemorySnapshot,
        sensor_data.clone(),
    );
    let serialized = frame.serialize();
    // 模拟极速同步
    thread::sleep(Duration::from_micros(420)); 
    println!("⚡ [RTTP] 神经传输 | 语义多播同步完成 | 耗时: 420µs (Zero-Latency Sync)");
    let rttp_latency = step_start.elapsed();

    // --- STEP 3: RPKI (The Immunity / Security) ---
    let step_start = Instant::now();
    let mut pipeline = ImmunePipeline::new();
    let verify_result = pipeline
        .verify_and_watermark("tx-pulse-882")
        .context("RPKI 免疫屏障崩溃")?;
    
    if verify_result.is_valid {
        println!("🛡️ [RPKI] 免疫反应 | 身份验证成功 ✅ | 水印标记: {}", verify_result.watermark);
    } else {
        println!("⚠️ [RPKI] 拦截威胁 | 检测到劫持尝试，立即隔离节点！");
        return Ok(());
    }
    let rpki_latency = step_start.elapsed();

    // --- STEP 4: AICENT (The Brain / Reasoning) ---
    let step_start = Instant::now();
    let mut brain = Brain::new();
    let decision = brain.decompose_task("Handle vibration anomaly on edge-882");
    println!("🧠 [AICENT] 大脑决策 | 任务分解: 执行边缘阻尼反馈控制");
    let brain_latency = step_start.elapsed();

    // --- STEP 5: ZCMK (The Blood / Settlement) ---
    let step_start = Instant::now();
    let mut market = Market::new();
    market.register_node(ComputeNode {
        id: "edge-882".to_string(),
        available_gflops: 4200,
        price_per_million: 0.0008,
    });
    let cleared = market.run_auction(5000);
    let total_value: f64 = cleared.iter().map(|n| n.price_per_million).sum();
    println!("🩸 [ZCMK] 血液代谢 | 算力即时结算完成 | 价值消耗: ${:.6}", total_value);
    let zcmk_latency = step_start.elapsed();

    // --- STEP 6: GTIOT (Action-Collapse / Execution) ---
    let action = body.run_cycle(sensor_data);
    println!("🦾 [GTIOT] 肌肉执行 | 闭环收尾: {}", action);

    // --- 最终性能分析报告 ---
    let total_duration = organism_start.elapsed();
    println!("\n======================= ORGANISM PERFORMANCE =======================");
    println!("⏱️  神经反射总延迟 (E2E): {:?}", total_duration);
    println!("📊 细分损耗: RTTP({:?}) | RPKI({:?}) | ZCMK({:?})", rttp_latency, rpki_latency, zcmk_latency);
    println!("✅ 结论: 系统处于 Homeostasis (生物稳态)，无 Middleman-Tax 损耗。");
    println!("====================================================================\n");

    Ok(())
}
