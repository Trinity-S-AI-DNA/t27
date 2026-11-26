`default_nettype none
module mac16 (
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

// Fixed: initialize accum to 0 to avoid X-propagation
initial accum = 32'd0;

// Fixed: compute products and accumulate in always block
// Use registers for intermediates to avoid X-propagation
reg [31:0] p0, p1, p2, p3;
reg [31:0] p4, p5, p6, p7;
reg [31:0] p8, p9, p10, p11;
reg [31:0] p12, p13, p14, p15;
reg [31:0] sum;

always @(*) begin
    // Compute each product using $unsigned to ensure deterministic results
    p0 = $unsigned(A0) * $unsigned(B0);
    p1 = $unsigned(A1) * $unsigned(B1);
    p2 = $unsigned(A2) * $unsigned(B2);
    p3 = $unsigned(A3) * $unsigned(B3);
    p4 = $unsigned(A4) * $unsigned(B4);
    p5 = $unsigned(A5) * $unsigned(B5);
    p6 = $unsigned(A6) * $unsigned(B6);
    p7 = $unsigned(A7) * $unsigned(B7);
    p8 = $unsigned(A8) * $unsigned(B8);
    p9 = $unsigned(A9) * $unsigned(B9);
    p10 = $unsigned(A10) * $unsigned(B10);
    p11 = $unsigned(A11) * $unsigned(B11);
    p12 = $unsigned(A12) * $unsigned(B12);
    p13 = $unsigned(A13) * $unsigned(B13);
    p14 = $unsigned(A14) * $unsigned(B14);
    p15 = $unsigned(A15) * $unsigned(B15);
    
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