use std::fs;



pub struct Report {
    pub levels: Vec<i32>,
}

impl Report {
    pub fn is_safe(&self) -> bool {
        if self.levels.len() < 2 {
            return true;
        }
        let decreasing = self.levels[0] > self.levels[1];
        for i in 0..self.levels.len() - 1 {
            let value = self.levels[i];
            let next_value = self.levels[i + 1];
            if value == next_value {
                return false;
            }
            if (value - next_value).abs() > 3 {
                return false;
            }

            if decreasing && value < next_value {
                return false;
            }
            if !decreasing && value > next_value {
                return false;
            }
        }
        true
    }

    pub fn is_safe_with_dampener(&self) -> bool {
        if self.is_safe() {
            return true;
        }

        for i in 0..self.levels.len() {
            let mut levels = self.levels.clone();
            levels.remove(i);
            let new_report = Report{levels};
            if new_report.is_safe() {
                return true;
            }
        }
        false
    }
}

pub struct ReportList {
    pub reports: Vec<Report>,
}

pub fn read_input() -> ReportList{
    let input = fs::read_to_string("inputs/day_two.txt").unwrap();
    let lines = input.lines().map(|line| line.split(" ").map(|num| num.parse::<i32>().unwrap()).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();
    let reports = lines.iter().map(|line| Report { levels: line.clone() }).collect::<Vec<Report>>();
    ReportList { reports }
}

pub fn count_safe_reports(report_list: ReportList) -> i32 {
    let mut count = 0;
    for report in report_list.reports {
        if report.is_safe() {
            count += 1;
        }
    }
    count
}

pub fn count_safe_reports_with_dampener(report_list: ReportList) -> i32 {
    let mut count = 0;
    for report in report_list.reports {
        if report.is_safe_with_dampener() {
            count += 1;
        }
    }
    count
}
