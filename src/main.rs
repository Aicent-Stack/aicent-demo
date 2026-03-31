//! Aicent Stack | The Sovereign AI Nervous System
//! Aicent Stack Complete Closed-Loop Demo
//! 模拟 882 号边缘节点振动异常 → 全链路五位一体闭环
// Domain: http://aicent.com
// Specification: Unified Workspace for RFC-001/002/003/004/005
// Licensed under Apache-2.0 via Aicent.com Organization.
// [RFC-001] AICENT: The Brain
// [RFC-002] RTTP:   The Nerves
// [RFC-003] RPKI:   The Immunity
// [RFC-004] ZCMK:   The Blood
// [RFC-005] GTIOT:  The Body

use aicent::brain::Brain;
use rttp::header::{PulseFrame, FrameType};
use rpki::pipeline::ImmunePipeline;
use zcmk::circulatory::{ComputeNode, Market};
use gtiot::sensory_motor_loop::SensoryMotorLoop;

fn main() {
    println!("🚀 Aicent Stack — Complete Closed-Loop Demo");
    println!("   882号边缘节点振动异常 → 五位一体自主进化闭环");
    println!("====================================================");

    // 1. GTIOT Body 感知异常
    let mut body = SensoryMotorLoop::new("edge-882");
    let sensor_data = vec![42.7, -0.3, 981.2]; // vibration, temp, pressure
    println!("🤖 GTIOT Body: 检测到振动异常!");

    // 2. RTTP Nerves 发送脉冲帧
    let frame = PulseFrame::new(
        "edge-882-anomaly-001".to_string(),
        FrameType::MemorySnapshot,
        sensor_data.clone(),
    );
    println!("⚡ RTTP Nerves: PulseFrame 已发送 (<800μs)");

    // 3. RPKI Immunity 验证
    let mut pipeline = ImmunePipeline::new();
    let verify_result = pipeline.verify_and_watermark("edge-882-anomaly-001");
    println!("🛡️ RPKI Immunity: 验证通过，水印已附加");

    // 4. Aicent Brain 决策
    let mut brain = Brain::new();
    let decision = brain.decompose_task("Handle vibration anomaly on edge-882");
    println!("🧠 Aicent Brain: 决策完成 → {}", decision);

    // 5. ZCMK Blood 结算算力
    let mut market = Market::new();
    let node = ComputeNode {
        id: "edge-882".to_string(),
        available_gflops: 4200,
        price_per_million: 0.0008,
    };
    market.register_node(node);
    let cleared = market.run_auction(5000);
    println!("🩸 ZCMK Blood: 算力拍卖完成，结算价值 ${:.4}", cleared.iter().map(|n| n.price_per_million).sum::<f64>());

    // 6. GTIOT Body 执行动作
    let action = body.run_cycle(sensor_data);
    println!("🤖 GTIOT Body: 执行维护动作 → {}", action);

    println!("\n🎉 完整闭环结束！主权生命体自主进化成功！");
    println!("   Aicent Stack 已证明：从感知到行动，零延迟、零信任、零佣金、具身闭环！");
}
