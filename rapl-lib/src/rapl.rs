use csv::{Writer, WriterBuilder};
use once_cell::sync::OnceCell;
use std::{
    fs::{File, OpenOptions},
    sync::Once,
};
use thiserror::Error;

// Use the OS specific implementation
#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "windows")]
pub mod windowss;

// Import the MSR constants per CPU type
#[cfg(amd)]
use crate::rapl::amd::{MSR_RAPL_PKG_ENERGY_STAT, MSR_RAPL_POWER_UNIT};
#[cfg(intel)]
use crate::rapl::intel::{MSR_RAPL_PKG_ENERGY_STAT, MSR_RAPL_POWER_UNIT};

// Import the OS specific functions
#[cfg(target_os = "linux")]
use self::linux::{read_msr, start_rapl_impl};
#[cfg(target_os = "windows")]
use self::windowss::{read_msr, start_rapl_impl};

#[derive(Error, Debug)]
pub enum RaplError {
    #[cfg(target_os = "windows")]
    #[error("windows error")]
    Windows(#[from] windows::core::Error),
    #[error("unknown RAPL error")]
    Unknown,
}

#[cfg(amd)]
static mut RAPL_START: (u64, u64) = (0, 0);

#[cfg(intel)]
static mut RAPL_START: (u64, u64, u64, u64) = (0, 0, 0, 0);

static RAPL_INIT: Once = Once::new();
static RAPL_POWER_UNITS: OnceCell<u64> = OnceCell::new();
static mut CSV_WRITER: Option<Writer<File>> = None;

pub fn start_rapl() {
    // Run the OS specific start_rapl_impl function
    start_rapl_impl();

    RAPL_INIT.call_once(|| {
        // Read power unit and store it in the power units global variable
        let pwr_unit = read_rapl_power_unit().expect("failed to read RAPL power unit");
        RAPL_POWER_UNITS.get_or_init(|| pwr_unit);
    });

    // Safety: RAPL_START is only accessed in this function and only from a single thread
    unsafe { RAPL_START = read_rapl_registers() };
}

fn write_to_csv<T>(data: (u64, u64, u64, u64), columns: T)
where
    T: IntoIterator<Item = &'static str>,
{
    let wtr = match unsafe { CSV_WRITER.as_mut() } {
        Some(wtr) => wtr,
        None => {
            // Open the file to write to CSV. First argument is CPU type, second is RAPL power units
            let file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(format!(
                    "{}_{}.csv",
                    get_cpu_type(),
                    RAPL_POWER_UNITS.get().unwrap()
                ))
                .unwrap();

            // Create the CSV writer
            let mut wtr = WriterBuilder::new().from_writer(file);
            /*
            wtr.write_record([
                "PP0Start",
                "PP0End",
                "PP1Start",
                "PP1End",
                "PkgStart",
                "PkgEnd",
                "DramStart",
                "DramEnd",
            ])
            .unwrap();
            */
            wtr.write_record(columns).unwrap();

            // Store the CSV writer in a static variable
            unsafe { CSV_WRITER = Some(wtr) };

            // Return a mutable reference to the CSV writer
            unsafe { CSV_WRITER.as_mut().unwrap() }
        }
    };

    wtr.serialize((data.0, data.1, data.2, data.3)).unwrap();
    wtr.flush().unwrap();
}

#[cfg(intel)]
pub fn stop_rapl() {}

#[cfg(amd)]
pub fn stop_rapl() {
    // Read the RAPL end values
    let (pkg_end, core_end) = read_rapl_registers();

    // Load in the RAPL start value
    // Safety: RAPL_START is only accessed in this function and only from a single thread
    let (pkg_start, core_start) = unsafe { RAPL_START };

    write_to_csv(
        (pkg_start, pkg_end, core_start, core_end),
        ["PkgStart", "PkgEnd", "CoreStart", "CoreEnd"],
    );

    /*
    // TODO: Revise if we can use timestamps

    let current_time = SystemTime::now();
    let duration_since_epoch = current_time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let timestamp = duration_since_epoch.as_millis();
    */
}

// Get the CPU type based on the compile time configuration
pub fn get_cpu_type() -> &'static str {
    #[cfg(intel)]
    {
        "Intel"
    }

