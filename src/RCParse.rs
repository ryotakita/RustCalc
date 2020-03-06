#![allow(non_snake_case)]
use std::fs::File;
use std::io::{self, BufRead,BufReader};
/// parse Of Girder 
/// createNode
/// ```
/// use CalcArc::GroupOfNode;
/// let node = GroupOfNode::createNode(0., 0.);
/// ```
pub fn parseOfGirder(filename: &str) -> Vec<String>
{
    let mut lstGirder = Vec::new();
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        // Show the line and its number.
        lstGirder.push(line);
    }
    lstGirder
}