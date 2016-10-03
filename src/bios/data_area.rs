const ADDRESS: *const DataArea = 0x0400 as *const _;

const BIOS_DATA_AREA: *const DataArea = ADDRESS as *const DataArea;

pub fn data_area() -> DataArea {
    unsafe {
        *BIOS_DATA_AREA
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct DataArea
{
    pub serial_ports: [u16; 4],
    pub parallel_ports: [u16; 3],
    pub ebda_base_address: u16,
    pub hardware: u16,
    pub keyboard_state: u16,
    pub keyboard_buffer: [u8; 32],
    pub display_mode: u8,
    pub display_columns: u16,
    pub video_port: u16,
    pub irq_ticks: u16,
    pub keyboard_buffer_start: u16,
    pub keyboard_buffer_end: u16,
    pub keyboard_shift_state: u8,
}