    #[cfg(amd)]
    {
        "AMD"
    }
}

pub fn read_rapl_power_unit() -> Result<u64, RaplError> {
    read_msr(MSR_RAPL_POWER_UNIT)
}

pub fn read_rapl_pkg_energy_stat() -> Result<u64, RaplError> {
    read_msr(MSR_RAPL_PKG_ENERGY_STAT)
}

#[cfg(amd)]
fn read_rapl_registers() -> (u64, u64) {
    use self::amd::AMD_MSR_CORE_ENERGY;

    let pkg = read_rapl_pkg_energy_stat().expect("failed to read pkg energy stat");
    let core = read_msr(AMD_MSR_CORE_ENERGY).unwrap();

    (pkg, core)
}

#[cfg(intel)]
fn read_rapl_registers() -> (u64, u64, u64, u64) {
    use self::intel::{INTEL_MSR_RAPL_DRAM, INTEL_MSR_RAPL_PP0, INTEL_MSR_RAPL_PP1};

    let pp0 = read_msr(INTEL_MSR_RAPL_PP0).expect("failed to read PP0");
    let pp1 = read_msr(INTEL_MSR_RAPL_PP1).expect("failed to read PP1");
    let dram = read_msr(INTEL_MSR_RAPL_DRAM).expect("failed to read DRAM");
    let pkg = read_rapl_pkg_energy_stat().expect("failed to read PKG_ENERGY_STAT");

    (pp0, pp1, dram, pkg)
}

#[cfg(amd)]
pub mod amd {
    /*
    https://lore.kernel.org/lkml/20180817163442.10065-2-calvin.walton@kepstin.ca/

    "A notable difference from the Intel implementation is that AMD reports
    the "Cores" energy usage separately for each core, rather than a
    per-package total"
     */

    pub const MSR_RAPL_POWER_UNIT: u64 = 0xC0010299; // Similar to Intel MSR_RAPL_POWER_UNIT
    pub const MSR_RAPL_PKG_ENERGY_STAT: u64 = 0xC001029B; // Similar to Intel PKG_ENERGY_STATUS (This is for the whole socket)

    pub const AMD_MSR_CORE_ENERGY: u64 = 0xC001029A; // Similar to Intel PP0_ENERGY_STATUS (PP1 is for the GPU)

    /*
    const AMD_TIME_UNIT_MASK: u64 = 0xF0000;
    const AMD_ENERGY_UNIT_MASK: u64 = 0x1F00;
    const AMD_POWER_UNIT_MASK: u64 = 0xF;
    */
}

#[cfg(intel)]
pub mod intel {
    pub const MSR_RAPL_POWER_UNIT: u64 = 0x606;
    pub const MSR_RAPL_PKG_ENERGY_STAT: u64 = 0x611;

    pub const INTEL_MSR_RAPL_PP0: u64 = 0x639;
    pub const INTEL_MSR_RAPL_PP1: u64 = 0x641;
    pub const INTEL_MSR_RAPL_DRAM: u64 = 0x619;
    /*
    const INTEL_TIME_UNIT_MASK: u64 = 0xF000;
    const INTEL_ENGERY_UNIT_MASK: u64 = 0x1F00;
    const INTEL_POWER_UNIT_MASK: u64 = 0x0F;

    const INTEL_TIME_UNIT_OFFSET: u64 = 0x10;
    const INTEL_ENGERY_UNIT_OFFSET: u64 = 0x08;
    const INTEL_POWER_UNIT_OFFSET: u64 = 0;
    */
}
