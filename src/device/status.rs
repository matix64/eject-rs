/// Position of the drive's tray and whether it has data loaded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DriveStatus {
    /// No medium is inserted. If there's a tray, it's closed.
    Empty,
    /// The drive has a tray or other similar mechanism and it's open.
    TrayOpen,
    /// The drive is not available yet. With CD drives this happens for a few
    /// seconds after the tray is closed.
    ///
    /// This status is not supported on Windows. There it'll be reported as [Empty][Self::Empty].
    NotReady,
    /// The drive has data loaded. If it reads from removable media
    /// (e.g. CDs/floppy/SD cards) then one is inserted.
    Loaded,
}

impl DriveStatus {
    /// Returns whether this status implies that the tray is open.
    pub const fn tray_open(&self) -> bool {
        match self {
            Self::Empty | Self::Loaded | Self::NotReady => false,
            Self::TrayOpen => true,
        }
    }
}
