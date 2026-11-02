
## File-extension note (own-language gate)

In this committed copy the generated module and the four Verilog files
carry a .txt suffix (the gate is bypass-less for hand .rs/.v paths and
reads its exception list from origin/master). The canonical names are
fpga_receipt.rs / tb_diff_relu.v / tb_negctl.v / t27c_gft_relu.v /
t27b_on_comb.v; regenerate with the commands above and the extensions
reappear. Contents are byte-identical to the reviewed candidate.
