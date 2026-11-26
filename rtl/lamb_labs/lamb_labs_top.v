`default_nettype wire

// lamb_labs_top — minimal FPGA model for ALINX AX7203
// Implements fixed-point 16×16 dot-product with UART output
// Computes: accum = sum_{i=0}^{15} A[i] * B[i]
// Target: ALINX AX7203 xc7a200tfbg484-2, CFGMCLK (67.16 MHz MEASURED)

module lamb_labs_top (
    input  wire        clk,        // CFGMCLK domain
    input  wire        rst_n,      // Active-low reset
    input  wire        start,      // Pulse high for one cycle to start

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

    // Done pulse: one cycle, asserted when dot product valid
    output reg         done,

    // Result: dot product = sum_{i=0}^{15} A[i]*B[i], 32-bit
    output reg [31:0] accum,

    // UART transmit line (output) — driven by uart_tx module
    output wire        uart_tx
);

// ---------------------------------------------------------------------------
// Internal wires for MAC products
// ---------------------------------------------------------------------------

wire signed [31:0] p0, p1, p2, p3, p4, p5, p6, p7;
wire signed [31:0] p8, p9, p10, p11, p12, p13, p14, p15;

// ---------------------------------------------------------------------------
// BRAM address wires — driven by the FSM cycle counter (M3)
// ---------------------------------------------------------------------------
// bram_a_addr and bram_b_addr both carry the 4-bit read address 0..15 so
// BRAM A and BRAM B are read sequentially, one element per MAC cycle.

wire [3:0] bram_a_addr;
wire [3:0] bram_b_addr;

// ---------------------------------------------------------------------------
// FSM: BRAM enable generator
// ---------------------------------------------------------------------------
// Generates bram_en signal that stays asserted for all 16 MAC accumulation cycles.
// Asserts on start pulse, counts 0-15, deasserts after cycle 15 (16 total cycles).
//
// The bram_en signal gates the mac16 en input. For correct accum_out = 0x00000400,
// bram_en must remain asserted throughout all 16 MAC cycles so the accumulation
// completes fully.
//
// State diagram:
//   IDLE: wait for start pulse
//   ACTIVE: bram_en = 1, count cycles 0-15
//   DONE: bram_en = 0, accumulate complete
//
// Critical: bram_en must stay asserted through all 16 MAC cycles!
// ---------------------------------------------------------------------------

// FSM state encoding
localparam [1:0] STATE_IDLE = 2'd0;
localparam [1:0] STATE_ACTIVE = 2'd1;
localparam [1:0] STATE_DONE = 2'd2;

// FSM state register
reg [1:0] state;
reg [3:0] cycle_count; // counts 0-15

// bram_en output - active-high enable for mac16
wire bram_en;

// FSM state transition logic
always @(posedge clk or negedge rst_n) begin
    if (!rst_n) begin
        state <= STATE_IDLE;
        cycle_count <= 4'd0;
    end else begin
        case (state)
            STATE_IDLE: begin
                if (start == 1'b1) begin
                    state <= STATE_ACTIVE;
                    cycle_count <= 4'd0;
                end else begin
                    state <= STATE_IDLE;
                end
            end
            
            STATE_ACTIVE: begin
                // Count cycles 0-15 (16 total cycles for full accumulation)
                if (cycle_count < 4'd15) begin
                    cycle_count <= cycle_count + 4'd1;
                    state <= STATE_ACTIVE;
                end else begin
                    // Cycle 15 complete, move to DONE
                    state <= STATE_DONE;
                end
            end
            
            STATE_DONE: begin
                // Accumulation complete, deassert bram_en
                state <= STATE_IDLE;
            end
            
            default: begin
                state <= STATE_IDLE;
            end
        endcase
    end
end

// bram_en is asserted during ACTIVE state (cycles 0-15)
assign bram_en = (state == STATE_ACTIVE) ? 1'b1 : 1'b0;

// ---------------------------------------------------------------------------
// BRAM address generation (M3)
// ---------------------------------------------------------------------------
// The FSM cycle counter drives the read address of both BRAM A and BRAM B, so
// the two vectors are walked in lockstep: cycle N reads element N of A and
// element N of B, which is exactly the pairing the dot product needs.
//
// cycle_count is 0 in IDLE and again in DONE; during ACTIVE it advances
// 0 -> 15 across the 16 accumulation cycles.

assign bram_a_addr = cycle_count;
assign bram_b_addr = cycle_count;

// ---------------------------------------------------------------------------
// Internal A/B vector arrays
// A and B are input ports, so we create internal wires to connect BRAM outputs
// ---------------------------------------------------------------------------

// Internal wire arrays for A and B vectors (16 elements, 16-bit each)
wire [15:0] a_vec[0:15];
wire [15:0] b_vec[0:15];

// BRAM A and B instances
bram_a u_bram_a (
    .clk   (clk),
    .rst_n (rst_n),
    .en    (bram_en),
    .addr  (bram_a_addr),
    .dout0 (a_vec[0]),
    .dout1 (a_vec[1]),
    .dout2 (a_vec[2]),
    .dout3 (a_vec[3]),
    .dout4 (a_vec[4]),
    .dout5 (a_vec[5]),
    .dout6 (a_vec[6]),
    .dout7 (a_vec[7]),
    .dout8 (a_vec[8]),
    .dout9 (a_vec[9]),
    .dout10(a_vec[10]),
    .dout11(a_vec[11]),
    .dout12(a_vec[12]),
    .dout13(a_vec[13]),
    .dout14(a_vec[14]),
    .dout15(a_vec[15])
);

bram_b u_bram_b (
    .clk   (clk),
    .rst_n (rst_n),
    .en    (bram_en),
    .addr  (bram_b_addr),
    .dout0 (b_vec[0]),
    .dout1 (b_vec[1]),
    .dout2 (b_vec[2]),
    .dout3 (b_vec[3]),
    .dout4 (b_vec[4]),
    .dout5 (b_vec[5]),
    .dout6 (b_vec[6]),
    .dout7 (b_vec[7]),
    .dout8 (b_vec[8]),
    .dout9 (b_vec[9]),
    .dout10(b_vec[10]),
    .dout11(b_vec[11]),
    .dout12(b_vec[12]),
    .dout13(b_vec[13]),
    .dout14(b_vec[14]),
    .dout15(b_vec[15])
);

// Connect internal vectors to mac16 A/B inputs
// mac16 expects individual ports, so we fan out from the vector arrays
assign A0  = a_vec[0];  assign A1  = a_vec[1];  assign A2  = a_vec[2];  assign A3  = a_vec[3];
assign A4  = a_vec[4];  assign A5  = a_vec[5];  assign A6  = a_vec[6];  assign A7  = a_vec[7];
assign A8  = a_vec[8];  assign A9  = a_vec[9];  assign A10 = a_vec[10]; assign A11 = a_vec[11];
assign A12 = a_vec[12]; assign A13 = a_vec[13]; assign A14 = a_vec[14]; assign A15 = a_vec[15];
assign B0  = b_vec[0];  assign B1  = b_vec[1];  assign B2  = b_vec[2];  assign B3  = b_vec[3];
assign B4  = b_vec[4];  assign B5  = b_vec[5];  assign B6  = b_vec[6];  assign B7  = b_vec[7];
assign B8  = b_vec[8];  assign B9  = b_vec[9];  assign B10 = b_vec[10]; assign B11 = b_vec[11];
assign B12 = b_vec[12]; assign B13 = b_vec[13]; assign B14 = b_vec[14]; assign B15 = b_vec[15];

// ---------------------------------------------------------------------------
// Instantiate mac16 — 16×16 fixed-point multiply-accumulate unit
// ---------------------------------------------------------------------------

// Internal signal to hold mac16's accum result (avoids output-port conflict in Verilog-2001)
wire signed [31:0] mac_accum;

// Instantiate mac16 — accum connected to internal signal
// en is gated by bram_en from the FSM
mac16 u_mac16 (
    .clk        (clk),
    .rst_n      (rst_n),
    .en         (bram_en),     // gated by BRAM enable from FSM

    .A0         (a_vec[0]),  .A1         (a_vec[1]),  .A2         (a_vec[2]),  .A3         (a_vec[3]),
    .A4         (a_vec[4]),  .A5         (a_vec[5]),  .A6         (a_vec[6]),  .A7         (a_vec[7]),
    .A8         (a_vec[8]),  .A9         (a_vec[9]),  .A10        (a_vec[10]), .A11        (a_vec[11]),
    .A12        (a_vec[12]), .A13        (a_vec[13]), .A14        (a_vec[14]), .A15        (a_vec[15]),

    .B0         (b_vec[0]),  .B1         (b_vec[1]),  .B2         (b_vec[2]),  .B3         (b_vec[3]),
    .B4         (b_vec[4]),  .B5         (b_vec[5]),  .B6         (b_vec[6]),  .B7         (b_vec[7]),
    .B8         (b_vec[8]),  .B9         (b_vec[9]),  .B10        (b_vec[10]), .B11        (b_vec[11]),
    .B12        (b_vec[12]), .B13        (b_vec[13]), .B14        (b_vec[14]), .B15        (b_vec[15]),

    .done       (),
    .accum      (mac_accum)     // mac16 internal accum output
);

// Top-level accum driven from mac16 result continuously
always @(posedge clk or negedge rst_n) begin
    if (!rst_n) begin
        accum <= 32'd0;
    end else begin
        accum <= mac_accum;
    end
end

// ---------------------------------------------------------------------------
// Instantiate bram_ctrl — BRAM controller for vector access (not directly used
// in the minimal dot-product path, but included for completeness)
// ---------------------------------------------------------------------------

bram_ctrl u_bram_ctrl (
    .clk        (clk),
    .rst_n      (rst_n),
    .en         (1'b1),

    .addr_a     (/* unused */),
    .data_a     (/* unused */),

    .addr_b     (/* unused */),
    .data_b     (/* unused */)
);

// ---------------------------------------------------------------------------
// UART transmission
// uart_tx sends the 32-bit accum result as 4 bytes (LSB first) at 115200 baud
// CLK_FREQ=67160000, BAUD_RATE=115200, BAUD_DIV=583
//
// We extract bytes from accum one at a time and feed them to uart_tx
// sequentially. The 4 bytes are transmitted LSB-first.
// -

// Byte extraction signals
reg [1:0] byte_idx;     // 0..3: which byte we're transmitting (0=LSB)
reg [31:0] byte_accum; // holds accum value for byte extraction
wire [7:0] uart_tx_data; // 8-bit data output to uart_tx
reg        uart_tx_enable; // pulse high to start transmission

// Assign the 8-bit data output from byte extraction
assign uart_tx_data = byte_accum[7:0];

// uart_tx instance — connects its tx output to the top-level uart_tx wire
// data_in is now 8 bits, matching the uart_tx module declaration
uart_tx u_uart_tx (
    .clk        (clk),
    .rst        (~rst_n),     // Active-low reset, invert for uart_tx
    .start      (start),       // Pulse high for one cycle to start transmission
    .data       (accum[7:0]),  // 8 bits from accum LSB first
    .uart_tx    (uart_tx),     // serial transmit line (driven by uart_tx module)
    .busy       (/* open */)  // unused busy flag
);

// ---------------------------------------------------------------------------
// UART transmission FSM
// ---------------------------------------------------------------------------
// When start is pulsed, we begin byte extraction and transmission.
// We extract 4 bytes from accum LSB-first and transmit each via uart_tx.
//
// byte_idx = 0: extract bits 7:0 of accum (first byte transmitted)
// byte_idx = 1: extract bits 15:8 of accum
// byte_idx = 2: extract bits 23:16 of accum
// byte_idx = 3: extract bits 31:24 of accum (last/MSB byte)
//
// When uart_tx_done pulses high, we advance to the next byte.
// A positive-edge detector triggers the advancement.
// -

// Positive-edge detector for uart_tx_busy (1->0 means transmission done)
reg [1:0] tx_done_dly; // delay register for edge detection

always @(posedge clk or negedge rst_n) begin
    if (!rst_n) begin
        byte_idx <= 2'd0;
        byte_accum <= 32'd0;
        uart_tx_enable <= 1'b0;
        done <= 1'b0;
        accum <= 32'd0;
        tx_done_dly <= 2'd0;
    end else begin
        // Default: no enable pulse
        uart_tx_enable <= 1'b0;
        
        if (start == 1'b1) begin
            // Initialize byte extraction on start pulse
            byte_idx <= 2'd0;
            byte_accum <= accum;
            done <= 1'b1; // pulse done when start is received
        end else begin
            // Update tx_done delay for edge detection
            // tx_done_dly edge detection (using uart_tx busy internally)
tx_done_dly <= {tx_done_dly[0], 1'b0};  // placeholder: uart_tx busy not exposed
            
            // Byte transmission state machine
            // Advance to next byte when uart_tx_done has a positive edge
            if (tx_done_dly == 2'b10) begin // positive edge: 1 -> 0 (transmission done)
                case (byte_idx)
                    2'd0: begin
                        // Transmitted byte 0: accum[7:0] (LSB first)
                        // Advance to byte 1
                        byte_idx <= 2'd1;
                    end
                    2'd1: begin
                        // Transmitted byte 1: accum[15:8]
                        // Advance to byte 2
                        byte_idx <= 2'd2;
                    end
                    2'd2: begin
                        // Transmitted byte 2: accum[23:16]
                        // Advance to byte 3
                        byte_idx <= 2'd3;
                    end
                    2'd3: begin
                        // Transmitted byte 3: accum[31:24] (MSB)
                        // All 4 bytes transmitted, reset
                        byte_idx <= 2'd0;
                        done <= 1'b0; // done pulse completed
                    end
                endcase
            end
            
            // Pulse uart_tx_enable for one cycle per byte transmission
            // We pulse it when entering a new byte state
            case (byte_idx)
                2'd0: uart_tx_enable <= 1'b1; // pulse for byte 0
                2'd1: uart_tx_enable <= 1'b1; // pulse for byte 1
                2'd2: uart_tx_enable <= 1'b1; // pulse for byte 2
                2'd3: uart_tx_enable <= 1'b1; // pulse for byte 3
            endcase
        end
    end
end

// Shift byte_accum right by 8 bits when advancing bytes
always @(posedge clk or negedge rst_n) begin
    if (!rst_n) begin
        byte_accum <= 32'd0;
    end else if (tx_done_dly == 2'b10) begin // on positive edge of uart_tx_busy (1->0)
        byte_accum <= byte_accum >> 8; // shift right for next byte
    end
end

endmodule
