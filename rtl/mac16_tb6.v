`default_nettype none

module mac16_tb6;
    // Clock generation — 100MHz
    reg clk;
    always #5 clk = ~clk;

    // Reset
    reg rst_n;

    // mac16 signals
    reg en;
    
    // Vector A (16 elements, 16-bit each) — individual inputs
    reg [15:0] A0,
               A1,
               A2,
               A3,
               A4,
               A5,
               A6,
               A7,
               A8,
               A9,
               A10,
               A11,
               A12,
               A13,
               A14,
               A15;
    
    // Vector B (16 elements, 16-bit each) — individual inputs
    reg [15:0] B0,
               B1,
               B2,
               B3,
               B4,
               B5,
               B6,
               B7,
               B8,
               B9,
               B10,
               B11,
               B12,
               B13,
               B14,
               B15;
    
    wire [31:0] accum;
    wire done;
    
    mac16 u_mac16 (
        .clk(clk),
        .rst_n(rst_n),
        .en(en),
        .A0(A0), .A1(A1), .A2(A2), .A3(A3),
        .A4(A4), .A5(A5), .A6(A6), .A7(A7),
        .A8(A8), .A9(A9), .A10(A10), .A11(A11),
        .A12(A12), .A13(A13), .A14(A14), .A15(A15),
        .B0(B0), .B1(B1), .B2(B2), .B3(B3),
        .B4(B4), .B5(B5), .B6(B6), .B7(B7),
        .B8(B8), .B9(B9), .B10(B10), .B11(B11),
        .B12(B12), .B13(B13), .B14(B14), .B15(B15),
        .done(done),
        .accum(accum)
    );
    
    initial begin
        $monitor("time=%0t | rst_n=%b en=%b accum=%h done=%b", $time, rst_n, en, accum, done);
        
        // Initialize
        rst_n = 0;
        en = 0;
        A0 = A1 = A2 = A3 = A4 = A5 = A6 = A7 = 16'h0080;
        A8 = A9 = A10 = A11 = A12 = A13 = A14 = A15 = 16'h0080;
        B0 = B1 = B2 = B3 = B4 = B5 = B6 = B7 = 16'h0080;
        B8 = B9 = B10 = B11 = B12 = B13 = B14 = B15 = 16'h0080;
        
        // Apply reset
        #100;
        rst_n = 1;
        en = 1;
        
        // Wait for computation
        #500;
        
        $display("FINAL: accum=%h (expected 0x00000400 = 1024)", accum);
        
        $finish;
    end
endmodule
