mod div;

fn main() {
    div::func_ex_print_some(div::func_ex_div_some(10, 5));
    div::func_ex_print_some(div::func_ex_div_some(10, 0));
    div::func_ex_print_some_match(div::func_ex_div_some(10, 5));
    div::func_ex_print_some_match(div::func_ex_div_some(10, 0));
    div::func_ex_print_result(div::func_ex_div_result(10, 5));
    div::func_ex_print_result(div::func_ex_div_result(10, 0));
    div::func_ex_print_result(div::func_ex_div_result(300, 4));
}
