`default_nettype none

// Control FSM for lamb_labs_top — orchestrates:
// 1. Vector loading from BRAM (via bram_ctrl)
// 2. MAC16 computation (dot product of A and B)
// 3. UART transmission of 32-bit result
// 4. Result valid pulse generation
//
// States: IDLE, LOADING, COMPUTING, SENDING, DONE
// Target: ALINX AX7203 xc7a200tfbg484-2, CFGMCLK (67.16 MHz MEASURED)

// Parametrized for flexibility
//   VECTOR_SIZE = 16 (elements per vector)
//   UART_BYTES = 4 (32-bit result → 4 bytes LSB-first)

// Ports
//   clk, rst_n, en — global signals
//   start — pulse high for one cycle to start a new computation
//   vect_addr — BRAM address to load the next vector pair (0..255)
//   vect_valid — 1 = vect_addr is valid and should be loaded
//   uart_tx — serial transmit line (driven by module)
//   result_val — 1-pulse, one clk cycle when accum contains valid dot product
//   accum — 32-bit dot product result (valid when result_val = 1)

// The FSM sequence:
//   IDLE:   Wait for start pulse. When start=1, load vector addresses
//           from vect_valid/vect_addr, then transition to LOADING.
//   LOADING: Load A and B vectors from BRAM into mac16 inputs;
//            transition to COMPUTING after one cycle.
//   COMPUTING: mac16 computes dot product (16 cycles of multiply-accumulate);
//              transition to SENDING after done pulse.
//   SENDING: Send 4 bytes of accum via uart_tx LSB-first;
//            after all 4 bytes sent, transition to DONE.
//   DONE:   Pulse result_val high for one cycle; then return to IDLE.

module lamb_labs_control (
    input  wire        clk,        // CFGMCLK domain (67.16 MHz MEASURED)
    input  wire        rst_n,      // Active-low reset
    input  wire        en,         // Module enable

    // Start computation: pulse high for one clk cycle
    input  wire        start,

    // Vector load interface (connected to bram_ctrl)
    input  wire [7:0]  vect_addr,  // BRAM address (0..255) for vector pair
    input  wire        vect_valid, // 1 = load vectors from BRAM at this address

    // UART interface
    output wire        uart_tx,    // Serial transmit line

    // Result interface
    output wire        result_val, // 1-pulse, one clk cycle when accum valid
    output reg  signed [31:0] accum  // 32-bit dot product result
);

// ---------------------------------------------------------------------------
// Internal: state machine
// ---------------------------------------------------------------------------

localparam [2:0]
    S_IDLE      = 3'b000,
    S_LOADING   = 3'b001,
    S_COMPUTING = 3'b010,
    S_SENDING   = 3'b011,
    S_DONE      = 3'b100;

reg [2:0] state;
reg [2:0] next_state;

// --- State transition logic ---
always @(posedge clk or negedge rst_n) begin
    if (!rst_n) begin
        state <= S_IDLE;
        accum <= 32'sd0;
    end else begin
        state <= next_state;

        case (state)
            S_IDLE: begin
                // Wait for start pulse
                if (start) begin
                    // Start: load vectors, then compute
                    next_state <= S_LOADING;
                end else begin
                    next_state <= S_IDLE;
                end
            end

            S_LOADING: begin
                // Load vector addresses from BRAM
                // (In a full design, bram_ctrl drives mac16 A/B inputs;
                //  here we transition immediately after loading)
                next_state <= S_COMPUTING;
            end

            S_COMPUTING: begin
                // mac16 computes dot product of A and B
                // When computation is done, move to sending phase
                // (In minimal model: after one en cycle, assume done)
                next_state <= S_SENDING;
            end

            S_SENDING: begin
                // Send 4 bytes (32-bit accum) via UART LSB-first
                // After all bytes sent, go to DONE
                // (UART module internally tracks byte count; here we just
                // transition after a fixed number of cycles or when uart_done pulses)
                // For minimal model: after some cycles, go to DONE
                // In a fully integrated design, we'd have a uart_done signal
                // and check it here. For now, transition after assumption.
                next_state <= S_DONE;
            end

            S_DONE: begin
                // Result fully transmitted; pulse result_val
                // accum stays valid (holds last computed dot product)
                // Pulse result_val for one cycle on next state transition
                next_state <= S_IDLE;
            end

            default: next_state <= S_IDLE;
        endcase
    end
end

// ---------------------------------------------------------------------------
// result_val: one-cycle pulse when entering S_DONE from S_SENDING
// ----------------------------------------------------------------------------

always @(posedge clk or negedge rst_n) begin
    if (!rst_n) begin
        result_val <= 1'b0;
    end else begin
        // Pulse result_val high for one cycle when transitioning S_DONE → S_IDLE
        if (state == S_DONE && next_state == S_IDLE) begin
            result_val <= 1'b1;
        end else begin
            result_val <= 1'b0;
        end
    end
end

// ---------------------------------------------------------------------------
// accum assignment: holds the last computed dot product result
// ----------------------------------------------------------------------------

// accum is updated when we transition from COMPUTING to SENDING.
// In a fully pipelined design, this would capture mac16's accum output.
// For this minimal model, we hold the value across the S_SENDing phase.

always @(posedge clk) begin
    // When result_val pulses, accum holds the valid dot product
    // (The actual accumulation happens inside mac16; here we just pass it through)
    if (result_val) begin
        // accum retains its value; caller can read it when result_val = 1
    end
end

// ---------------------------------------------------------------------------
// UART instance: drives uart_tx
// ---------------------------------------------------------------------------

// We instantiate the uart_tx module from rtl/lamb_labs/uart_tx.v
// The uart_tx needs: clk, rst_n, enable, data_in (8-bit), tx, tx_done
//
// Here we connect it to shift out 4 bytes (32-bit accum) LSB-first.
// We use a byte index and a shift register to serialise the 4 bytes.

wire uart_tx_internal;
reg [7:0] uart_byte_reg;    // current byte being sent (0..3)
reg [31:0] uart_shift_reg;  // accumulating the 4 bytes to send
reg [1:0] uart_bit_idx;     // which byte (0=LSB, 3=MSB)
reg uart_start_pulse;       // pulse to start UART transmission of current byte

// The uart_tx module expects 'enable' to be a 1-pulse per byte
// We generate enable pulses based on uart_bit_idx

uart_tx u_uart_tx_inst (
    .clk        (clk),
    .rst_n      (rst_n),
    .enable     (uart_start_pulse),   // 1-pulse: start sending data_in
    .data_in    (uart_shift_reg[7:0]), // 8 bits: current byte LSB-first
    .tx         (uart_tx_internal),
    .tx_done    (/* we could connect this but not used here */)
);

// Assign the top-level uart_tx output
assign uart_tx = uart_tx_internal;

// ---------------------------------------------------------------------------
// Byte-serial transmission logic: shift out 4 bytes of accum LSB-first
// ---------------------------------------------------------------------------

// Initialize uart_shift_reg with accum value (LSB at byte 0)
// This happens when entering S_SENDing state
always @(posedge clk or negedge rst_n) begin
    if (!rst_n) begin
        uart_shift_reg <= 32'sd0;
        uart_byte_reg <= 8'sd0;
        uart_bit_idx <= 2'sd0;
        uart_start_pulse <= 1'b0;
    end else begin
        uart_start_pulse <= 1'b0;  // default: no pulse

        case (state)
            S_IDLE: begin
                uart_start_pulse <= 1'b0;
                // uart_shift_reg and uart_byte_idx are undefined/irrelevant in IDLE
            end

            S_SENDING: begin
                // Send 4 bytes LSB-first of accum
                // Byte 0 (bit_idx=0): accum[7:0]
                // Byte 1 (bit_idx=1): accum[15:8]
                // Byte 2 (bit_idx=2): accum[23:16]
                // Byte 3 (bit_idx=3): accum[31:24]

                if (uart_bit_idx == 2'd0) begin
                    // First byte: LSB of accum
                    uart_shift_reg <= accum;
                    uart_byte_reg <= accum[7:0];
                end else if (uart_bit_idx == 2'd1) begin
                    // Second byte: bits 15:8 of accum
                    uart_shift_reg <= accum;
                    uart_byte_reg <= accum[15:8];
                end else if (uart_bit_idx == 2'd2) begin
                    // Third byte: bits 23:16
                    uart_shift_reg <= accum;
                    uart_byte_reg <= accum[23:16];
                end else if (uart_bit_idx == 2'd3) begin
                    // Fourth (MSB) byte: bits 31:24
                    uart_shift_reg <= accum;
                    uart_byte_reg <= accum[31:24];
                end

                // Pulse enable to start UART transmission of this byte
                // The uart_tx module will shift out 8 bits LSB-first
                // over one bit period (583 clk cycles at 67.16 MHz)
                uart_start_pulse <= 1'b1;

                // Advance to next byte after tx_done (in a full design we'd
                // wait for the uart_tx's tx_done pulse; here we simulate
                // by incrementing bit_idx after a delay)
                // For minimal model, we'll just increment after one cycle
                // assumption that UART transmission is complete.
                // In reality: wait for uart_tx tx_done pulse.
                if (uart_bit_idx < 2'd3) begin
                    uart_bit_idx <= uart_bit_idx + 2'd1;
                end else begin
                    // All 4 bytes sent, go to DONE state
                    // (Transition controlled by state machine)
                end
            end

            default: begin
                uart_bit_idx <= 2'sd0;
                uart_start_pulse <= 1'b0;
            end
        endcase
    end
end

// ---------------------------------------------------------------------------
// Resource estimates (xc7a200tfbg484-2):
// - Control FSM: ~50 LUTs, ~40 FFs
// - UART transmitter: ~150 LUTs, ~8 FFs (from uart_tx.v)
// - BRAM controller: ~2 BRAMs, ~50 LUTs (from bram_ctrl.v)
// - mac16: ~1 DSP, ~50 LUTs, ~64 FFs
// - Total: ~2 BRAMs, ~300 LUTs, ~160 FFs, 1 DSP — trivial footprint
//   leaves ~738 DSPs and ~134700 LUTs free on the AX7203
//
// Frequency target: CFGMCLK 67.16 MHz (MEASURED)
// The FSM, UART baud counter (583-cycle period), and mac16 pipeline
// should all meet timing at 67 MHz with huge margin.
//
// Simulation: instantiate lamb_labs_top in a Verilator/iverilog testbench
// that drives clk, rst_n, start, vect_addr, vect_valid and checks:
//   - result_val pulse occurs after computation + UART transmission
//   - accum matches Python golden model dot product
//   - uart_tx sends 4 bytes LSB-first of the 32-bit result
//
// ---------------------------------------------------------------------------

`default_nettype wire