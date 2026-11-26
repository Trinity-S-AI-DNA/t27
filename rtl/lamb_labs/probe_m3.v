`timescale 1ns/1ps
// Temporary probe: check Icarus SV feature support + DUT drive conflict semantics.
module probe_m3;
    reg clk, rst_n, start;
    wire [31:0] accum;
    wire done;
    wire uart_tx;

    reg [15:0] A0r, A1r;
    reg [15:0] B0r, B1r;

    lamb_labs_top u_top (
        .clk(clk), .rst_n(rst_n), .start(start),
        .A0(A0r), .A1(A1r), .A2(16'h0080), .A3(16'h0080),
        .A4(16'h0080), .A5(16'h0080), .A6(16'h0080), .A7(16'h0080),
        .A8(16'h0080), .A9(16'h0080), .A10(16'h0080), .A11(16'h0080),
        .A12(16'h0080), .A13(16'h0080), .A14(16'h0080), .A15(16'h0080),
        .B0(B0r), .B1(B1r), .B2(16'h0080), .B3(16'h0080),
        .B4(16'h0080), .B5(16'h0080), .B6(16'h0080), .B7(16'h0080),
        .B8(16'h0080), .B9(16'h0080), .B10(16'h0080), .B11(16'h0080),
        .B12(16'h0080), .B13(16'h0080), .B14(16'h0080), .B15(16'h0080),
        .done(done), .accum(accum), .uart_tx(uart_tx)
    );

    initial begin clk = 0; forever #7.458 clk = ~clk; end

    initial begin
        rst_n = 0; start = 0; A0r = 16'h1234; A1r = 16'h5678;
        B0r = 16'hAAAA; B1r = 16'hBBBB;
        #50;
        $display("[PROBE] TB drives A0=%h A1=%h B0=%h B1=%h", A0r, A1r, B0r, B1r);
        $display("[PROBE] DUT port view: A0=%h A1=%h B0=%h B1=%h", u_top.A0, u_top.A1, u_top.B0, u_top.B1);
        $display("[PROBE] DUT internal a_vec[0]=%h a_vec[1]=%h", u_top.a_vec[0], u_top.a_vec[1]);
        $display("[PROBE] u_bram_a.dout0=%h dout1=%h", u_top.u_bram_a.dout0, u_top.u_bram_a.dout1);
        rst_n = 1;
        #50;
        $display("[PROBE] after rst_n=1: DUT port A0=%h A1=%h", u_top.A0, u_top.A1);

        // Probe force/release on BRAM output regs
        force u_top.u_bram_a.dout0 = 16'h1111;
        force u_top.u_bram_b.dout0 = 16'h2222;
        #50;
        $display("[PROBE] forced: u_bram_a.dout0=%h u_bram_b.dout0=%h", u_top.u_bram_a.dout0, u_top.u_bram_b.dout0);
        $display("[PROBE] forced: a_vec[0]=%h b_vec[0]=%h mac_A0=%h", u_top.a_vec[0], u_top.b_vec[0], u_top.u_mac16.A0);
        release u_top.u_bram_a.dout0;
        release u_top.u_bram_b.dout0;
        #50;
        $display("[PROBE] released: u_bram_a.dout0=%h", u_top.u_bram_a.dout0);
        $display("[PROBE] DUT state=%d cycle_count=%d bram_en=%b", u_top.state, u_top.cycle_count, u_top.bram_en);
        $finish;
    end
endmodule
