`default_nettype none
module bram_a (
    input  wire        clk,
    input  wire        rst_n,
    input  wire        en,
    input  wire [3:0]  addr,
    output reg [15:0]  dout0,
    output reg [15:0]  dout1,
    output reg [15:0]  dout2,
    output reg [15:0]  dout3,
    output reg [15:0]  dout4,
    output reg [15:0]  dout5,
    output reg [15:0]  dout6,
    output reg [15:0]  dout7,
    output reg [15:0]  dout8,
    output reg [15:0]  dout9,
    output reg [15:0]  dout10,
    output reg [15:0]  dout11,
    output reg [15:0]  dout12,
    output reg [15:0]  dout13,
    output reg [15:0]  dout14,
    output reg [15:0]  dout15
);

// Preload test values: all 0x0080 (128 in Q7.8 format)
// These are set once at initialization and preserved while rst_n is high
initial begin
    dout0 <= 16'h0080;  dout1 <= 16'h0080;  dout2 <= 16'h0080;  dout3 <= 16'h0080;
    dout4 <= 16'h0080;  dout5 <= 16'h0080;  dout6 <= 16'h0080;  dout7 <= 16'h0080;
    dout8 <= 16'h0080;  dout9 <= 16'h0080;  dout10 <= 16'h0080; dout11 <= 16'h0080;
    dout12 <= 16'h0080; dout13 <= 16'h0080; dout14 <= 16'h0080; dout15 <= 16'h0080;
end

// BRAM output registers: preserve preload values while rst_n is high
// When rst_n goes low, values restore to 0x0080 preload (not zero)
always @(posedge clk) begin
    if (!rst_n) begin
        // Reset case: restore preload values (0x0080 in Q7.8 format)
        dout0 <= 16'h0080;  dout1 <= 16'h0080;  dout2 <= 16'h0080;  dout3 <= 16'h0080;
        dout4 <= 16'h0080;  dout5 <= 16'h0080;  dout6 <= 16'h0080;  dout7 <= 16'h0080;
        dout8 <= 16'h0080;  dout9 <= 16'h0080;  dout10 <= 16'h0080; dout11 <= 16'h0080;
        dout12 <= 16'h0080; dout13 <= 16'h0080; dout14 <= 16'h0080; dout15 <= 16'h0080;
    end
    // When rst_n is high, preload values persist (no update needed - they hold)
end

endmodule
