use arrayvec::ArrayVec;
use fdt::Fdt;
use log::{debug, info};
use spin::Once;

pub static MACHINE_META: Once<MachineMeta> = Once::new();

#[derive(Clone, Debug)]
pub struct Device {
    pub base_address: usize,
    pub size: usize,
}

#[derive(Clone, Debug)]
pub struct Hart {
    pub hartid: usize,
    pub plic_context: usize,
}

#[derive(Debug, Clone, Default)]
pub struct MachineMeta {
    pub phys_mem_start: usize,
    pub phys_mem_size: usize,
    pub harts: ArrayVec<Hart, 16>,
    pub virtio: ArrayVec<Device, 16>,
}

pub fn parse(dtb: usize) {
    let fdt = unsafe { Fdt::from_ptr(dtb as *const u8) }.unwrap();
    info!("ftd: {:?}", fdt);
    let mut meta = MachineMeta::default();
    for region in fdt.memory().regions() {
        meta.phys_mem_start = region.starting_address as usize;
        meta.phys_mem_size = region.size.unwrap();
    }
    for cpu in fdt.cpus() {
        meta.harts.push(Hart {
            hartid: cpu.ids().first(),
            // TODO: get plic context
            plic_context: 0,
        });
    }
    for node in fdt.find_all_nodes("/soc/virtio_mmio") {
        if let Some(reg) = node.reg().and_then(|mut reg| reg.next()) {
            let paddr = reg.starting_address as usize;
            let size = reg.size.unwrap();
            meta.virtio.push(Device {
                base_address: paddr,
                size,
            })
        }
    }
    MACHINE_META.call_once(|| meta);
}
