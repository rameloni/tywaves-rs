use std::str::FromStr;
use tywaves::vcd_rewrite::{IdCodeWithShift, VcdRewriteVariable};

#[test]
fn vcd_rewrite_variable() {
    let id_code = vcd::IdCode::FIRST;
    let id_code_a = id_code.next();
    let id_code_b = id_code_a.next();
    let id_code_c = id_code_b.next();

    let width: usize = 10;
    let mut input_sum = VcdRewriteVariable::new(
        id_code,
        width as u32,
        vec![
            IdCodeWithShift::new(id_code_a, vcd::Vector::from_str("000").unwrap()), // 0b00000001
            IdCodeWithShift::new(id_code_b, vcd::Vector::from_str("10001").unwrap()), // 0b00001000

            IdCodeWithShift::new(id_code_c, vcd::Vector::from_str("0").unwrap()), // 0b00100000
        ],
    );

    assert_eq!(input_sum.get_value(), vcd::Vector::from_str(&format!("{:0>width$}", "10001000", width = width)).unwrap());

    input_sum.update_value(id_code_a, &vcd::Vector::from_str("001").unwrap());
    assert_eq!(input_sum.get_value(), vcd::Vector::from_str(&format!("{:0>width$}", "10001001", width = width)).unwrap());

    input_sum.update_value(id_code_b, &vcd::Vector::from_str("00000").unwrap());
    assert_eq!(input_sum.get_value(), vcd::Vector::from_str(&format!("{:0>width$}", "00000001", width = width)).unwrap());
}


#[test]
#[should_panic]
fn vec_push_vs_instert() {
    let mut vec_push = Vec::new();
    let mut vec_insert = Vec::new();
    for i in 0..10 {
        vec_push.push(i);
        vec_insert.insert(0, i);
    }

    assert_eq!(vec_push, vec_insert);
}
