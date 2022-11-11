// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0 Copyright 2022, John McNamara,
// jmcnamara@cpan.org

use rust_xlsxwriter::{Format, Workbook, XlsxError};

mod common;

// Test case to demonstrate cell font formatting.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let mut format1 = Format::default();
    let format2 = Format::new().set_font_name("Arial");
    let format3 = Format::new().set_font_name("Consolas").set_font_family(3);
    let format4 = Format::new().set_font_size(10);
    let format5 = Format::new().set_font_size(9.5);

    let worksheet = workbook.add_worksheet();
    worksheet.write_string(0, 0, "Rust", &format1)?;
    worksheet.write_string(1, 0, "Rust", &format2)?;
    worksheet.write_string(2, 0, "Rust", &format3)?;
    worksheet.write_string(3, 0, "Rust", &format4)?;
    worksheet.write_string(4, 0, "Rust", &format5)?;

    // Secondary test for default format.
    workbook.register_format(&mut format1);
    workbook.register_format(&mut format1);

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn bootstrap19_font_name_and_size() {
    let test_runner = common::TestRunner::new()
        .set_name("bootstrap19")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
