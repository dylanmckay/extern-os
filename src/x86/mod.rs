pub use self::mode::Mode;

pub mod a20;
pub mod protected;
pub mod interrupt;
pub mod mode;

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

