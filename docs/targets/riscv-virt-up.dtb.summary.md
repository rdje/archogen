# The `riscv-virt-up` platform, as the pinned emulator presents it

This is the hardware description QEMU hands its guest under the pinned configuration: `qemu-system-riscv64`
`11.1.1` (`targets/riscv-virt-up.env`), `-machine virt -cpu rv64 -smp 1 -m 128M -bios none`. It is the device
tree `scripts/target_emulator.sh --dump-dtb` wrote on `2026-09-30`, kept beside this file as
`docs/targets/riscv-virt-up.dtb`, and rendered by `cargo xtask dtb-summary` into the block at the end. Nothing
below the block's marker is typed by hand (leaf `M2.8.2`).

It is checked, not trusted. The emulator step of the `integration` tier (`scripts/target_emulator.sh --check`)
dumps the device tree again and runs `cargo xtask dtb-check` against this file. Any line that differs fails
the step. A QEMU that presented another platform under the pinned options would be another target (§3.2,
"Do not rely on changing defaults"). A unit test in `xtask` checks that the kept dump renders to exactly this
block. Every node path this prose names is also checked against the platform, so the prose cannot name a
device the platform lacks.

One property is masked, by name. `/chosen` carries `rng-seed`, 32 random bytes that differ at every boot.
Two dumps with the pinned options differ there and nowhere else.

## What `rt-static-up-v1` stands on

`ROADMAP.md` §3.1 asks of a target: one core, a timer, observable output, executable RAM. These are the
nodes that supply them, and a change to any of them changes what the runtime can promise:

| Need | Node | What the dump says |
| --- | --- | --- |
| one core | `/cpus/cpu@0` | the only hart; `/cpus` holds no other |
| executable RAM | `/memory@80000000` | 128 MiB at `0x8000_0000`, where `-bios none` starts the image |
| a timer | `/soc/clint@2000000` | `sifive,clint0`, 64 KiB; `/cpus` gives `timebase-frequency` 10 MHz |
| the timer's interrupt route | `/cpus/cpu@0/interrupt-controller` | the hart's own `riscv,cpu-intc`; the CLINT's `interrupts-extended` delivers machine-software (3) and machine-timer (7) interrupts to it |
| observable output | `/soc/serial@10000000` | `ns16550a`, the node `/chosen`'s `stdout-path` names |

