use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;

use ndarray::Array2;

fn main() {
    let args: Vec<String> = env::args().collect();
    let dssp = fs::read_to_string(&args[1]).unwrap();
    // let mut buf: String = String::new();
    // dssp.read_to_string(&mut buf).unwrap();
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

    write_array_to_csv_manual(&new_ss_matrix, "ss.csv");
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


// using DataFrames
// using CSV
// using Utils

// export read_xpm, xpm2csv, convert_ss_table!
// export XPM

// mutable struct XPM
//     xpmdata::DataFrame
//     xaxis
//     yaxis
//     xlabel
//     ylabel
// end

// function read_xpm(xpmfilename::String)
//     println("Reading $xpmfilename...")
//     xpmfile = readlines(xpmfilename)
//     xpm_index = findfirst(x -> contains(x, "static char *"), xpmfile)
//     params = split(match(r"\"(.*)\"", xpmfile[xpm_index + 1]).captures[1])
//     width, height, ncolors, cpp = parse.(Int32, params[1:4])  # 颜色数, 每个像素的字符数
//     # color_table structure: 
//     # ' ' => {"c" => "red", "m" => "white", "s" => "light_color"}
//     # 'Y' => {"c" => "green", "m" => "black", "s" => "lines_in_mix"}
//     # '+' => {"c" => "yellow", "m" => "white", "s" => "lines_in_dark"}
//     color_table = Dict()
//     for i in 1:ncolors
//         line = match(r"\"(.*?)\"", xpmfile[xpm_index + i + 1]).captures[1]
//         # 提取对应每种关键字的颜色定义
//         cs = reshape(split(line[2:end]), 2, :)
//         color_def = [(cs[1, c] => cs[2, c]) for c in 1:size(cs, 2)]
//         push!(color_table, line[1] => color_def)
//     end
//     xpm_data_index_start = findfirst(x -> match(r"^\".*", x) !== nothing, xpmfile[xpm_index + ncolors + 2:end])
//     xpm_data = xpmfile[xpm_data_index_start + xpm_index + ncolors + 1:xpm_data_index_start + xpm_index + ncolors + height]
//     xpm_data = [x.captures[1] for x in match.(r"\"(.*?)\"", xpm_data)]   # 字符串数组
//     xpm_matrix = fill("", width, height)   # 矩阵, 纵向为x, 横向为y
//     for i in 1:height
//         for j in 1:width
//             xpm_matrix[j, i] = xpm_data[i][(j - 1) * cpp + 1:j * cpp]
//         end
//     end
//     # 坐标轴
//     xaxis = []
//     for x in filter(x -> contains(x, "/* x-axis:"), xpmfile)
//         append!(xaxis, parse.(Float64, split(x)[3:end - 1]))
//     end
//     yaxis = []
//     for y in filter(y -> contains(y, "/* y-axis:"), xpmfile)
//         append!(yaxis, parse.(Float64, split(y)[3:end - 1]))
//     end
//     # label
//     xlabel_index = findfirst(x -> contains(x, "/* x-label"), xpmfile)
//     xlabel = match(r"/* x-label: \"(.*)\".*", xpmfile[xlabel_index]).captures[1]
//     ylabel_index = findfirst(x -> contains(x, "/* y-label"), xpmfile)
//     ylabel = match(r"/* y-label: \"(.*)\".*", xpmfile[ylabel_index]).captures[1]
//     finish_hint("Finished reading $xpmfilename.\n")
//     return XPM(DataFrame([xaxis xpm_matrix], insert!(Symbol.(yaxis), 1, Symbol(xlabel))), xaxis, yaxis, xlabel, ylabel)
// end

// function convert_ss_table!(xpm::XPM)
//     color_table = Dict(
//         "~" => "Coil",
//         "E" => "B-Sheet",
//         "B" => "B-Bridge",
//         "P" => "PPII-Helix",
//         "S" => "Bend",
//         "T" => "Turn",
//         "H" => "A-Helix",
//         "I" => "5-Helix",
//         "G" => "3-Helix",
//         "=" => "Chain_Separator"
//     )
//     for (k, v) in color_table
//         xpm.xpmdata[!, :] .= ifelse.(xpm.xpmdata[!, :] .== k, v, xpm.xpmdata[!, :])
//     end
// end

// function xpm2csv(xpm::XPM, path::AbstractString)
//     CSV.write(path, xpm.xpmdata)
// end

// end # module
