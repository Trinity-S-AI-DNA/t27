`default_nettype none
module result_bram (
    input  wire        clk,
    input  wire        rst_n,
    input  wire        en,
    input  wire [3:0]  addr,
    input  wire [31:0] din,
    output reg [31:0]  dout
);

    always @(posedge clk or negedge rst_n) begin
        if (!rst_n) begin
            dout <= 32'd0;
        end else if (en) begin
            dout <= din;
        end
    end

    // Preload zero
    initial begin
        dout <= 32'd0;
    end

endmodule