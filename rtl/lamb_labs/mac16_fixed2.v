`default_nettype none
module mac16_fixed2 (
    input  wire        clk,
    input  wire        rst_n,
    input  wire        en,

    // Vector A (16 elements, 16-bit each) — individual inputs
    input  wire [15:0] A0,
    input  wire [15:0] A1,
    input  wire [15:0] A2,
    input  wire [15:0] A3,
    input  wire [15:0] A4,
    input  wire [15:0] A5,
    input  wire [15:0] A6,
    input  wire [15:0] A7,
    input  wire [15:0] A8,
    input  wire [15:0] A9,
    input  wire [15:0] A10,
    input  wire [15:0] A11,
    input  wire [15:0] A12,
    input  wire [15:0] A13,
    input  wire [15:0] A14,
    input  wire [15:0] A15,

    // Vector B (16 elements, 16-bit each) — individual inputs
    input  wire [15:0] B0,
    input  wire [15:0] B1,
    input  wire [15:0] B2,
    input  wire [15:0] B3,
    input  wire [15:0] B4,
    input  wire [15:0] B5,
    input  wire [15:0] B6,
    input  wire [15:0] B7,
    input  wire [15:0] B8,
    input  wire [15:0] B9,
    input  wire [15:0] B10,
    input  wire [15:0] B11,
    input  wire [15:0] B12,
    input  wire [15:0] B13,
    input  wire [15:0] B14,
    input  wire [15:0] B15,

    output reg         done,
    output reg [31:0]  accum
);

// Fixed: compute products directly, no concatenation
// Use registers for intermediates
reg [31:0] p0, p1, p2, p3;
reg [31:0] p4, p5, p6, p7;
reg [31:0] p8, p9, p10, p11;
reg [31:0] p12, p13, p14, p15;
reg [31:0] sum;

always @(*) begin
    // Compute each product directly - just A * B
    p0 = A0 * B0;
    p1 = A1 * B1;
    p2 = A2 * B2;
    p3 = A3 * B3;
    p4 = A4 * B4;
    p5 = A5 * B5;
    p6 = A6 * B6;
    p7 = A7 * B7;
    p8 = A8 * B8;
    p9 = A9 * B9;
    p10 = A10 * B10;
    p11 = A11 * B11;
    p12 = A12 * B12;
    p13 = A13 * B13;
    p14 = A14 * B14;
    p15 = A15 * B15;
    
    // Sum all products
    sum = p0 + p1 + p2 + p3 + p4 + p5 + p6 + p7 +
          p8 + p9 + p10 + p11 + p12 + p13 + p14 + p15;
end

always @(posedge clk or negedge rst_n) begin
    if (!rst_n) begin
        accum <= 32'd0;
        done <= 1'b0;
    end else if (en) begin
        accum <= sum >>> 8;
        done <= 1'b1;
    end else begin
        done <= 1'b0;
    end
end

endmodule