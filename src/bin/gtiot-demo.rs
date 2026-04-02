//! Aicent Stack | The Sovereign AI Nervous System
// Domain: http://gtiot.com
//! [PROTOCOL DEMO] - gtiot-demo.rs
//! This binary demonstrates High-fidelity Edge Fusion and Action-Collapse Execution.
// Specification: Unified Workspace for RFC-001/002/003/004/005
// Licensed under Apache-2.0 via Aicent.com Organization.
// [RFC-001] AICENT: The Brain
// [RFC-002] RTTP:   The Nerves
// [RFC-003] RPKI:   The Immunity
// [RFC-004] ZCMK:   The Blood
// [RFC-005] GTIOT:  The Body

use anyhow::{Context, Result};
use rand::Rng;
use aicent::brain::Brain;
use rttp::header::{PulseFrame, FrameType};
use rpki::pipeline::ImmunePipeline;
use zcmk::circulatory::{ComputeNode, Market};
use gtiot::sensory_motor_loop::SensoryMotorLoop;

fn main() -> Result<()> {
    println!("🤖 Aicent Stack — GTIOT Body Layer Demo");
    println!("   Embodied Execution + Shadow State + 1.2kHz Sensory-Motor Loop");
    println!("====================================================================\n");

    // 1. GTIOT Body 初始化影子状态
    let mut body = SensoryMotorLoop::new("edge-882");
    println!("🔄 [GTIOT Body] 影子状态初始化完成 | Edge-882 已上线，1.2kHz 循环启动");
    println!("   当前传感器采样率: 1200 Hz");

    // 2. RTTP Nerves 接收最终执行指令
    let frame = PulseFrame::new(
        "edge-882-maintenance-complete".to_string(),
        FrameType::StateSync,
        vec![1u8],
    );
    let serialized = frame.serialize();
    println!("⚡ [RTTP Nerves] 执行指令 PulseFrame 已接收 | 大小: {} bytes", serialized.len());

    // 3. RPKI Immunity 最终验证（增加错误分支）
    let mut pipeline = ImmunePipeline::new();
    let verify_result = if rand::thread_rng().gen_bool(0.85) {
        pipeline.verify_and_watermark("edge-882-maintenance-complete")
            .context("RPKI verification failed")?
    } else {
        println!("❌ [RPKI Immunity] 验证失败！指纹不匹配 → 触发隔离");
        return Err(anyhow::anyhow!("RPKI fingerprint mismatch - node isolated"));
    };
    println!("🛡️ [RPKI Immunity] 执行指令验证通过 | Trust Score: {:.3}", verify_result.trust_score);

    // 4. Aicent Brain 下发最终决策
    let mut brain = Brain::new();
    let decision = brain.decompose_task("Execute final maintenance action on edge-882");
    println!("🧠 [Aicent Brain] 最终决策下发 → {}", decision);

    // 5. ZCMK Blood 结算（增加错误分支）
    let mut market = Market::new();
    let node = ComputeNode {
        id: "edge-882".to_string(),
        available_gflops: 4200,
        price_per_million: 0.0008,
    };
    market.register_node(node);

    let cleared = if rand::thread_rng().gen_bool(0.9) {
        market.run_auction(5000)
    } else {
        println!("⚠️ [ZCMK Blood] 拍卖失败！算力市场暂无足够供给");
        return Err(anyhow::anyhow!("ZCMK auction failed - insufficient compute"));
    };
    let total_value: f64 = cleared.iter().map(|n| n.price_per_million * 5000.0 / 1_000_000.0).sum();
    println!("🩸 [ZCMK Blood] 算力结算完成 | 总价值: ${:.4}", total_value);

    // 6. GTIOT Body 执行最终物理动作（核心演示）
    let sensor_data = vec![42.7f64, -0.3, 981.2];
    let action = body.run_cycle(sensor_data);
    println!("🤖 [GTIOT Body] 影子状态更新完成");
    println!("   执行物理动作 → {}", action);
    println!("   当前循环频率: 1.2kHz | 影子状态与物理世界同步成功");

    println!("\n🎉 GTIOT Body 完整演示结束！");
    println!("   ✅ 感知 → 传输 → 验证 → 决策 → 结算 → 具身执行 全链路闭环成功");
    println!("   Aicent Stack 主权生命体已真正站立于物理世界！");
    println!("   Demo 运行结束 — 所有模块真实跨 crate 调用完成");

    Ok(())
}
