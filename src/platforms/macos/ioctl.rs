use nix::ioctl_none;

ioctl_none!(dkioceject, b'd', 21);
