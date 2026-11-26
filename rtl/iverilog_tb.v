`default_nettype none

// iverilog testbench module for lamb_labs_top
// Tests the fixed-point 16x16 dot product MAC with UART output

module tb;

// Clock generation — 100MHz; lamb_labs_top uses CFGMCLK 67.16MHz
reg clk;
always #5 clk = ~clk;

// Reset and start signals
reg rst_n;
reg start;

// Output signals from top-level
wire done;
wire [31:0] accum;
wire uart_tx;

// Monitor outputs - display at each clock edge
initial begin
    $monitor("time=%0t | rst_n=%b start=%b accum=%h uart_tx=%b done=%b", 
             $time, rst_n, start, accum, uart_tx, done);
end

// Generate a VCD dump for waveform inspection
initial begin
    $dumpfile("/tmp/lcd_tb.vcd");
    $dumpvars(0, tb);
end

// Stimulus process
initial begin
    // Initialize
    rst_n = 0;
    start = 0;
    
    // Assert reset
    #100;
    
    // De-assert reset
    rst_n = 1;
    
    // Wait a bit for reset to clear
    #50;
    
    // Pulse start to initiate dot product
    start = 1;
    #10;
    start = 0;
    
    // Run for many cycles to observe result
    // The mac16 computes on each cycle when en=1 and rst_n=1
    // After start pulse, accum should be valid after a few cycles
    #500;
    
    // Display final values
    $display("=== FINAL ===");
    $display("time=%0t | accum=%h (decimal=%d)", $time, accum, accum);
    $display("uart_tx=%b", uart_tx);
    $display("done=%b", done);
    
    // Finish
    $finish;
end

// Lamb_labs_top instance — connects to individual A0-A15, B0-B15 ports
lamb_labs_top u_lamb_labs_top (
    .clk(clk),
    .rst_n(rst_n),
    .start(start),
    
    // Vector A individual inputs (16 ports)
    .A0(16'h0080),
    .A1(16'h0080),
    .A2(16'h0080),
    .A3(16'h0080),
    .A4(16'h0080),
    .A5(16'h0080),
    .A6(16'h0080),
    .A7(16'h0080),
    .A8(16'h0080),
    .A9(16'h0080),
    .A10(16'h0080),
    .A11(16'h0080),
    .A12(16'h0080),
    .A13(16'h0080),
    .A14(16'h0080),
    .A15(16'h0080),
    
    // Vector B individual inputs (16 ports)
    .B0(16'h0080),
    .B1(16'h0080),
    .B2(16'h0080),
    .B3(16'h0080),
    .B4(16'h0080),
    .B5(16'h0080),
    .B6(16'h0080),
    .B7(16'h0080),
    .B8(16'h0080),
    .B9(16'h0080),
    .B10(16'h0080),
    .B11(16'h0080),
    .B12(16'h0080),
    .B13(16'h0080),
    .B14(16'h0080),
    .B15(16'h0080),
    
    .done(done),
    .accum(accum),
    .uart_tx(uart_tx)
);

endmodule

`default_nettype wire