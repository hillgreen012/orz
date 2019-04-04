#![allow(dead_code)]

use std;
use std::io::Write;

pub fn generate() {
    let fvalue_dest_path = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).join("MTF_VALUE_ARRAY.txt");
    let findex_dest_path = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).join("MTF_INDEX_ARRAY.txt");
    let fnext_dest_path  = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).join("MTF_NEXT_ARRAY.txt");
    let mut fvalue = std::io::BufWriter::new(std::fs::File::create(&fvalue_dest_path).unwrap());
    let mut findex = std::io::BufWriter::new(std::fs::File::create(&findex_dest_path).unwrap());
    let mut fnext  = std::io::BufWriter::new(std::fs::File::create(&fnext_dest_path).unwrap());

    let value_array = [
        32,  115, 101, 256, 111, 97,  116, 351, 114, 110, 347, 105, 337, 342, 356, 104,
        327, 352, 108, 46,  39,  91,  99,  331, 332, 317, 336, 261, 326, 321, 341, 117,
        93,  109, 100, 322, 44,  98,  112, 346, 301, 124, 102, 107, 338, 316, 291, 10,
        103, 307, 121, 329, 311, 349, 312, 49,  353, 257, 343, 350, 348, 306, 281, 286,
        297, 266, 340, 271, 345, 77,  70,  84,  355, 65,  50,  40,  262, 61,  75,  52,
        354, 339, 57,  119, 118, 74,  72,  232, 47,  41,  319, 292, 314, 78,  80,  48,
        333, 330, 276, 45,  76,  38,  122, 197, 68,  334, 67,  66,  328, 158, 90,  273,
        279, 106, 303, 79,  344, 277, 83,  318, 335, 299, 216, 141, 295, 53,  55,  325,
        323, 125, 324, 309, 302, 73,  289, 282, 227, 165, 304, 285, 92,  258, 188, 233,
        69,  288, 59,  231, 170, 58,  308, 56,  284, 313, 54,  320, 236, 164, 310, 300,
        163, 264, 296, 293, 133, 265, 260, 208, 71,  42,  89,  36,  209, 181, 195, 298,
        137, 82,  225, 287, 272, 268, 259, 267, 214, 217, 305, 283, 95,  294, 187, 224,
        131, 159, 230, 182, 196, 51,  274, 128, 218, 168, 87,  135, 177, 235, 219, 172,
        9,   167, 43,  278, 176, 63,  169, 189, 37,  179, 270, 263, 129, 234, 175, 269,
        146, 171, 160, 62,  150, 139, 226, 275, 123, 64,  156, 152, 290, 33,  239, 206,
        85,  178, 145, 315, 154, 151, 138, 215, 229, 120, 173, 251, 252, 253, 254, 255,
        60,  143, 162, 194, 149, 81,  200, 0,   1,   96,  213, 2,   3,   4,   191, 5,
        280, 161, 6,   7,   153, 88,  8,   11,  12,  157, 147, 13,  190, 210, 14,  15,
        16,  142, 17,  35,  199, 18,  134, 19,  94,  20,  205, 21,  22,  174, 23,  136,
        24,  211, 25,  126, 26,  204, 228, 27,  203, 28,  212, 29,  155, 34,  30,  31,
        220, 180, 127, 113, 130, 192, 193, 198, 221, 207, 237, 223, 86,  238, 241, 240,
        242, 222, 202, 243, 144, 185, 244, 140, 245, 166, 132, 246, 148, 247, 184, 186,
        248, 249, 183, 201, 250
    ];
    write!(fvalue, "[{}]", value_array.iter().map(ToString::to_string).collect::<Vec<_>>().join(",")).unwrap();

    let index_array = [
        263, 264, 267, 268, 269, 271, 274, 275, 278, 208, 47,  279, 280, 283, 286, 287,
        288, 290, 293, 295, 297, 299, 300, 302, 304, 306, 308, 311, 313, 315, 318, 319,
        0,   237, 317, 291, 171, 216, 101, 20,  75,  89,  169, 210, 36,  99,  19,  88,
        95,  55,  74,  197, 79,  125, 154, 126, 151, 82,  149, 146, 256, 77,  227, 213,
        233, 73,  107, 106, 104, 144, 70,  168, 86,  133, 85,  78,  100, 69,  93,  115,
        94,  261, 177, 118, 71,  240, 332, 202, 277, 170, 110, 21,  140, 32,  296, 188,
        265, 5,   37,  22,  34,  2,   42,  48,  15,  11,  113, 43,  18,  33,  9,   4,
        38,  323, 8,   1,   6,   31,  84,  83,  249, 50,  102, 232, 41,  129, 307, 322,
        199, 220, 324, 192, 346, 164, 294, 203, 303, 176, 246, 229, 343, 123, 289, 257,
        340, 242, 224, 282, 348, 260, 228, 245, 235, 276, 244, 316, 234, 281, 109, 193,
        226, 273, 258, 160, 157, 137, 345, 209, 201, 214, 148, 225, 207, 250, 301, 222,
        212, 204, 241, 217, 321, 173, 195, 354, 350, 341, 351, 190, 142, 215, 284, 270,
        325, 326, 259, 174, 196, 103, 327, 292, 262, 355, 338, 312, 309, 298, 239, 329,
        167, 172, 285, 305, 314, 266, 184, 247, 122, 185, 200, 206, 320, 328, 337, 331,
        191, 178, 230, 136, 310, 248, 194, 147, 87,  143, 221, 205, 156, 330, 333, 238,
        335, 334, 336, 339, 342, 344, 347, 349, 352, 353, 356, 251, 252, 253, 254, 255,
        3,   57,  141, 182, 166, 27,  76,  219, 161, 165, 65,  183, 181, 223, 218, 67,
        180, 111, 198, 231, 98,  117, 211, 112, 272, 62,  135, 187, 152, 139, 63,  179,
        145, 134, 236, 46,  91,  163, 189, 124, 162, 64,  175, 121, 159, 40,  132, 114,
        138, 186, 61,  49,  150, 131, 158, 52,  54,  153, 92,  243, 45,  25,  119, 90,
        155, 29,  35,  128, 130, 127, 28,  16,  108, 51,  97,  23,  24,  96,  105, 120,
        26,  12,  44,  81,  66,  30,  13,  58,  116, 68,  39,  10,  60,  53,  59,  7,
        17,  56,  80,  72,  14
    ];
    write!(findex, "[{}]", index_array.iter().map(ToString::to_string).collect::<Vec<_>>().join(",")).unwrap();

    write!(fnext, "[").unwrap();
    for i in 0 .. value_array.len() {
        let next = (i as f64 * 0.999999).powf(1.0 - 0.06 * (i * 256 / value_array.len()) as f64 / 256.0) as usize;
        write!(fnext, "{},", std::cmp::min(255, next)).unwrap();
    }
    write!(fnext, "]").unwrap();
}
