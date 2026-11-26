`timescale 1ns/1ps

module test_m3;
    // ---------------------------------------------------------------------------
    // Clock and reset
    // ---------------------------------------------------------------------------
    reg clk;
    reg rst_n;
    reg start;

    // Output signals
    wire uart_tx;

    // Instantiate top module
    lamb_labs_top u_top (
        .clk        (clk),
        .rst_n      (rst_n),
        .start      (start),
        .A0         (16'h0080),  .A1         (16'h0080),  .A2         (16'h0080),  .A3         (16'h0080),
        .A4         (16'h0080),  .A5         (16'h0080),  .A6         (16'h0080),  .A7         (16'h0080),
        .A8         (16'h0080),  .A9         (16'h0080),  .A10        (16'h0080), .A11        (16'h0080),
        .A12        (16'h0080), .A13        (16'h0080), .A14        (16'h0080), .A15        (16'h0080),
        .B0         (16'h0080),  .B1         (16'h0080),  .B2         (16'h0080),  .B3         (16'h0080),
        .B4         (16'h0080),  .B5         (16'h0080),  .B6         (16'h0080),  .B7         (16'h0080),
        .B8         (16'h0080),  .B9         (16'h0080),  .B10        (16'h0080), .B11        (16'h0080),
        .B12        (16'h0080), .B13        (16'h0080), .B14        (16'h0080), .B15        (16'h0080),
        .done       (),
        .accum      (),
        .uart_tx    (uart_tx)
    );

    // ---------------------------------------------------------------------------
    // Clock generation (simulate 67.16 MHz CFGMCLK)
    // ---------------------------------------------------------------------------
    initial begin
        clk = 0;
        forever #7.458 clk = ~clk;  // 67.16 MHz => 1/67.16e6 = 14.916 ns period
    end

    // ---------------------------------------------------------------------------
    // Test stimulus
    // ---------------------------------------------------------------------------
    initial begin
        // Initialize
        rst_n = 0;
        start = 0;

        // Wait for reset
        #100;
        rst_n = 1;

        // Pulse start
        #100;
        start = 1;
        #10;
        start = 0;

        // Wait for computation + UART transmission
        // Each UART byte at 115200 baud takes ~87us
        #200000;

        $finish;
    end

    // ---------------------------------------------------------------------------
    // Monitor and display
    // ---------------------------------------------------------------------------
    initial begin
        $monitor("Time=%0t: accum=%h", $time, u_top.accum);
    end

    initial begin
        $dumpfile("test_m3.vcd");
        $dumpvars(0, test_m3);
    end
endmodule
