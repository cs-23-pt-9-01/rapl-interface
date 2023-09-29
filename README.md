# rapl-rust-test

The repository for RAPL testing.

## GitHub Actions

An issue currently too is that MSR's are not accessible with GitHub Actions, likely due to that it runs under Docker.

## Windows

Currently this does not work on Windows because `readmsr` requires kernel access. It will require a kernel driver to make it work. Intel Power Gadget can support it by design but it will only be for Intel processors in that case.

## RAPL test

https://github.com/djselbeck/rapl-read-ryzen

https://me.sakana.moe/2023/09/06/measuring-cpu-power-consumption/

https://github.com/hubblo-org/windows-rapl-driver

https://github.com/amd/amd_energy

## Test for CPU

https://github.com/RRZE-HPC/likwid/issues/373

List the CPU's MSR's on Linux.

`ls -la /dev/cpu/*/msr`

Enable MSR.

`sudo modprobe msr`

## Install driver on Windows

Use command prompt as administrator.

Create:

`sc create rapl type= kernel binPath= "C:\\Users\\Jakob\\Documents\\GitHub\\cs-23-pt-9-01\\rapl-rust-test\\LibreHardwareMonitor.sys"`

Start:

`sc start rapl`

Stop:

`sc stop rapl`

Delete:

`sc delete rapl`
