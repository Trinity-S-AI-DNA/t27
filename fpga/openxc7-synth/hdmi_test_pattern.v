// Minimal HDMI/VGA Test Pattern Generator for ALINX AX7203
// Generates simple color bars and solid color patterns
// For testing pixel clock, timing, and video output path

`timescale 1ns / 1ps

module hdmi_test_pattern (
    // Clock inputs
    input  wire        clk_200m,        // 200 MHz oscillator from AX7203 (R4/T4)
    input  wire        rst_n,          // Active low reset
    
    // HDMI output pins (simplified VGA-style)
    output reg  [7:0]  red,            // Red output
    output reg  [7:0]  green,          // Green output
    output reg  [7:0]  blue,           // Blue output
    output reg         hsync,          // Horizontal sync
    output reg         vsync,          // Vertical sync
    output reg         de,             // Data enable
    
    // Test pattern control
    input  wire [1:0]  pattern_sel,    // 00=red, 01=green, 10=blue, 11=color bars
    input  wire        en              // Enable video output
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

// Pixel and frame counters
reg [9:0] h_count;
reg [9:0] v_count;
reg [9:0] pixel_x;
reg [9:0] pixel_y;

// Timing state machine
reg video_active;
reg h_sync_active;
reg v_sync_active;

// Clock divider for testing (200MHz -> 25MHz pixel clock approx)
reg [1:0] clk_div;

// Synchronize clocks
always @(posedge clk_200m or negedge rst_n) begin
    if (!rst_n) begin
        clk_div <= 2'b00;
        h_count <= 10'd0;
        v_count <= 10'd0;
        video_active <= 1'b0;
        h_sync_active <= 1'b0;
        v_sync_active <= 1'b0;
    end else begin
        // Simple clock divider (200MHz / 8 = 25MHz approx)
        clk_div <= clk_div + 1'b1;
        
        // Horizontal counter
        if (clk_div == 2'b11) begin
            if (h_count == H_TOTAL - 1) begin
                h_count <= 10'd0;
                v_count <= v_count + 1'b1;
                
                // Vertical counter
                if (v_count == V_TOTAL - 1) begin
                    v_count <= 10'd0;
                end
            end else begin
                h_count <= h_count + 1'b1;
            end
            
            // Horizontal sync timing
            h_sync_active <= (h_count >= H_ACTIVE + H_FP) && 
                             (h_count < H_ACTIVE + H_FP + H_SYNC);
            
            // Vertical sync timing
            v_sync_active <= (v_count >= V_ACTIVE + V_FP) && 
                             (v_count < V_ACTIVE + V_FP + V_SYNC);
            
            // Video active region
            video_active <= (h_count < H_ACTIVE) && (v_count < V_ACTIVE);
            
            // Calculate pixel position
            pixel_x <= (h_count < H_ACTIVE) ? h_count : 10'd0;
            pixel_y <= (v_count < V_ACTIVE) ? v_count : 10'd0;
        end
    end
end

// Generate outputs
always @(posedge clk_200m or negedge rst_n) begin
    if (!rst_n) begin
        red <= 8'd0;
        green <= 8'd0;
        blue <= 8'd0;
        hsync <= 1'b1;
        vsync <= 1'b1;
        de <= 1'b0;
    end else begin
        // Horizontal sync
        hsync <= h_sync_active;
        
        // Vertical sync
        vsync <= v_sync_active;
        
        // Data enable
        de <= video_active && en;
        
        // Generate test pattern
        if (video_active && en) begin
            case (pattern_sel)
                2'b00: // Solid red
                    {red, green, blue} <= {8'hff, 8'd0, 8'd0};
                2'b01: // Solid green  
                    {red, green, blue} <= {8'd0, 8'hff, 8'd0};
                2'b10: // Solid blue
                    {red, green, blue} <= {8'd0, 8'd0, 8'hff};
                2'b11: // Color bars
                    if (pixel_x < 160) {red, green, blue} <= {8'hff, 8'd0, 8'd0};      // Red
                    else if (pixel_x < 320) {red, green, blue} <= {8'd0, 8'hff, 8'd0};  // Green
                    else if (pixel_x < 480) {red, green, blue} <= {8'd0, 8'd0, 8'hff};  // Blue
                    else if (pixel_x < 560) {red, green, blue} <= {8'hff, 8'hff, 8'd0}; // Yellow
                    else if (pixel_x < 640) {red, green, blue} <= {8'hff, 8'd0, 8'hff}; // Magenta
            endcase
        end else begin
            // Blanking
            {red, green, blue} <= {8'd0, 8'd0, 8'd0};
        end
    end
end

endmodule