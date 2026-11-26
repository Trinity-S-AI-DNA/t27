// HDMI Test Pattern Integration for ALINX AX7203
// This module provides basic VGA/HDMI output capability for M3 HDMI Proof
// Generates test patterns: solid colors -> color bars -> moving patterns

`timescale 1ns / 1ps

module hdmi_test_integration (
    // Clock and reset
    input  wire        clk_200m,        // 200 MHz oscillator (R4/T4)
    input  wire        rst_n,          // Active low reset
    
    // WASM interface (clock domain crossing)
    input  wire        wasm_en,        // Enable from WASM core
    input  wire [1:0]  pattern_sel,    // Pattern selection from WASM
    
    // HDMI/VGA outputs
    output reg [7:0]  hdmi_red,       // Red output
    output reg [7:0]  hdmi_green,     // Green output  
    output reg [7:0]  hdmi_blue,      // Blue output
    output reg        hdmi_hsync,      // Horizontal sync
    output reg        hdmi_vsync,      // Vertical sync
    output reg        hdmi_de,        // Data enable
    
    // Status back to WASM core
    output reg [7:0]  hdmi_status,    // HDMI status (lock, sync, etc.)
    output reg [31:0] frame_count     // Frame counter
);

// Constants for 640x480 @ 60Hz (VGA timing)
localparam H_ACTIVE   = 640;
localparam H_FP       = 16;
localparam H_SYNC     = 96;
localparam H_BP       = 48;
localparam H_TOTAL    = H_ACTIVE + H_FP + H_SYNC + H_BP;

localparam V_ACTIVE   = 480;
localparam V_FP       = 10;
localparam V_SYNC     = 2;
localparam V_BP       = 33;
localparam V_TOTAL    = V_ACTIVE + V_FP + V_SYNC + V_BP;

// Internal registers
reg [9:0] h_count;
reg [9:0] v_count;
reg [9:0] pixel_x;
reg [9:0] pixel_y;

// Timing and sync signals
reg video_active;
reg h_sync_active;
reg v_sync_active;

// Clock divider (200MHz -> ~25MHz pixel clock)
reg [2:0] clk_div;

// Frame counter
reg [31:0] internal_frame_count;

// Status registers
reg mmcm_locked;
reg video_locked;

// Initialize on reset
always @(posedge clk_200m or negedge rst_n) begin
    if (!rst_n) begin
        // Reset all registers
        clk_div <= 3'b000;
        h_count <= 10'd0;
        v_count <= 10'd0;
        pixel_x <= 10'd0;
        pixel_y <= 10'd0;
        video_active <= 1'b0;
        h_sync_active <= 1'b0;
        v_sync_active <= 1'b0;
        mmcm_locked <= 1'b0;
        video_locked <= 1'b0;
        internal_frame_count <= 32'd0;
        hdmi_red <= 8'd0;
        hdmi_green <= 8'd0;
        hdmi_blue <= 8'd0;
        hdmi_hsync <= 1'b1;
        hdmi_vsync <= 1'b1;
        hdmi_de <= 1'b0;
        hdmi_status <= 8'd0;
        frame_count <= 32'd0;
    end else begin
        // Clock divider for 200MHz -> ~25MHz
        clk_div <= clk_div + 1'b1;
        
        // Horizontal counter
        if (clk_div == 3'b111) begin
            if (h_count == H_TOTAL - 1) begin
                h_count <= 10'd0;
                v_count <= v_count + 1'b1;
                
                // Vertical counter
                if (v_count == V_TOTAL - 1) begin
                    v_count <= 10'd0;
                    internal_frame_count <= internal_frame_count + 1'b1;
                end
            end else begin
                h_count <= h_count + 1'b1;
            end
            
            // Update timing signals
            h_sync_active <= (h_count >= H_ACTIVE + H_FP) && 
                             (h_count < H_ACTIVE + H_FP + H_SYNC);
            
            v_sync_active <= (v_count >= V_ACTIVE + V_FP) && 
                             (v_count < V_ACTIVE + V_FP + V_SYNC);
            
            video_active <= (h_count < H_ACTIVE) && (v_count < V_ACTIVE);
            
            // Calculate pixel position
            if (h_count < H_ACTIVE) begin
                pixel_x <= h_count;
            end else begin
                pixel_x <= 10'd0;
            end
            
            if (v_count < V_ACTIVE) begin
                pixel_y <= v_count;
            end else begin
                pixel_y <= 10'd0;
            end
        end
        
        // MMCM lock simulation (in real design would monitor actual MMCM)
        mmcm_locked <= 1'b1;
        
        // Video lock status
        video_locked <= (internal_frame_count > 10) && mmcm_locked;
        
        // Generate outputs
        hdmi_hsync <= h_sync_active;
        hdmi_vsync <= v_sync_active;
        
        // Data enable
        hdmi_de <= video_active && wasm_en && video_locked;
        
        // Generate test pattern
        if (video_active && wasm_en && video_locked) begin
            case (pattern_sel)
                2'b00: // Solid red
                    {hdmi_red, hdmi_green, hdmi_blue} <= {8'hff, 8'd0, 8'd0};
                2'b01: // Solid green  
                    {hdmi_red, hdmi_green, hdmi_blue} <= {8'd0, 8'hff, 8'd0};
                2'b10: // Solid blue
                    {hdmi_red, hdmi_green, hdmi_blue} <= {8'd0, 8'd0, 8'hff};
                2'b11: // Color bars
                    if (pixel_x < 160) {hdmi_red, hdmi_green, hdmi_blue} <= {8'hff, 8'd0, 8'd0};      // Red
                    else if (pixel_x < 320) {hdmi_red, hdmi_green, hdmi_blue} <= {8'd0, 8'hff, 8'd0};  // Green
                    else if (pixel_x < 480) {hdmi_red, hdmi_green, hdmi_blue} <= {8'd0, 8'd0, 8'hff};  // Blue
                    else if (pixel_x < 560) {hdmi_red, hdmi_green, hdmi_blue} <= {8'hff, 8'hff, 8'd0}; // Yellow
                    else {hdmi_red, hdmi_green, hdmi_blue} <= {8'hff, 8'd0, 8'hff}; // Magenta
            endcase
        end else begin
            // Blanking
            {hdmi_red, hdmi_green, hdmi_blue} <= {8'd0, 8'd0, 8'd0};
            hdmi_de <= 1'b0;
        end
        
        // Update status
        if (video_locked) begin
            hdmi_status <= 8'h03; // Lock + sync OK
        end else begin
            hdmi_status <= 8'h01; // Lock only
        end
        
        // Update frame count output
        frame_count <= internal_frame_count;
    end
end

endmodule