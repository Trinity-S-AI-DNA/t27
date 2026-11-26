# ALINX AX7203 HDMI/VGA Test Constraints
# Defines pinout for HDMI test pattern generation
# Target: xc7a200tfbg484-2

# Clock inputs
set_property PACKAGE_PIN R4 [get_ports clk_200m]
set_property IOSTANDARD LVCMOS15 [get_ports clk_200m]

set_property PACKAGE_PIN B3 [get_ports rst_n]
set_property IOSTANDARD LVCMOS33 [get_ports rst_n]

# UART interface
set_property PACKAGE_PIN P20 [get_ports uart_rxd]
set_property IOSTANDARD LVCMOS33 [get_ports uart_rxd]

set_property PACKAGE_PIN N15 [get_ports uart_txd]
set_property IOSTANDARD LVCMOS33 [get_ports uart_txd]

# HDMI/VGA outputs (simplified VGA-style, may need level shifters for HDMI)
set_property PACKAGE_PIN E18 [get_ports {hdmi_red[0]}]
set_property PACKAGE_PIN F19 [get_ports {hdmi_red[1]}]
set_property PACKAGE_PIN E19 [get_ports {hdmi_red[2]}]
set_property PACKAGE_PIN D19 [get_ports {hdmi_red[3]}]
set_property PACKAGE_PIN C17 [get_ports {hdmi_red[4]}]
set_property PACKAGE_PIN B20 [get_ports {hdmi_red[5]}]
set_property PACKAGE_PIN A20 [get_ports {hdmi_red[6]}]
set_property PACKAGE_PIN A19 [get_ports {hdmi_red[7]}]

set_property PACKAGE_PIN C20 [get_ports {hdmi_green[0]}]
set_property PACKAGE_PIN D20 [get_ports {hdmi_green[1]}]
set_property PACKAGE_PIN A18 [get_ports {hdmi_green[2]}]
set_property PACKAGE_PIN A16 [get_ports {hdmi_green[3]}]
set_property PACKAGE_PIN B18 [get_ports {hdmi_green[4]}]
set_property PACKAGE_PIN C18 [get_ports {hdmi_green[5]}]
set_property PACKAGE_PIN C19 [get_ports {hdmi_green[6]}]
set_property PACKAGE_PIN D18 [get_ports {hdmi_green[7]})

set_property PACKAGE_PIN F18 [get_ports {hdmi_blue[0]}]
set_property PACKAGE_PIN G18 [get_ports {hdmi_blue[1]}]
set_property PACKAGE_PIN H18 [get_ports {hdmi_blue[2]}]
set_property PACKAGE_PIN G19 [get_ports {hdmi_blue[3]}]
set_property PACKAGE_PIN F19 [get_ports {hdmi_blue[4]}]
set_property PACKAGE_PIN E18 [get_ports {hdmi_blue[5]}]
set_property PACKAGE_PIN D19 [get_ports {hdmi_blue[6]}]
set_property PACKAGE_PIN C19 [get_ports {hdmi_blue[7]}]

# Sync signals
set_property PACKAGE_PIN T10 [get_ports hdmi_hsync]
set_property IOSTANDARD LVCMOS33 [get_ports hdmi_hsync]

set_property PACKAGE_PIN U10 [get_ports hdmi_vsync]
set_property IOSTANDARD LVCMOS33 [get_ports hdmi_vsync]

set_property PACKAGE_PIN V10 [get_ports hdmi_de]
set_property IOSTANDARD LVCMOS33 [get_ports hdmi_de]

# Configuration signals
set_property PACKAGE_PIN K17 [get_ports {hdmi_status[0]}]
set_property PACKAGE_PIN K18 [get_ports {hdmi_status[1]}]
set_property PACKAGE_PIN L18 [get_ports {hdmi_status[2]}]
set_property PACKAGE_PIN T11 [get_ports {hdmi_status[3]}]
set_property PACKAGE_PIN T12 [get_ports {hdmi_status[4]}]
set_property PACKAGE_PIN U12 [get_ports {hdmi_status[5]}]
set_property PACKAGE_PIN V12 [get_ports {hdmi_status[6]}]
set_property PACKAGE_PIN W12 [get_ports {hdmi_status[7]}]

# Clock constraints
create_clock -name clk_200m -period 5.0 [get_ports clk_200m]

# Set drive strength for outputs
set_property DRIVE 12 [get_ports {hdmi_red[*]}]
set_property DRIVE 12 [get_ports {hdmi_green[*]}]
set_property DRIVE 12 [get_ports {hdmi_blue[*]}]
set_property DRIVE 12 [get_ports {hdmi_hsync}]
set_property DRIVE 12 [get_ports {hdmi_vsync}]
set_property DRIVE 12 [get_ports {hdmi_de}]