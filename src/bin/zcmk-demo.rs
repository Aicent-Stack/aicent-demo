//! Aicent Stack | The Sovereign AI Nervous System
// Domain: http://aicent.com
//! Aicent Stack — ZCMK Blood Demo
//! 零佣金 DePIN 算力市场 + 价值流转完整演示
//! 真实跨 crate 调用闭环（RTTP 触发需求 → RPKI 验证 → ZCMK 拍卖结算）
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
    println!("🩸 Aicent Stack — ZCMK Blood Layer Demo");
    println!("   Zero-Commission DePIN Compute Market + Value Circulation");
    println!("====================================================================\n");

    // 1. GTIOT Body 触发算力需求（模拟边缘节点需要维护算力）
    let mut body = SensoryMotorLoop::new("edge-882");
    println!("🤖 [GTIOT Body] Edge-882 需要紧急维护算力（需求 5000 GFLOPS）");

    // 2. RTTP Nerves 发送需求脉冲
    let frame = PulseFrame::new(
        "edge-882-compute-demand".to_string(),
        FrameType::TaskPrimitive,
        vec![5000u8], // 需求编码
    );
    let serialized = frame.serialize();
    println!("⚡ [RTTP Nerves] 算力需求 PulseFrame 已发送 | 大小: {} bytes", serialized.len());

    // 3. RPKI Immunity 验证需求合法性
    let mut pipeline = ImmunePipeline::new();
    let verify_result = pipeline
        .verify_and_watermark("edge-882-compute-demand")
        .context("RPKI verification failed")?;
    println!("🛡️ [RPKI Immunity] 需求验证{} | Trust Score: {:.3}", 
             if verify_result.is_valid { "通过 ✅" } else { "失败 ❌" }, 
             verify_result.trust_score);

    // 4. Aicent Brain 决策分配算力任务
    let mut brain = Brain::new();
    let decision = brain.decompose_task("Allocate 5000 GFLOPS for edge-882 maintenance");
    println!("🧠 [Aicent Brain] 决策完成 → {}", decision);

    // 5. ZCMK Blood 零佣金算力市场拍卖 + 结算（核心演示）
    println!("\n🩸 [ZCMK Blood] 零佣金 DePIN 算力市场启动拍卖...");
    
    let mut market = Market::new();
    
    // 注册多个闲置节点
    market.register_node(ComputeNode {
        id: "node-tokyo-01".to_string(),
        available_gflops: 12000,
        price_per_million: 0.0007,
    });
    market.register_node(ComputeNode {
        id: "node-singapore-07".to_string(),
        available_gflops: 8500,
        price_per_million: 0.0009,
    });
    market.register_node(ComputeNode {
        id: "node-berlin-03".to_string(),
        available_gflops: 6500,
        price_per_million: 0.0006,
    });

    // 执行拍卖
    let cleared = market.run_auction(5000);
    let total_value: f64 = cleared.iter().map(|n| n.price_per_million * 5000.0 / 1_000_000.0).sum();

    println!("🏪 拍卖完成！匹配 {} 个节点", cleared.len());
    for node in &cleared {
        println!("   • {} | 提供 {} GFLOPS @ ${:.6}/M", node.id, node.available_gflops, node.price_per_million);
    }
    println!("💰 零佣金结算完成 | 总价值: ${:.4}", total_value);

    // 6. GTIOT Body 执行最终动作
    let action = body.run_cycle(vec![5000.0]);
    println!("🤖 [GTIOT Body] 算力已分配，维护动作执行 → {}", action);

    println!("\n🎉 ZCMK Blood 完整演示结束！");
    println!("   ✅ 零佣金 · 实时竞价 · 原子结算 · 全链路跨 crate 调用成功");
    println!("   Aicent Stack 血液循环系统已证明：算力即血液，价值即生命！");

    Ok(())
}