The rest is present and carries nothing the runtime relies on today. That covers the external interrupt
controller `/soc/interrupt-controller@c000000` (`sifive,plic-1.0.0`, routing the UART's interrupt 10), the
`/soc/rtc@101000`, the `/soc/test@100000` device behind `/poweroff` and `/reboot`, `/fw-cfg@10100000`,
`/flash@20000000`, `/soc/pci@30000000`, `/platform-bus@4000000`, the performance-monitoring `/pmu`, and the
`virtio` transports from `/soc/virtio_mmio@10001000` to `/soc/virtio_mmio@10008000`.

## The ISA it offers, and the subset archogen builds

`/cpus/cpu@0` offers `rv64imafdch` with a long tail of `z*` and `s*` extensions: floating point (`f`, `d`),
the hypervisor extension (`h`), compressed instructions and more. archogen builds
`riscv64imac-unknown-none-elf`: integer, multiply, atomics and compressed instructions, with **no floating
point**. That is a deliberate subset, and a correct one. The engine and the generated runtime use no floating
point (`M1.3`; `docs/decisions/decision_zero-dependency-engine-core.md`), and code built for the subset runs on
a hart that offers more. A reader who sees `f` and `d` here should not "fix" the build by enabling them.

## The device tree

<!-- BEGIN GENERATED: cargo xtask dtb-summary -->
```text
/
  #address-cells = <0x2>
  #size-cells = <0x2>
  compatible = "riscv-virtio"
  model = "riscv-virtio,qemu"
/poweroff
  value = <0x5555>
  offset = <0x0>
  regmap = <0x4>
  compatible = "syscon-poweroff"
/reboot
  value = <0x7777>
  offset = <0x0>
  regmap = <0x4>
  compatible = "syscon-reboot"
/platform-bus@4000000
  interrupt-parent = <0x3>
  ranges = <0x0 0x0 0x4000000 0x2000000>
  #address-cells = <0x1>
  #size-cells = <0x1>
  compatible = "qemu,platform", "simple-bus"
/memory@80000000
  device_type = "memory"
  reg = <0x0 0x80000000 0x0 0x8000000>
/cpus
  #address-cells = <0x1>
  #size-cells = <0x0>
  timebase-frequency = <0x989680>
/cpus/cpu@0
  phandle = <0x1>
  device_type = "cpu"
  reg = <0x0>
  status = "okay"
  compatible = "riscv"
  riscv,cbop-block-size = <0x40>
  riscv,cboz-block-size = <0x40>
  riscv,cbom-block-size = <0x40>
  riscv,isa-extensions = "i", "m", "a", "f", "d", "c", "h", "zic64b", "zicbom", "zicbop", "zicboz", "ziccamoa", "ziccif", "zicclsm", "ziccrse", "zicntr", "zicsr", "zifencei", "zihintntl", "zihintpause", "zihpm", "zmmul", "za64rs", "zaamo", "zalrsc", "zawrs", "zfa", "zca", "zcd", "zba", "zbb", "zbc", "zbs", "sdtrig", "shcounterenw", "shgatpa", "shtvala", "shvsatpa", "shvstvala", "shvstvecd", "ssccptr", "sscounterenw", "ssstrict", "sstc", "sstvala", "sstvecd", "ssu64xl", "svadu", "svvptc"
  riscv,isa-base = "rv64i"
  riscv,isa = "rv64imafdch_zic64b_zicbom_zicbop_zicboz_ziccamoa_ziccif_zicclsm_ziccrse_zicntr_zicsr_zifencei_zihintntl_zihintpause_zihpm_zmmul_za64rs_zaamo_zalrsc_zawrs_zfa_zca_zcd_zba_zbb_zbc_zbs_sdtrig_shcounterenw_shgatpa_shtvala_shvsatpa_shvstvala_shvstvecd_ssccptr_sscounterenw_ssstrict_sstc_sstvala_sstvecd_ssu64xl_svadu_svvptc"
  mmu-type = "riscv,sv57"
/cpus/cpu@0/interrupt-controller
  #interrupt-cells = <0x1>
  interrupt-controller
  compatible = "riscv,cpu-intc"
  phandle = <0x2>
/cpus/cpu-map
/cpus/cpu-map/cluster0
/cpus/cpu-map/cluster0/core0
  cpu = <0x1>
/pmu
  riscv,event-to-mhpmcounters = <0x1 0x1 0x7fff9 0x2 0x2 0x7fffc 0x10019 0x10019 0x7fff8 0x1001b 0x1001b 0x7fff8 0x10021 0x10021 0x7fff8>
  compatible = "riscv,pmu"
/fw-cfg@10100000
  dma-coherent
  reg = <0x0 0x10100000 0x0 0x18>
  compatible = "qemu,fw-cfg-mmio"
/flash@20000000
  bank-width = <0x4>
  reg = <0x0 0x20000000 0x0 0x2000000 0x0 0x22000000 0x0 0x2000000>
  compatible = "cfi-flash"
/aliases
  serial0 = "/soc/serial@10000000"
/chosen
  stdout-path = "/soc/serial@10000000"
  rng-seed = <32 bytes, random at every boot>
/soc
  #address-cells = <0x2>
  #size-cells = <0x2>
  compatible = "simple-bus"
  ranges
/soc/rtc@101000
  interrupts = <0xb>
  interrupt-parent = <0x3>
  reg = <0x0 0x101000 0x0 0x1000>
  compatible = "google,goldfish-rtc"
/soc/serial@10000000
  interrupts = <0xa>
  interrupt-parent = <0x3>
  clock-frequency = <0x384000>
  reg = <0x0 0x10000000 0x0 0x100>
  compatible = "ns16550a"
/soc/test@100000
  phandle = <0x4>
  reg = <0x0 0x100000 0x0 0x1000>
  compatible = "sifive,test1", "sifive,test0", "syscon"
/soc/virtio_mmio@10008000
  interrupts = <0x8>
  interrupt-parent = <0x3>
  reg = <0x0 0x10008000 0x0 0x1000>
  compatible = "virtio,mmio"
/soc/virtio_mmio@10007000
  interrupts = <0x7>
  interrupt-parent = <0x3>
  reg = <0x0 0x10007000 0x0 0x1000>
  compatible = "virtio,mmio"
/soc/virtio_mmio@10006000
  interrupts = <0x6>
  interrupt-parent = <0x3>
  reg = <0x0 0x10006000 0x0 0x1000>
  compatible = "virtio,mmio"
/soc/virtio_mmio@10005000
  interrupts = <0x5>
  interrupt-parent = <0x3>
  reg = <0x0 0x10005000 0x0 0x1000>
  compatible = "virtio,mmio"
/soc/virtio_mmio@10004000
  interrupts = <0x4>
  interrupt-parent = <0x3>
  reg = <0x0 0x10004000 0x0 0x1000>
  compatible = "virtio,mmio"
/soc/virtio_mmio@10003000
  interrupts = <0x3>
  interrupt-parent = <0x3>
  reg = <0x0 0x10003000 0x0 0x1000>
  compatible = "virtio,mmio"
/soc/virtio_mmio@10002000
  interrupts = <0x2>
  interrupt-parent = <0x3>
  reg = <0x0 0x10002000 0x0 0x1000>
  compatible = "virtio,mmio"
/soc/virtio_mmio@10001000
  interrupts = <0x1>
  interrupt-parent = <0x3>
  reg = <0x0 0x10001000 0x0 0x1000>
  compatible = "virtio,mmio"
/soc/interrupt-controller@c000000
  phandle = <0x3>
  riscv,ndev = <0x5f>
  reg = <0x0 0xc000000 0x0 0x600000>
  interrupts-extended = <0x2 0xb 0x2 0x9>
  interrupt-controller
  compatible = "sifive,plic-1.0.0", "riscv,plic0"
  #address-cells = <0x0>
  #interrupt-cells = <0x1>
/soc/clint@2000000
  interrupts-extended = <0x2 0x3 0x2 0x7>
  reg = <0x0 0x2000000 0x0 0x10000>
  compatible = "sifive,clint0", "riscv,clint0"
/soc/pci@30000000
  interrupt-map-mask = <0x1800 0x0 0x0 0x7>
  interrupt-map = <0x0 0x0 0x0 0x1 0x3 0x20 0x0 0x0 0x0 0x2 0x3 0x21 0x0 0x0 0x0 0x3 0x3 0x22 0x0 0x0 0x0 0x4 0x3 0x23 0x800 0x0 0x0 0x1 0x3 0x21 0x800 0x0 0x0 0x2 0x3 0x22 0x800 0x0 0x0 0x3 0x3 0x23 0x800 0x0 0x0 0x4 0x3 0x20 0x1000 0x0 0x0 0x1 0x3 0x22 0x1000 0x0 0x0 0x2 0x3 0x23 0x1000 0x0 0x0 0x3 0x3 0x20 0x1000 0x0 0x0 0x4 0x3 0x21 0x1800 0x0 0x0 0x1 0x3 0x23 0x1800 0x0 0x0 0x2 0x3 0x20 0x1800 0x0 0x0 0x3 0x3 0x21 0x1800 0x0 0x0 0x4 0x3 0x22>
  ranges = <0x1000000 0x0 0x0 0x0 0x3000000 0x0 0x10000 0x2000000 0x0 0x40000000 0x0 0x40000000 0x0 0x40000000 0x3000000 0x4 0x0 0x4 0x0 0x4 0x0>
  reg = <0x0 0x30000000 0x0 0x10000000>
  dma-coherent
  bus-range = <0x0 0xff>
  linux,pci-domain = <0x0>
  device_type = "pci"
  compatible = "pci-host-ecam-generic"
  #size-cells = <0x2>
  #interrupt-cells = <0x1>
  #address-cells = <0x3>
```
<!-- END GENERATED -->
