pub use plotters;
pub use plotters::prelude::*;
pub use serde::{Deserialize, Serialize};
pub use std::error::Error;
pub use std::path::Path;
pub use std::{fmt, ops};

type TheData = Vec<Vec<f32>>;
#[derive(Debug, Clone)]
pub struct DataCsv {
    pub name: Vec<String>,
    pub data: TheData,
}

impl DataCsv {
    fn new() -> Self {
        DataCsv {
            name: Vec::new(),
            data: Vec::new(),
        }
    }
}

pub const COLORDATA: [RGBColor; 7] = [RED, BLUE, GREEN, CYAN, YELLOW, MAGENTA, BLACK];

pub fn get_data<'a>(p: &'a Path) -> Result<DataCsv, Box<dyn Error>> {
    let mut data = DataCsv::new();
    let mut rdr = csv::Reader::from_path(&p).expect("ERROR: cant read data!");
    data.name = rdr
        .headers()
        .expect("ERROR: file csv: {} doesn't have headers")
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    data.data = rdr.deserialize().map(|f| f.unwrap()).collect();
    Ok(data)
}
pub fn get_data_generic<'a>(p: &'a Path) -> Result<(), Box<dyn Error>> {
    let data = get_data(&p)?;
    let lenght = data.data[0].len();
    for d in data.data {
        for i in 0..lenght {
            print!("| {}: {} |", data.name[i], d[i]);
        }
        print!("\n");
    }

    Ok(())
}

pub fn get_minmax_vec<D>(arr: D, idx: usize) -> (f32, f32)
where
    D: AsRef<TheData>,
{
    let temp: Vec<_> = arr.as_ref().iter().map(|v| v[idx]).collect();
    (
        temp.iter()
            .fold(0.0f32, |min, &val| if val < min { val } else { min }),
        temp.iter()
            .fold(0.0f32, |max, &val| if val > max { val } else { max }),
    )
}
pub fn get_minmax_all<D>(arr: D) -> (f32, f32)
where
    D: AsRef<Vec<Vec<f32>>>,
{
    (
        arr.as_ref()
            .iter()
            .map(|data| {
                data.iter()
                    .fold(0.0f32, |min, &val| if val < min { val } else { min })
            })
            .fold(0.0f32, |min, val| if val < min { val } else { min }),
        arr.as_ref()
            .iter()
            .map(|data| {
                data.iter()
                    .fold(0.0f32, |max, &val| if val > max { val } else { max })
            })
            .fold(0.0f32, |max, val| if val > max { val } else { max }),
    )
}

#[cfg(test)]
mod tests {
    use crate::data::{get_minmax_all, get_minmax_vec, get_data};
    use std::path::PathBuf;

    const NAMEFILE: &str = "./testdata.csv";

    #[test]
    fn test_lenght_data() {
        const LENGHT: usize = 100;
        let len = get_data(&PathBuf::from(NAMEFILE)).expect("Cannot Parse File").data.len();
        assert_eq!(
            len, LENGHT,
            "Testing Lenght of data CSV: {} == {}",
            len, LENGHT
        );
    }

    #[test]
    fn test_minmax_data_id() {
        const EXPECT: (f32, f32) = (0.0, 100.0);
        let data = get_minmax_vec(&get_data(&PathBuf::from(NAMEFILE)).expect("").data, 0);
        assert_eq!(
            data, EXPECT,
            "test_minmax_data: {:#?} == {:#?}",
            data, EXPECT
        );
    }
    #[test]
    fn test_minmax_data_speed() {
        const EXPECT: (f32, f32) = (0.0, 296.15);
        let data = get_minmax_vec(&get_data(&PathBuf::from(NAMEFILE)).expect("").data, 1);
        assert_eq!(
            data, EXPECT,
            "test_minmax_data: {:#?} == {:#?}",
            data, EXPECT
        );
    }

    #[test]
    fn test_minmax_data_rpm() {
        const EXPECT: (f32, f32) = (0.0, 499.14);
        let data = get_minmax_vec(&get_data(&PathBuf::from(NAMEFILE)).expect("").data, 2);
        assert_eq!(
            data, EXPECT,
            "test_minmax_data: {:#?} == {:#?}",
            data, EXPECT
        );
    }
    #[test]
    fn test_minmax_data_torsi() {
        const EXPECT: (f32, f32) = (0.0, 99.51);
        let data = get_minmax_vec(&get_data(&PathBuf::from(NAMEFILE)).expect("").data, 3);
        assert_eq!(
            data, EXPECT,
            "test_minmax_data: {:#?} == {:#?}",
            data, EXPECT
        );
    }
    #[test]
    fn test_minmax_data_hp() {
        const EXPECT: (f32, f32) = (0.0, 49.33);
        let data = get_minmax_vec(&get_data(&PathBuf::from(NAMEFILE)).expect("").data, 4);
        assert_eq!(
            data, EXPECT,
            "test_minmax_data: {:#?} == {:#?}",
            data, EXPECT
        );
    }

    #[test]
    fn test_minmax_data_all() {
        const EXPECT: (f32, f32) = (0.0, 499.14);
        let data = get_minmax_all(&get_data(&PathBuf::from(NAMEFILE)).expect("cannot parse").data);
        assert_eq!(
            data, EXPECT,
            "test_minmax_data: {:#?} == {:#?}",
            data, EXPECT
        );
    }

    #[test]
    fn test_sum_path() {
        let mut p = PathBuf::from("./uwuuwu.uwu");
        p.file_stem().unwrap();
        p.set_extension("png");

        let mut s = String::from("hehe");
        s.push_str(p.to_str().unwrap());
        println!("path : {}", s);
        assert_eq!("", s);
    }
}
