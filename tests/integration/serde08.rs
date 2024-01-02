// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{CustomSerializeField, Format, SerializeFieldOptions, Workbook, XlsxError};
use serde::Serialize;

// Test case for Serde serialization. First test isn't serialized.
fn create_new_xlsx_file_1(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    let bold = Format::new().set_bold();
    let italic = Format::new().set_italic();
    let bold_italic = Format::new().set_bold().set_italic();

    // Not serialized.
    worksheet.write(0, 0, "col1")?;
    worksheet.write_with_format(1, 0, 1, &bold)?;
    worksheet.write_with_format(2, 0, 2, &bold)?;
    worksheet.write_with_format(3, 0, 3, &bold)?;

    worksheet.write(0, 1, "col2")?;
    worksheet.write_with_format(1, 1, 4, &italic)?;
    worksheet.write_with_format(2, 1, 5, &italic)?;
    worksheet.write_with_format(3, 1, 6, &italic)?;

    worksheet.write(0, 2, "col3")?;
    worksheet.write_with_format(1, 2, 7, &bold_italic)?;
    worksheet.write_with_format(2, 2, "", &bold_italic)?;
    worksheet.write_with_format(3, 2, 9, &bold_italic)?;

    workbook.save(filename)?;

    Ok(())
}

// Test case for Serde serialization.
fn create_new_xlsx_file_2(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    let bold = Format::new().set_bold();
    let italic = Format::new().set_italic();
    let bold_italic = Format::new().set_bold().set_italic();

    // Create a serializable test struct.
    #[derive(Serialize)]
    struct MyStruct {
        col1: u8,
        col2: u8,
        col3: Option<u8>,
    }

    let data1 = MyStruct {
        col1: 1,
        col2: 4,
        col3: Some(7),
    };

    let data2 = MyStruct {
        col1: 2,
        col2: 5,
        col3: None,
    };

    let data3 = MyStruct {
        col1: 3,
        col2: 6,
        col3: Some(9),
    };

    let custom_headers = [
        CustomSerializeField::new("col1").set_value_format(&bold),
        CustomSerializeField::new("col2").set_value_format(&italic),
        CustomSerializeField::new("col3").set_value_format(&bold_italic),
    ];
    let header_options = SerializeFieldOptions::new().set_custom_headers(&custom_headers);

    worksheet.serialize_headers_with_options(0, 0, &data1, &header_options)?;

    worksheet.serialize(&data1)?;
    worksheet.serialize(&data2)?;
    worksheet.serialize(&data3)?;

    // Secondary test. This should be ignored since one of the field names is wrong.
    let custom_headers = [
        CustomSerializeField::new("col1"),
        CustomSerializeField::new("col2"),
        CustomSerializeField::new("col99"),
    ];
    let header_options = SerializeFieldOptions::new().set_custom_headers(&custom_headers);

    let _ = worksheet.serialize_headers_with_options(6, 0, &data1, &header_options);

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_serde08_1() {
    let test_runner = common::TestRunner::new()
        .set_name("serde08")
        .set_function(create_new_xlsx_file_1)
        .unique("1")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_serde08_2() {
    let test_runner = common::TestRunner::new()
        .set_name("serde08")
        .set_function(create_new_xlsx_file_2)
        .unique("2")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
