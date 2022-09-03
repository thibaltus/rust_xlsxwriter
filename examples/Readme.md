# Examples for the rust_xlsxwriter library.

This directory contains working examples showing different features of the
rust_xlsxwriter library.

The `app_{name}.rs` examples are small complete programs showing a feature or
collection of features. The `doc_{struct}_{function}.rs` examples are more
specific and generally show how an individual function works. The `doc_*.rs`
examples are usually repeated in the documentation.

* app_demo.rs - A simple, getting started, example of some of the features
  of the rust_xlsxwriter library.

* app_perf_test.rs - Simple performance test for rust_xlsxwriter.

* doc_enum_xlsxcolor.rs - Demonstrates using different XlsxColor enum
  values to set the color of some text in a worksheet.

* doc_format_clone.rs - Demonstrates cloning a format and setting the
  properties.

* doc_format_create.rs - Demonstrates create a new format and setting the
  properties.

* doc_format_currency1.rs - Demonstrates setting a currency format for a
  worksheet cell. This example doesn't actually set a currency format, for
  that see the followup example in doc_format_currency2.rs.

* doc_format_currency2.rs - Demonstrates setting a currency format for a
  worksheet cell.

* doc_format_default.rs - Demonstrates creating a default format.

* doc_format_locale.rs - Demonstrates setting a number format that appears
  differently in different locales.

* doc_format_new.rs - Demonstrates creating a new format.

* doc_format_set_bold.rs - Demonstrates setting the bold property for a
  format.

* doc_format_set_font_color.rs - Demonstrates setting the italic property
  for a format.

* doc_format_set_font_name.rs - Demonstrates setting the font name/type for
  a format.

* doc_format_set_font_size.rs - Demonstrates setting the font size for a
  format.

* doc_format_set_font_strikeout.rs - Demonstrates setting the text
  strikeout/strikethrough property for a format.

* doc_format_set_italic.rs - Demonstrates setting the italic property for a
  format.

* doc_format_set_num_format.rs - Demonstrates setting different types of
  Excel number formatting.

* doc_format_set_num_format_index.rs - Demonstrates setting one of the
  inbuilt format indices for a format.

* doc_format_set_underline.rs - Demonstrates setting underline properties
  for a format.

* doc_worksheet_set_column_format.rs - Demonstrates setting the format for
  a column in Excel.

* doc_worksheet_set_column_width.rs - Demonstrates setting the width of
  columns in Excel.

* doc_worksheet_set_column_width_pixels.rs - Demonstrates setting the width
  of columns in Excel in pixels.

* doc_worksheet_set_name.rs - Demonstrates setting user defined worksheet
  names and the default values when a name isn't set.

* doc_worksheet_set_row_format.rs - Demonstrates setting the format for a
  row in Excel.

* doc_worksheet_set_row_height.rs - Demonstrates setting the height for a
  row in Excel.

* doc_worksheet_set_row_height_pixels.rs - Demonstrates setting the height
  for a row in Excel.

* doc_worksheet_write_number.rs - Demonstrates setting different formatting
  for numbers in an Excel worksheet.

* doc_worksheet_write_number_only.rs - Demonstrates writing unformatted
  numbers to an Excel worksheet. Any numeric type that will convert
  [`Into`] f64 can be transferred to Excel.

* doc_worksheet_write_string.rs - Demonstrates setting different formatting
  for numbers in an Excel worksheet.

* doc_worksheet_write_string_only.rs - Demonstrates writing some UTF-8
  strings to a worksheet. The UTF-8 encoding is the only encoding supported
  by the Excel file format.
