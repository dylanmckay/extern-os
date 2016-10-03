use x86::{interrupt, a20, protected};

pub struct Kernel;

impl ::Kernel for Kernel
{
    fn initialize() {
        ::table::gdt::setup::setup();
        interrupt::disable_all();
        a20::enable();
        protected::enable();

    }

    fn step() {
        let bda = ::bios::data_area();
        debug!("Data: {:?}", bda.serial_ports);
    }

    fn deinitialize() {
    }
}

