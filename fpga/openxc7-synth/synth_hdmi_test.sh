#!/bin/bash
# M3 HDMI Proof Synthesis Script
# Generates bitstream for AX7203 HDMI test pattern

set -e

echo "=== M3 HDMI Proof Synthesis ==="
echo "Target: ax7203_hdmi_test_top.bit"
echo "Date: $(date)"
echo

# Check working directory
if [ ! -f "ax7203_hdmi_test_top.v" ]; then
    echo "ERROR: ax7203_hdmi_test_top.v not found"
    exit 1
fi

# Clean previous builds
rm -f ax7203_hdmi_test_top.json ax7203_hdmi_test_top.fasm ax7203_hdmi_test_top.frames ax7203_hdmi_test_top.bit

echo "Step 1: Yosys synthesis..."
yosys -p "
    read_verilog ax7203_hdmi_test_top.v hdmi_test_integration.v
    synth_xilinx -family xc7 -top ax7203_hdmi_test_top -flatten -run :coarse
    techmap -map +/cmp2lut.v -map +/cmp2lcu.v -D LUT_WIDTH=6
    alumacc
    opt
    memory -nomap
    opt_clean
    synth_xilinx -family xc7 -top ax7203_hdmi_test_top -flatten -run map_memory:
    delete t:\$print
    delete t:\$scopeinfo
    write_json ax7203_hdmi_test_top.json
" 2>&1 | tee synthesis.log

if [ ! -f ax7203_hdmi_test_top.json ]; then
    echo "ERROR: Yosys synthesis failed"
    cat synthesis.log
    exit 1
fi

echo "Step 2: NextPNR place and route..."
if [ ! -d "/Users/playom/t27/build/fpga/openxc7" ]; then
    echo "ERROR: OpenXC7 build directory not found"
    echo "Need to run toolchain recovery first"
    exit 1
fi

/Users/playom/t27/build/fpga/openxc7/nextpnr-xilinx/build/nextpnr-xilinx \
    --chipdb /Users/playom/t27/build/fpga/openxc7/xc7a200tfbg484-2.bin \
    --json ax7203_hdmi_test_top.json \
    --xdc ax7203_hdmi_test.xdc \
    --fasm ax7203_hdmi_test_top.fasm \
    --freq 125 \
    2>&1 | tee pnr.log

if [ $? -ne 0 ]; then
    echo "ERROR: NextPNR failed"
    cat pnr.log
    exit 1
fi

echo "Step 3: FASM to frames conversion..."
if [ ! -d "/Users/playom/t27/build/fpga/openxc7/prjxray" ]; then
    echo "ERROR: prjxray directory not found"
    exit 1
fi

export PYTHONPATH="/Users/playom/t27/build/fpga/openxc7/prjxray"
python /Users/playom/t27/build/fpga/openxc7/prjxray/utils/fasm2frames.py \
    --db-root /Users/playom/t27/build/fpga/openxc7/prjxray-db/artix7 \
    --part xc7a200tfbg484-2 \
    ax7203_hdmi_test_top.fasm > ax7203_hdmi_test_top.frames

if [ $? -ne 0 ]; then
    echo "ERROR: FASM to frames conversion failed"
    exit 1
fi

echo "Step 4: Generate bitstream..."
xc7frames2bit \
    --part_file /Users/playom/t27/build/fpga/openxc7/prjxray-db/artix7/xc7a200tfbg484-2/part.yaml \
    --part_name xc7a200tfbg484-2 \
    --frm_file ax7203_hdmi_test_top.frames \
    --output_file ax7203_hdmi_test_top.bit

if [ $? -ne 0 ]; then
    echo "ERROR: Bitstream generation failed"
    exit 1
fi

# Cleanup intermediate files (keep logs and bitstream)
rm -f ax7203_hdmi_test_top.json ax7203_hdmi_test_top.fasm ax7203_hdmi_test_top.frames

echo
echo "=== Synthesis Complete ==="
echo "Bitstream: ax7203_hdmi_test_top.bit"
echo "Size: $(ls -lh ax7203_hdmi_test_top.bit | awk '{print $5}')"
echo "SHA256: $(shasum -a 256 ax7203_hdmi_test_top.bit | cut -d' ' -f1)"
echo
echo "Ready for M3 HDMI Proof: Flash to AX7203 and verify on monitor"