
perf stat -e  cycles,cache-references,l1d.replacement,l2_lines_out.non_silent,l2_lines_out.silent,l2_trans.l2_wb ./target/release/bypass-cache standard
perf stat -e  cycles,cache-references,l1d.replacement,l2_lines_out.non_silent,l2_lines_out.silent,l2_trans.l2_wb ./target/release/bypass-cache nocache