`default_nettype none
//
module test_tb;
    //
    // Clock generation (free-running, ~100MHz)
    //
    reg clk;
    initial begin
        clk = 1'b0;
        forever #5 clk = ~clk;  // 100MHz clock, 5 time units period
    end
    //
    // DUT signals
    //
    reg rst_n;
    reg start;  // Active-high pulse to start FSM
    wire [31:0] accum;
    wire done;
    //
    // Instantiate the fixed lamb_labs_top with FSM generating bram_en
    //
    lamb_labs_top u_top (
        .clk(clk),
        .rst_n(rst_n),
        .start(start),
        
        // Vector A inputs - all 0x0080
        .A0(16'h0080), .A1(16'h0080), .A2(16'h0080), .A3(16'h0080),
        .A4(16'h0080), .A5(16'h0080), .A6(16'h0080), .A7(16'h0080),
        .A8(16'h0080), .A9(16'h0080), .A10(16'h0080), .A11(16'h0080),
        .A12(16'h0080), .A13(16'h0080), .A14(16'h0080), .A15(16'h0080),
        
        // Vector B inputs - all 0x0080
        .B0(16'h0080), .B1(16'h0080), .B2(16'h0080), .B3(16'h0080),
        .B4(16'h0080), .B5(16'h0080), .B6(16'h0080), .B7(16'h0080),
        .B8(16'h0080), .B9(16'h0080), .B10(16'h0080), .B11(16'h0080),
        .B12(16'h0080), .B13(16'h0080), .B14(16'h0080), .B15(16'h0080),
        
        .accum(accum),
        .done(done)
    );
    //
    // FSM test: hold rst_N permanently high (1'b1) so BRAM preload preserved
    // Apply active-high start pulse on start
    //
    initial begin
        start = 1'b0;  // Idle low
        rst_n = 1'b1;  // Permanently high - preserves BRAM preload values (dout=0x0080)
        // Apply active-high start pulse after clock edges for FSM capture
        #10;           // Wait for initial clock settling (2 cycles)
        @(posedge clk); // Wait for positive clock edge
        start = 1'b1;  // Pulse goes high to start FSM
        @(posedge clk); // Wait for next positive clock edge
        start = 1'b0;  // Pulse goes low - FSM now runs independently
    end
    //
    // Main simulation: run for enough clock cycles for FSM + 16 MAC cycles to complete
    //
    initial begin
        // Wait for first clock edge to settle
        #10;
        // Run simulation for 350 time units - enough for full FSM sequence + 16 MAC cycles
        #350 $finish;
    end
    //
    // Monitor accum value - display final value
    //
    initial begin
        #350;
        $display("=========================================");
        $display("FINAL TIME=%0t: accum = %h", $time, accum);
        $display("done = %b", done);
        $display("Expected: accum = 0x00000400 (16 * 0x0080 * 0x0080 >>> 8)");
        $display("=========================================");
        if (accum == 32'h00000400) begin
            $display("PASS: accum_out matches expected value!");
        end else begin
            $display("FAIL: accum_out mismatch! Got %h, expected 0x00000400", accum);
        end
    end
endmodule
