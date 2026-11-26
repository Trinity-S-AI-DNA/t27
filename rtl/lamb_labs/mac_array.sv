`default_nettype none
module mac_array (
    input  wire        clk,        // CFGMCLK domain
    input  wire        rst_n,      // Active-low reset
    input  wire        en,         // Enable

    // Matrix inputs from BRAM (16 words × 16-bit for dot product)
    // Using individual ports instead of unpacked array for iVerilog compatibility
    input  wire        A0,  input  wire        A1,  input  wire        A2,  input  wire        A3,
    input  wire        A4,  input  wire        A5,  input  wire        A6,  input  wire        A7,
    input  wire        A8,  input  wire        A9,  input  wire        A10, input  wire        A11,
    input  wire        A12, input  wire        A13, input  wire        A14, input  wire        A15,

    // Matrix B inputs
    input  wire        B0,  input  wire        B1,  input  wire        B2,  input  wire        B3,
    input  wire        B4,  input  wire        B5,  input  wire        B6,  input  wire        B7,
    input  wire        B8,  input  wire        B9,  input  wire        B10, input  wire        B11,
    input  wire        B12, input  wire        B13, input  wire        B14, input  wire        B15,

    // Control
    input  wire        start,      // Start new computation
    output reg         done,       // Computation complete

    // Accumulator result
    output reg [31:0]  accum       // 32-bit accumulated result
);

// Internal signals for partial products
wire [31:0] p0, p1, p2, p3, p4, p5, p6, p7;
wire [31:0] p8, p9, p10, p11, p12, p13, p14, p15;

// Partial product assignments (16×16 fixed-point multiply with >>>8)
// Each: A[i] * B[i] truncated to 8 fractional bits
assign p0 = {8'd0, {A0, A1, A2, A3, A4, A5, A6, A7}} * {8'd0, {B0, B1, B2, B3, B4, B5, B6, B7}};
// Actually this doesn't work - need to do each individually

// Per-element multiplies
assign p0 = {8'd0, A0} * {8'd0, B0};
assign p1 = {8'd0, A1} * {8'd0, B1};
assign p2 = {8'd0, A2} * {8'd0, B2};
assign p3 = {8'd0, A3} * {8'd0, B3};
assign p4 = {8'd0, A4} * {8'd0, B4};
assign p5 = {8'd0, A5} * {8'd0, B5};
assign p6 = {8'd0, A6} * {8'd0, B6};
assign p7 = {8'd0, A7} * {8'd0, B7};
assign p8 = {8'd0, A8} * {8'd0, B8};
assign p9 = {8'd0, A9} * {8'd0, B9};
assign p10 = {8'd0, A10} * {8'd0, B10};
assign p11 = {8'd0, A11} * {8'd0, B11};
assign p12 = {8'd0, A12} * {8'd0, B12};
assign p13 = {8'd0, A13} * {8'd0, B13};
assign p14 = {8'd0, A14} * {8'd0, B14};
assign p15 = {8'd0, A15} * {8'd0, B15};

// Sum all 16 partial products and apply final >>>8 shift
always @(posedge clk or negedge rst_n) begin
    if (!rst_n) begin
        accum <= 32'd0;
        done <= 1'b0;
    end else if (en) begin
        // Sum products and shift right by 8 (truncate fractional bits)
        accum <= (p0 + p1 + p2 + p3 + p4 + p5 + p6 + p7 +
                   p8 + p9 + p10 + p11 + p12 + p13 + p14 + p15) >>> 8;
        done <= 1'b1;
    end else begin
        done <= 1'b0;
    end
end

// When en=0, hold previous values (retention)

endmodule