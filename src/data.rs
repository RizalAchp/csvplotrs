pub use plotters;
pub use plotters::prelude::*;
pub use serde::{Deserialize, Serialize};
pub use std::error::Error;
pub use std::path::Path;
pub use std::{fmt, ops};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DataCsv {
    pub id: f32,
    pub speed: f32,
    pub rpm: f32,
    pub torsi: f32,
    pub horsepower: f32,
}
pub const NAMEDATA: [&str; 5] = ["id", "speed", "rpm", "torsi", "horsepower"];
pub const COLORDATA: [&RGBColor; 5] = [&BLACK, &RED, &BLUE, &GREEN, &CYAN];

impl DataCsv {
    fn get_slice(&self) -> [f32; 4] {
        return [self.speed, self.rpm, self.torsi, self.horsepower];
    }
}

impl ops::Index<usize> for DataCsv {
    type Output = f32;
    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => return &self.id,
            1 => return &self.speed,
            2 => return &self.rpm,
            3 => return &self.torsi,
            4 => return &self.horsepower,
            _ => return &self.id,
        }
    }
}

impl ops::IndexMut<usize> for DataCsv {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => return &mut self.id,
            1 => return &mut self.speed,
            2 => return &mut self.rpm,
            3 => return &mut self.torsi,
            4 => return &mut self.horsepower,
            _ => return &mut self.id,
        }
    }
}

pub fn get_data<'a>(p: &'a Path) -> Vec<DataCsv> {
    csv::Reader::from_path(&p)
        .unwrap()
        .deserialize()
        .map(|f| f.unwrap())
        .map(|i| DataCsv::from(i))
        .collect::<Vec<_>>()
}

pub fn get_data_generic<'a>(p: &'a Path) -> Vec<Vec<f32>> {
    csv::Reader::from_path(&p)
        .unwrap()
        .deserialize()
        .map(|f| f.unwrap())
        .collect::<Vec<_>>()
}

pub fn get_minmax_idx<D>(arr: D, idx: usize) -> (f32, f32)
where
    D: AsRef<Vec<DataCsv>>,
{
    let temp = arr.as_ref().iter().map(|d| d[idx]).collect::<Vec<_>>();
    (
        temp.iter()
            .fold(0.0f32, |min, &val| if val < min { val } else { min }),
        temp.iter()
            .fold(0.0f32, |max, &val| if val > max { val } else { max }),
    )
}
pub fn get_minmax_all<D>(arr: D) -> (f32, f32)
where
    D: AsRef<Vec<DataCsv>>,
{
    (
        arr.as_ref()
            .iter()
            .map(|d| {
                d.get_slice()
                    .iter()
                    .fold(0.0f32, |min, &val| if val < min { val } else { min })
            })
            .collect::<Vec<_>>()
            .iter()
            .fold(0.0f32, |min, &val| if val < min { val } else { min }),
        arr.as_ref()
            .iter()
            .map(|d| {
                d.get_slice()
                    .iter()
                    .fold(0.0f32, |max, &val| if val > max { val } else { max })
            })
            .collect::<Vec<_>>()
            .iter()
            .fold(0.0f32, |max, &val| if val > max { val } else { max }),
    )
}

#[cfg(test)]
mod tests {
    use crate::data::{get_data, get_data_generic, get_minmax_all, get_minmax_idx};
    use std::path::PathBuf;

    const NAMEFILE: &str = "./testdata.csv";

    #[test]
    fn test_lenght_data() {
        const LENGHT: usize = 100;
        let len = get_data_generic(&PathBuf::from(NAMEFILE)).len();
        assert_eq!(
            len, LENGHT,
            "Testing Lenght of data CSV: {} == {}",
            len, LENGHT
        );
    }

    #[test]
    fn test_minmax_data_id() {
        const EXPECT: (f32, f32) = (0.0, 100.0);
        let data = get_minmax_idx(get_data(&PathBuf::from(NAMEFILE)), 0);
        assert_eq!(
            data, EXPECT,
            "test_minmax_data: {:#?} == {:#?}",
            data, EXPECT
        );
    }
    #[test]
    fn test_minmax_data_speed() {
        const EXPECT: (f32, f32) = (0.0, 296.15);
        let data = get_minmax_idx(get_data(&PathBuf::from(NAMEFILE)), 1);
        assert_eq!(
            data, EXPECT,
            "test_minmax_data: {:#?} == {:#?}",
            data, EXPECT
        );
    }

    #[test]
    fn test_minmax_data_rpm() {
        const EXPECT: (f32, f32) = (0.0, 499.14);
        let data = get_minmax_idx(get_data(&PathBuf::from(NAMEFILE)), 2);
        assert_eq!(
            data, EXPECT,
            "test_minmax_data: {:#?} == {:#?}",
            data, EXPECT
        );
    }
    #[test]
    fn test_minmax_data_torsi() {
        const EXPECT: (f32, f32) = (0.0, 99.51);
        let data = get_minmax_idx(get_data(&PathBuf::from(NAMEFILE)), 3);
        assert_eq!(
            data, EXPECT,
            "test_minmax_data: {:#?} == {:#?}",
            data, EXPECT
        );
    }
    #[test]
    fn test_minmax_data_hp() {
        const EXPECT: (f32, f32) = (0.0, 49.33);
        let data = get_minmax_idx(get_data(&PathBuf::from(NAMEFILE)), 4);
        assert_eq!(
            data, EXPECT,
            "test_minmax_data: {:#?} == {:#?}",
            data, EXPECT
        );
    }

    #[test]
    fn test_minmax_data_all() {
        const EXPECT: (f32, f32) = (0.0, 499.14);
        let data = get_minmax_all(get_data(&PathBuf::from(NAMEFILE)));
        assert_eq!(
            data, EXPECT,
            "test_minmax_data: {:#?} == {:#?}",
            data, EXPECT
        );
    }
}
