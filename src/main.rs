use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;

use ndarray::Array2;

fn main() {
    let args: Vec<String> = env::args().collect();
    let dssp = if args.len() > 1 {
        &args[1]
    } else {
        "dssp.dat"
    };
    println!("Reading dssp.dat...");
    let dssp = fs::read_to_string(dssp).unwrap();
    println!("Converting dssp.dat...");
    let dssp_m: Vec<&str> = dssp.split("\n").filter(|l| !l.trim().is_empty()).collect();
    let ln = dssp_m.len();
    let dssp: Vec<&str> = dssp.split("").filter(|l| !l.trim().is_empty()).collect();
    let ss_matrix = Array2::from_shape_vec((ln, dssp_m[0].len()), dssp).unwrap();
    let mut replace_map: HashMap<&str, &str> = HashMap::new();
    replace_map.insert("~", "Coil");
    replace_map.insert("E", "B-Sheet");
    replace_map.insert("B", "B-Bridge");
    replace_map.insert("P", "PPII-Helix");
    replace_map.insert("S", "Bend");
    replace_map.insert("T", "Turn");
    replace_map.insert("H", "A-Helix");
    replace_map.insert("I", "5-Helix");
    replace_map.insert("G", "3-Helix");
    replace_map.insert("=", "Chain_Separator");
    let new_ss_matrix = ss_matrix.mapv(|x| replace_map[x]);

    println!("Writing new dssp file ss.csv...");
    write_array_to_csv_manual(&new_ss_matrix, "ss.csv");
    println!("Writing dssp summary scount.csv...");
    scount(&new_ss_matrix, "scount.csv");
}

fn scount(array: &Array2<&str>, file_path: &str) {
    let mut file = File::create(file_path).unwrap();
    writeln!(file, "Helix.A-Helix,Helix.3-Helix,Helix.5-Helix,Helix.PPII-Helix,Sheet.B-Sheet,Sheet.B-Bridge,Turn.Turn,Turn.Bend,Coil").unwrap();
    for row in array.rows() {
        write!(file, "{},", row.iter().filter(|&&ss| ss.eq("A-Helix")).count()).unwrap();
        write!(file, "{},", row.iter().filter(|&&ss| ss.eq("3-Helix")).count()).unwrap();
        write!(file, "{},", row.iter().filter(|&&ss| ss.eq("5-Helix")).count()).unwrap();
        write!(file, "{},", row.iter().filter(|&&ss| ss.eq("PPII-Helix")).count()).unwrap();
        write!(file, "{},", row.iter().filter(|&&ss| ss.eq("B-Sheet")).count()).unwrap();
        write!(file, "{},", row.iter().filter(|&&ss| ss.eq("B-Bridge")).count()).unwrap();
        write!(file, "{},", row.iter().filter(|&&ss| ss.eq("Turn")).count()).unwrap();
        write!(file, "{},", row.iter().filter(|&&ss| ss.eq("Bend")).count()).unwrap();
        write!(file, "{}\n", row.iter().filter(|&&ss| ss.eq("Coil")).count()).unwrap();
    }
}

fn write_array_to_csv_manual(array: &Array2<&str>, file_path: &str) {
    let mut file = File::create(file_path).unwrap();
    for row in array.rows() {
        // 使用逗号连接每一行的元素
        let line = row.iter().map(|&x| x).collect::<Vec<_>>().join(",");
        writeln!(file, "{}", line).unwrap();
    }
}
