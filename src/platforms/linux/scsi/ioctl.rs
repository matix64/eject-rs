use super::sg_io_hdr::SgIoHdr;
use nix::ioctl_readwrite_bad;

ioctl_readwrite_bad!(sg_io, 0x2285, SgIoHdr);
