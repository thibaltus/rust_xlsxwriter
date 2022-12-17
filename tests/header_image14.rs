// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2022, John McNamara, jmcnamara@cpan.org

use rust_xlsxwriter::{Image, Workbook, XlsxError, XlsxImagePosition};

mod common;

// Test to demonstrate adding header/footer images to worksheets.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    let image1 = Image::new("tests/input/images/black_72e.png")?;
    let image2 = Image::new("tests/input/images/black_150e.png")?;
    let image3 = Image::new("tests/input/images/black_300e.png")?;

    worksheet.set_header("&L&G&C&G&R&G");
    worksheet.set_header_image(&image1, XlsxImagePosition::Left)?;
    worksheet.set_header_image(&image2, XlsxImagePosition::Center)?;
    worksheet.set_header_image(&image3, XlsxImagePosition::Right)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_header_image14() {
    let test_runner = common::TestRunner::new()
        .set_name("header_image14")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
