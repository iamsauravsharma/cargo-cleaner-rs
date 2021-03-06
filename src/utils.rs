use std::{collections::HashMap, fs, path::Path};

use anyhow::Result;
use colored::Colorize;

// remove semver version part from crates full name
pub(crate) fn clear_version_value(full_name: &str) -> (String, String) {
    let version_split: Vec<&str> = full_name.split('-').collect();
    let mut version_start_position = version_split.len();
    // check a split part to check from where a semver start for crate
    for (pos, split_part) in version_split.iter().enumerate() {
        if semver::Version::parse(split_part).is_ok() {
            version_start_position = pos;
            break;
        }
    }
    let (clear_name_vec, version_vec) = version_split.split_at(version_start_position);
    let clear_name = clear_name_vec.join("-");
    let version = version_vec.join("-");
    (clear_name, version)
}

// delete folder with folder path provided
pub(crate) fn delete_folder(path: &Path, dry_run: bool) -> Result<()> {
    if path.exists() {
        if path.is_file() {
            if dry_run {
                println!(
                    "{} {} {:?}",
                    "Dry run:".color("yellow"),
                    "Removed".color("red"),
                    path
                );
            } else {
                fs::remove_file(&path)?;
            }
        } else if path.is_dir() {
            if dry_run {
                println!(
                    "{} {} {:?}",
                    "Dry run:".color("yellow"),
                    "Removed".color("red"),
                    path
                );
            } else {
                fs::remove_dir_all(path)?;
            }
        }
    }
    Ok(())
}

// delete index .cache file
pub(crate) fn delete_index_cache(index_dir: &Path, dry_run: bool) -> Result<()> {
    for entry in fs::read_dir(index_dir)? {
        let registry_dir = entry?.path();
        for folder in fs::read_dir(registry_dir)? {
            let folder = folder?.path();
            let folder_name = folder.file_name().unwrap();
            if folder_name == ".cache" {
                delete_folder(&folder, dry_run)?;
            }
        }
    }
    Ok(())
}

//  get size of directory
pub(crate) fn get_size(path: &Path) -> Result<u64> {
    let mut total_size = 0;
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry_path = entry?.path();
            if entry_path.is_dir() {
                total_size += get_size(&entry_path)?;
            } else {
                total_size += entry_path.metadata()?.len();
            }
        }
    } else {
        total_size += path.metadata()?.len();
    }
    Ok(total_size)
}

#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
pub(crate) fn convert_pretty(num: u64) -> String {
    if num == 0 {
        return "0 B".to_string();
    }
    let num = num as f64;
    let units = ["B", "kB", "MB", "GB", "TB"];
    let factor = (num.log10() / 3_f64).floor();
    let power_factor = if factor >= units.len() as f64 {
        (units.len() - 1) as f64
    } else {
        factor
    };
    let pretty_bytes = format!("{:.3}", num / 1000_f64.powf(power_factor))
        .parse::<f64>()
        .unwrap();
    let unit = units[power_factor as usize];
    format!("{} {}", pretty_bytes, unit)
}

// show title
pub(crate) fn show_title(
    title: &str,
    first_path_len: usize,
    second_path_len: usize,
    dash_len: usize,
) {
    print_dash(dash_len);
    println!(
        "|{:^first_width$}|{:^second_width$}|",
        title.bold(),
        "SIZE(MB)".bold(),
        first_width = first_path_len,
        second_width = second_path_len
    );
    print_dash(dash_len);
}

// show total count using data and size
pub(crate) fn show_total_count(
    data: &[String],
    size: f64,
    first_path_len: usize,
    second_path_len: usize,
    dash_len: usize,
) {
    if data.is_empty() {
        println!(
            "|{:^first_width$}|{:^second_width$}|",
            "NONE".color("red"),
            "0.000".color("red"),
            first_width = first_path_len,
            second_width = second_path_len,
        );
    }
    print_dash(dash_len);
    println!(
        "|{:^first_width$}|{:^second_width$}|",
        format!("Total no of crates:- {}", data.len()).color("blue"),
        format!("{:.3}", size).color("blue"),
        first_width = first_path_len,
        second_width = second_path_len,
    );
    print_dash(dash_len);
}

