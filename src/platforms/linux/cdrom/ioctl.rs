use nix::{ioctl_none_bad, ioctl_write_int_bad};

ioctl_none_bad!(cdromeject, 0x5309);
ioctl_none_bad!(cdromclosetray, 0x5319);
ioctl_write_int_bad!(cdrom_drive_status, 0x5326);
ioctl_write_int_bad!(cdrom_lockdoor, 0x5329);
