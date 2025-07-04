#!/bin/bash

set -euo pipefail

cmd=$(basename $0)

name=chromeos
recoveryDriveOpt=()
cdromOpt=()
vmRootDir="$HOME/vm/qemu"
xres=1280
yres=800
memory=4G
smp=2
hddSize=50G
vga="-device virtio-vga-gl,xres="$xres",yres="$yres""

if [ -f ~/.chromeoslauncher ];then
   source ~/.chromeoslauncher
fi

usage() {
    cat <<EOF
usage: ${cmd} [options]
    options:
    -r recoveryImage
    -c isoImage
    -n name
    -s num :speficy number of cpu cores(default: $smp)
    -3 : use virtual 3d acceleration
EOF
}


while getopts 3s:r:c:n:h OPT; do
    case $OPT in
        r)  recoveryDriveOpt+=("-drive")
            recoveryDriveOpt+=("format=raw,file=$OPTARG")
            ;;
        c)  cdromOpt+=("-boot")
            cdromOpt+=("order=d")
            cdromOpt+=("-cdrom")
            cdromOpt+=("$OPTARG")
            ;;
        n)  name="$OPTARG"
            ;;
        s)  smp="$OPTARG"
            ;;
        3)  vga="-vga virtio"
            ;;
        ?)  usage
            exit 1
            ;;
    esac
done
shift  `expr  $OPTIND  -  1`

vmDir="$vmRootDir/$name"

if [ -f "$vmDir"/.chromeoslauncher ];then
   source "$vmDir"/.chromeoslauncher
fi

hddImage="$vmDir/image.qcow2"

if [ ! -d "$vmDir" ]; then
   mkdir -p "$vmDir"
fi

if [ ! -f "$hddImage" ]; then
   qemu-img create -f qcow2 "$hddImage" "$hddSize"
fi

if [ ! -f "$vmDir"/OVMF_VARS_4M.fd ]; then
   cp /usr/share/OVMF/OVMF_VARS_4M.fd "$vmDir"
fi

if [ "${#cdromOpt[@]}" -gt 0 ]; then
    exec qemu-system-x86_64 \
    -display sdl,show-cursor=on \
    "${cdromOpt[@]}" \
    -drive format=qcow2,file="$hddImage" \
    -m "$memory" \
    -enable-kvm \
    -smp "$smp" \
    -usb -device usb-tablet

    exit
fi

exec qemu-system-x86_64 \
    -drive if=pflash,format=raw,readonly=on,file=/usr/share/OVMF/OVMF_CODE_4M.fd \
    -drive if=pflash,format=raw,file=$vmDir/OVMF_VARS_4M.fd \
    -display sdl,show-cursor=on,gl=on \
    $vga \
    -usb -device usb-tablet \
    "${recoveryDriveOpt[@]}" \
    -drive format=qcow2,file="$hddImage" \
    -m "$memory" \
    -enable-kvm \
    -smp "$smp" \
    -audiodev sdl,id=audio0 \
    -device intel-hda -device hda-output,audiodev=audio0 \
    -cpu host
