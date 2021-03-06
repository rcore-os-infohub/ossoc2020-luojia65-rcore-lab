use crate::mem::VirtualAddress;
use core::slice;
use device_tree::{DeviceTree, Node};
use riscv_sbi::println;

const DEVICE_TREE_MAGIC: u32 = 0xd00d_feed;

/// 递归遍历设备树
fn walk(node: &Node) {
    // 检查设备的协议支持并初始化
    if let Ok(compatible) = node.prop_str("compatible") {
        match compatible {
            "virtio,mmio" => super::virtio::virtio_probe(node),
            "ns16550a" => super::ns16550a::ns16550a_probe(node),
            _ => {}
        }
    }
    println!(
        "Name: {}; Compatible: {:?}",
        node.name,
        node.prop_str("compatible")
    );

    // 遍历子树
    for child in node.children.iter() {
        walk(child);
    }
}

/// 整个设备树的 Headers（用于验证和读取）
struct DtbHeader {
    magic: u32,
    size: u32,
}

/// 遍历设备树并初始化设备
pub fn init(dtb_va: VirtualAddress) {
    let header = unsafe { &*(dtb_va.0 as *const DtbHeader) };
    // from_be 是大小端序的转换（from big endian）
    let magic = u32::from_be(header.magic);
    if magic == DEVICE_TREE_MAGIC {
        let size = u32::from_be(header.size);
        // 拷贝数据，加载并遍历
        let data = unsafe { slice::from_raw_parts(dtb_va.0 as *const u8, size as usize) };
        if let Ok(dt) = DeviceTree::load(data) {
            walk(&dt.root);
        }
    }
}
