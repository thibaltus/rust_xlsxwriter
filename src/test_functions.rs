// Shared test functions for unit tests.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2022, John McNamara, jmcnamara@cpan.org

use std::fs::File;
use std::io::{Read, Seek};

#[cfg(test)]
// Convert XML string/doc into a vector for comparison testing.
pub fn xml_to_vec(xml_string: &str) -> Vec<String> {
    let mut xml_elements: Vec<String> = Vec::new();
    let re = regex::Regex::new(r">\s*<").unwrap();
    let tokens: Vec<&str> = re.split(xml_string).collect();

    for token in &tokens {
        let mut element = token.trim().to_string();

        // Add back the removed brackets.
        if !element.starts_with('<') {
            element = format!("<{}", element);
        }
        if !element.ends_with('>') {
            element = format!("{}>", element);
        }

        xml_elements.push(element);
    }
    xml_elements
}

// Test helper to read xml data back from a filehandle.
#[allow(dead_code)]
pub fn read_xmlfile_data(tempfile: &mut File) -> String {
    let mut got = String::new();
    tempfile.rewind().unwrap();
    tempfile.read_to_string(&mut got).unwrap();
    got
}