// print dash
pub(crate) fn print_dash(len: usize) {
    println!("{}", "-".repeat(len).color("green"));
}

// top_crates() help to list out top n crates
pub(crate) fn show_top_number_crates(
    crates: &HashMap<String, u64>,
    crate_type: &str,
    number: usize,
) {
    // sort crates by size
    let mut vector = crates.iter().collect::<Vec<_>>();
    vector.sort_by(|a, b| (b.1).cmp(a.1));
    let top_number = std::cmp::min(vector.len(), number);
    let title = format!("Top {} {}", top_number, crate_type);
    let first_path_len = 40;
    let second_path_len = 10;
    let dash_len = first_path_len + second_path_len + 3;
    show_title(title.as_str(), first_path_len, second_path_len, dash_len);
    // check n size and determine if to print n number of output NONE for 0 crates
    if vector.is_empty() {
        println!("|{:^40}|{:^10}|", "NONE".color("red"), "0.000".color("red"));
    } else {
        (0..top_number).for_each(|i| print_index_value_crate(&vector, i));
    }
    print_dash(dash_len);
}

// print crate name
#[allow(clippy::cast_precision_loss)]
pub(crate) fn print_index_value_crate(vector: &[(&String, &u64)], i: usize) {
    let crate_name = vector[i].0;
    let size = vector[i].1;
    let size = (*size as f64) / 1000_f64.powi(2);
    println!("|{:^40}|{:^10.3}|", crate_name, size);
}

fn query_param_widths() -> (usize, usize) {
    (50, 10)
}

pub(crate) fn query_full_width() -> usize {
    let (a, b) = query_param_widths();
    a + b + 1
}

pub(crate) fn query_print(first_param: &str, second_param: &str) {
    let (first_path_width, second_path_width) = query_param_widths();
    println!(
        "{:first_width$} {:>second_width$}",
        first_param,
        second_param,
        first_width = first_path_width,
        second_width = second_path_width
    );
}

#[cfg(test)]
mod test {
    use super::{clear_version_value, convert_pretty};

    #[test]
    fn test_clear_version_value() {
        assert_eq!(
            clear_version_value("sample_crate-0.12.0"),
            ("sample_crate".to_string(), "0.12.0".to_string())
        );
        assert_eq!(
            clear_version_value("another-crate-name-1.4.5"),
            ("another-crate-name".to_string(), "1.4.5".to_string())
        );
        assert_eq!(
            clear_version_value("crate-name-12-123-0.1.0"),
            ("crate-name-12-123".to_string(), "0.1.0".to_string())
        );
        assert_eq!(
            clear_version_value("complex_name-12.0.0-rc.1"),
            ("complex_name".to_string(), "12.0.0-rc.1".to_string())
        );
        assert_eq!(
            clear_version_value("build-number-2.3.4+was0-5"),
            ("build-number".to_string(), "2.3.4+was0-5".to_string())
        );
        assert_eq!(
            clear_version_value("complex_spec-0.12.0-rc.1+name0.4.6"),
            (
                "complex_spec".to_string(),
                "0.12.0-rc.1+name0.4.6".to_string()
            )
        );
    }

    #[test]
    fn test_convert_pretty() {
        assert_eq!(convert_pretty(0), "0 B".to_string());
        assert_eq!(convert_pretty(12), "12 B".to_string());
        assert_eq!(convert_pretty(1234), "1.234 kB".to_string());
        assert_eq!(convert_pretty(23908), "23.908 kB".to_string());
        assert_eq!(convert_pretty(874940334), "874.94 MB".to_string());
        assert_eq!(convert_pretty(8849909404), "8.85 GB".to_string());
        assert_eq!(convert_pretty(3417849409404), "3.418 TB".to_string());
        assert_eq!(
            convert_pretty(93453982182159417),
            "93453.982 TB".to_string()
        );
    }
}
