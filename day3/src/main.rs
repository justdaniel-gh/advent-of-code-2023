use std::{cell::RefCell, rc::Rc};

use utils::{BoxIter, Grid, GridCoordinateIter, StaticGrid};

#[derive(Default)]
struct ANumber {
    value: u32,
    counted: bool,
}

#[derive(Default, Clone)]
struct EngineCell {
    num: Option<Rc<RefCell<ANumber>>>,
    is_symbol: bool,
    is_asterisk: bool,
}

impl EngineCell {
    fn new_empty() -> EngineCell {
        EngineCell {
            num: None,
            is_symbol: false,
            is_asterisk: false,
        }
    }
    fn new_symbol(is_asterisk: bool) -> EngineCell {
        EngineCell {
            num: None,
            is_symbol: true,
            is_asterisk,
        }
    }
    fn new_num(num: Rc<RefCell<ANumber>>) -> EngineCell {
        EngineCell {
            num: Some(num),
            is_symbol: false,
            is_asterisk: false,
        }
    }
}

fn parser(s: String) -> StaticGrid<EngineCell> {
    let mut num_rows = 0;
    let mut num_cols = 0;
    let mut cells: Vec<EngineCell> = vec![];
    for row in s.split('\n') {
        let mut cur_num = None;
        cells.extend(row.chars().enumerate().map(|(c_ndx, v)| match v {
            '.' => {
                cur_num = None;
                EngineCell::new_empty()
            }
            '0'..='9' => {
                if cur_num.is_none() {
                    // Build a single ref to the full number, store refcells to it
                    // max num length is 3
                    // NB This would fail on a two digit number at the edge of the grid...
                    cur_num = Some(Rc::new(RefCell::new(ANumber {
                        value: row
                            .get(c_ndx..c_ndx + 3)
                            .unwrap()
                            .replace(|c: char| !c.is_ascii_digit(), "")
                            .parse::<u32>()
                            .unwrap(),
                        counted: false,
                    })));
                }
                EngineCell::new_num(cur_num.as_ref().unwrap().clone())
            }
            '*' => {
                cur_num = None;
                EngineCell::new_symbol(true)
            }
            _ => {
                cur_num = None;
                EngineCell::new_symbol(false)
            }
        }));
        if num_cols == 0 {
            num_cols = cells.len();
        }
        num_rows += 1;
    }
    StaticGrid {
        cells,
        num_rows,
        num_cols,
    }
}

fn solve(grid: &StaticGrid<EngineCell>) -> u32 {
    // 1. Go through all of the cells, looking for a symbol
    // 2. Look all around it for a number
    // 3. Has the number been counted?
    // 3.t Skip
    // 3.f Get the full number, accumulate it, mark each num as counted
    let mut running_total = 0;
    for cell in GridCoordinateIter::new(grid.first_cell_coord(), grid.last_cell_coord()) {
        if grid.get_cell(cell.x, cell.y).unwrap().is_symbol {
            for surround_cell in BoxIter::new(
                grid,
                &utils::CardinalDirection::North,
                utils::ClockDirection::Clockwise,
                cell.x,
                cell.y,
            ) {
                if let Some(cell_ref) = &surround_cell.num {
                    if !cell_ref.borrow().counted {
                        running_total += cell_ref.borrow().value;
                        cell_ref.borrow_mut().counted = true;
                    }
                }
            }
        }
    }
    running_total
}

fn solve2(grid: &StaticGrid<EngineCell>) -> u32 {
    // 1. Go through all of the cells, looking for an *
    // 2. Look all around it for 2 numbers
    // 3. Create both numbers, multiply them, accumulate it
    let mut running_total = 0;
    for cell in GridCoordinateIter::new(grid.first_cell_coord(), grid.last_cell_coord()) {
        if grid.get_cell(cell.x, cell.y).unwrap().is_asterisk {
            let mut adjacent_numbers: Vec<u32> = vec![];
            for surround_cell in BoxIter::new(
                grid,
                &utils::CardinalDirection::North,
                utils::ClockDirection::Clockwise,
                cell.x,
                cell.y,
            ) {
                if let Some(cell_ref) = &surround_cell.num {
                    if !cell_ref.borrow().counted {
                        adjacent_numbers.push(cell_ref.borrow().value);
                        cell_ref.borrow_mut().counted = true;
                    }
                }
            }
            if adjacent_numbers.len() == 2 {
                // Exactly two numbers!
                running_total += adjacent_numbers[0] * adjacent_numbers[1];
            }
        }
    }
    running_total
}

fn main() {
    let grid = utils::load_puzzle_data(3, parser);
    let engine_part_num_sum = solve(&grid);
    println!("Solution 1: Sum of part numbers: {}", engine_part_num_sum);

    // Reset numbers to not counted
    grid.cell_iter().for_each(|c| { if let Some(cell_ref) = &c.num { cell_ref.borrow_mut().counted = false; } });

    let engine_part_num_sum = solve2(&grid);
    println!("Solution 2: Sum of gear ratios: {}", engine_part_num_sum);
}

#[cfg(test)]
mod tests {
    use crate::{parser, solve, solve2};

    #[test]
    fn test_puzzle() {
        let test_data = utils::load_puzzle_test(3, 1, parser);
        let solution = solve(&test_data);
        assert_eq!(solution, 4361);
    }

    #[test]
    fn test_puzzle2() {
        let test_data = utils::load_puzzle_test(3, 2, parser);
        let solution = solve2(&test_data);
        assert_eq!(solution, 467835);
    }
}
