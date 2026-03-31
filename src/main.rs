//! Aicent Stack | The Sovereign AI Nervous System
//! Aicent Stack Complete Closed-Loop Demo
//! 真实跨 crate 调用：GTIOT → RTTP → RPKI → Aicent → ZCMK → GTIOT + 结构化输出 + 错误处理
// Domain: http://aicent.com
// Specification: Unified Workspace for RFC-001/002/003/004/005
// Licensed under Apache-2.0 via Aicent.com Organization.
// [RFC-001] AICENT: The Brain
// [RFC-002] RTTP:   The Nerves
// [RFC-003] RPKI:   The Immunity
// [RFC-004] ZCMK:   The Blood
// [RFC-005] GTIOT:  The Body

use anyhow::{Context, Result};
use aicent::brain::Brain;
use rttp::header::{PulseFrame, FrameType};
use rpki::pipeline::ImmunePipeline;
use zcmk::circulatory::{ComputeNode, Market};
use gtiot::sensory_motor_loop::SensoryMotorLoop;

fn main() -> Result<()> {
    println!("🚀 Aicent Stack — Real Cross-Crate Closed-Loop Demo");
    println!("   882号边缘节点振动异常 → 五位一体主权生命体完整闭环");
    println!("====================================================================\n");

    // 1. GTIOT Body 感知异常
    let mut body = SensoryMotorLoop::new("edge-882");
    let sensor_data = vec![42.7f64, -0.3, 981.2];
    println!("🤖 [GTIOT Body] 感知到振动异常 | Sensor data: {:?}", sensor_data);

    // 2. RTTP Nerves 发送脉冲帧
    let frame = PulseFrame::new(
        "edge-882-anomaly-001".to_string(),
        FrameType::MemorySnapshot,
        sensor_data.clone(),
    );
    let serialized = frame.serialize();
    println!("⚡ [RTTP Nerves] PulseFrame 生成并序列化 | 大小: {} bytes", serialized.len());

    // 3. RPKI Immunity 验证 + 水印
    let mut pipeline = ImmunePipeline::new();
    let verify_result = pipeline
        .verify_and_watermark("edge-882-anomaly-001")
        .context("RPKI verification failed")?;
    println!("🛡️ [RPKI Immunity] 验证{} | 水印: {}", 
             if verify_result.is_valid { "通过 ✅" } else { "失败 ❌" }, 
             verify_result.watermark);

    // 4. Aicent Brain 决策
    let mut brain = Brain::new();
    let decision = brain.decompose_task("Handle vibration anomaly on edge-882");
    println!("🧠 [Aicent Brain] 任务分解完成 → {}", decision);

    // 5. ZCMK Blood 算力拍卖结算
    let mut market = Market::new();
    let node = ComputeNode {
        id: "edge-882".to_string(),
        available_gflops: 4200,
        price_per_million: 0.0008,
    };
    market.register_node(node);
    let cleared = market.run_auction(5000);
    let total_value: f64 = cleared.iter().map(|n| n.price_per_million).sum();
    println!("🩸 [ZCMK Blood] 算力拍卖完成 | 结算价值: ${:.4}", total_value);

    // 6. GTIOT Body 执行动作（闭环收尾）
    let action = body.run_cycle(sensor_data);
    println!("🤖 [GTIOT Body] 执行维护动作 → {}", action);

    println!("\n🎉 五位一体完整闭环成功执行！");
    println!("   ✅ 感知 → 传输 → 验证 → 决策 → 结算 → 执行 全链路真实跨 crate 调用完成");
    println!("   Aicent Stack 主权生命体自主进化演示完毕！");

    Ok(())
}
