// AX7203 board top of the generated WASM cell fpga_wasm_board (specs/fpga/wasm_board_ax7203.t27).
// Issue #12456, label owner-approved-foreign: the only hand-written Verilog of the WASM build;
// every decision lives in the spec. Pins: gf16_clean_ax7203.xdc of gHashTag/trinity-fpga
// (rst_n T6, led B13 C13 D14 D15, uart_rx P20, uart_tx N15). No clock pin: STARTUPE2 CFGMCLK.
module wasm_board_ax7203_top(input wire rst_n, output wire [3:0] led, input wire uart_rx, output wire uart_tx);
    wire cfgmclk, clk, ready;
    wire [7:0] txd, responding;
    STARTUPE2 #(.PROG_USR("FALSE"), .SIM_CCLK_FREQ(10.0)) startup (
        .CFGCLK(), .CFGMCLK(cfgmclk), .EOS(), .PREQ(),
        .CLK(1'b0), .GSR(1'b0), .GTS(1'b0), .KEYCLEARB(1'b0),
        .PACK(1'b0), .USRCCLKO(1'b0), .USRCCLKTS(1'b0),
        .USRDONEO(1'b1), .USRDONETS(1'b1));
    BUFG bufg_i (.I(cfgmclk), .O(clk));

    // reset: held low 16 clocks after configuration and while the button is pressed
    reg [3:0] rstc = 4'd0;
    reg rst_q0 = 1'b0, rst_q1 = 1'b0;
    always @(posedge clk) begin
        if (rstc != 4'hF) rstc <= rstc + 4'd1;
        rst_q0 <= rst_n;
        rst_q1 <= rst_q0;
    end
    wire rst_sync = (rstc == 4'hF) & rst_q1;

    // UART receive pin through two flops, idle high
    reg rx_q0 = 1'b1, rx_q1 = 1'b1;
    always @(posedge clk) begin rx_q0 <= uart_rx; rx_q1 <= rx_q0; end
    assign uart_tx = txd[0];

    fpga_wasm_board cell_i (.clk(clk), .rst_n(rst_sync), .en(1'b1), .rxd({7'b0, rx_q1}),
                            .ready(ready), .txd(txd), .responding(responding));
    // LED polarity is informational; the UART answer is the receipt.
    assign led = {responding[1:0], ready, rst_sync};
endmodule
