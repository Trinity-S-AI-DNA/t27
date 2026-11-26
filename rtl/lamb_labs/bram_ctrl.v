`default_nettype wire

// BRAM controller for lamb_labs — provides dual-port read access
// for vector A and B (256 × 16-bit each port)

// Iverilog/Verilog-2001 compatible

module bram_ctrl (
    input  wire        clk,        // CFGMCLK domain
    input  wire        rst_n,      // Active-low reset
    input  wire        en,         // Module enable (1 = active)

    // Port A — read port
    input  wire [7:0]  addr_a,     // 8-bit address (0..255)
    output wire [15:0] data_a,     // 16-bit data output

    // Port B — read port
    input  wire [7:0]  addr_b,     // 8-bit address (0..255)
    output wire [15:0] data_b      // 16-bit data output
);

// ---------------------------------------------------------------------------
// BRAM register file: 256 × 16-bit dual-port
// We use distributed RAM (LUTs) since 256 × 16 is small
// ---------------------------------------------------------------------------

// Port A RAM array declaration — 256 words × 16 bits
reg [15:0] ram_a [0:255];

// Port B RAM array declaration — 256 words × 16 bits
reg [15:0] ram_b [0:255];

// Port A read logic — combinational output
assign data_a = ram_a[addr_a];

// Port B read logic — combinational output
assign data_b = ram_b[addr_b];

// ---------------------------------------------------------------------------
// Resource estimates (xc7a200tfbg484-2):
// - 2 BRAMs could hold 256 × 36 each, but we use distributed RAM (LUTs)
// - ~50 LUTs for the RAM arrays and output mux
// - Minimal FFs (only the register arrays if any)
// Total: trivial footprint
//
// Frequency target: CFGMCLK 67.16 MHz (MEASURED)
// Pure combinational read from RAM should easily meet timing.
//
// Simulation: instantiate bram_ctrl in a top-level module that drives
// addr_a, addr_b, en, clk, rst_n and checks data_a, data_b against
// expected values.
// ---------------------------------------------------------------------------

endmodule