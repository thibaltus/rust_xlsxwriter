// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use rust_xlsxwriter::{FilterCondition, FilterCriteria, Format, Workbook, XlsxError};

mod common;

// Test to demonstrate autofilters.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();
    let bold = Format::new().set_bold();

    worksheet.write_string_only(0, 0, "Header")?;
    worksheet.write_string_only(1, 0, "")?;
    worksheet.write_string(2, 0, "", &bold)?;
    worksheet.write_string_only(3, 0, " ")?;
    worksheet.write_string_only(4, 0, "  ")?;
    worksheet.write_string_only(5, 0, "Foo")?;
    worksheet.write_boolean_only(6, 0, true)?;
    worksheet.write_formula_only(7, 0, "=1-1")?;

    worksheet.autofilter(0, 0, 7, 0)?;

    let filter_condition =
        FilterCondition::new().add_custom_string_filter(FilterCriteria::NotEqualTo, " ");
    worksheet.filter_column(0, &filter_condition)?;

    workbook.save(filename)?;

    Ok(())
}
#[test]
fn test_bootstrap59() {
    let test_runner = common::TestRunner::new()
        .set_name("bootstrap59")
        .set_function(create_new_xlsx_file)
        .ignore_calc_chain()
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
