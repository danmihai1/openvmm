# Emulated NVDIMM

OpenVMM can expose a file-backed persistent-memory range as an ACPI NVDIMM.

## Configuration

The RPC compatibility path translates a `VirtioPmem` device in the initial
PCIe topology into an emulated NVDIMM. The named PCIe port remains reserved,
but OpenVMM does not create a virtio-pmem endpoint for the request.

The backing file must be:

- a non-empty regular file;
- readable by OpenVMM;
- a multiple of 4 KiB in size.

The NVDIMM is static. It must be included in `CreateVM` and cannot be added
after VM creation.

The `--virtio-pmem` command-line option continues to create a virtio-pmem
device. The ACPI NVDIMM translation currently applies only to the RPC
`VirtioPmem` compatibility path.

## Guest interface

OpenVMM reserves a guest physical address range for the image and describes it
with an NVDIMM Firmware Interface Table (NFIT). The table contains:

- a byte-addressable persistent-memory SPA range;
- a memory-device mapping marked `NOT_ARMED`;
- a control region without block windows;
- an `ACPI0012` namespace device and a child whose `_ADR` matches the NFIT
  device handle.

The device deliberately has no namespace-label methods. Linux therefore
creates one label-less namespace spanning the complete image. A partitioned
image normally appears as `/dev/pmem0` and `/dev/pmem0p1`.

Linux interprets `NOT_ARMED` as read-only and reports:

```bash
cat /sys/class/block/pmem0/ro
```

The expected value is `1`. Raw block writes fail without changing the guest
view or the host file.

## Host mapping

OpenVMM maps the backing file privately with copy-on-write permissions. This
allows hypervisors that require a writable userspace memory slot to read the
image while preventing guest writes from modifying the backing file.

```admonish warning
NFIT read-only state is enforced by the guest NVDIMM and block drivers. The
host mapping remains writable copy-on-write for hypervisor compatibility.
A compromised guest kernel that writes directly to the SPA can alter its
private view, but it cannot persist those changes to the backing file.
```

The implementation uses `acpi_spec`, `vmm_core`, and `membacking`.
