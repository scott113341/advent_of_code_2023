mod data;
mod input;

use data::*;
use input::*;
use std::collections::{HashMap, HashSet};
use PipeType::*;

fn main() {
    println!("day: 10");
    println!("  part 1: {}", part_1(get_input(Input::Real)));
    println!("  part 2: {}", part_2(get_input(Input::Real)));
}

// How many steps along the loop does it take to get from the starting position to farthest point from the start?
fn part_1(lines: Vec<String>) -> usize {
    let grid = Grid::build(&lines);
    let mut loop_pipes: HashMap<Node, usize> = HashMap::new();
    let mut current_pipes: Vec<Node> = Vec::with_capacity(2);
    let mut step_count = 0;

    // Starting location
    loop_pipes.insert(grid.start_pipe, 0);
    current_pipes.push(grid.start_pipe);

    // Travel around loop
    while !current_pipes.is_empty() {
        let mut next_pipes = Vec::with_capacity(2);

        for pipe in current_pipes.iter() {
            loop_pipes.insert(*pipe, step_count);

            next_pipes.append(
                &mut grid
                    .connected_pipes(pipe)
                    .into_iter()
                    .filter(|n| !loop_pipes.contains_key(n))
                    .collect(),
            );
        }

        current_pipes = next_pipes;
        step_count += 1;
    }

    step_count - 1
}

// How many tiles are enclosed by the loop?
fn part_2(lines: Vec<String>) -> usize {
    let grid = Grid::build(&lines);
    let mut pipes_in_loop = HashSet::new();
    let mut nodes_on_inside = HashSet::new();
    let mut nodes_on_outside = HashSet::new();

    // Travel loop to build a set of nodes that are actually part of the loop, since there are "junk" bits of pipe
    let mut current_pipe = Some(grid.start_pipe);
    while let Some(pipe) = current_pipe {
        pipes_in_loop.insert(pipe);

        let next_pipe = grid
            .connected_pipes(&pipe)
            .into_iter()
            .find(|n| !pipes_in_loop.contains(n));

        current_pipe = next_pipe;
    }

    // Raytrace each node to the "outside". If the ray crosses an odd number of loop pipe segments, the point is inside
    // the loop.
    for row in 0..grid.rows {
        for col in 0..grid.cols {
            let row = row as isize;
            let col = col as isize;
            let node = Node { row, col };

            // Skip tracing to nodes that ARE part of the loop
            if pipes_in_loop.contains(&node) {
                continue;
            }

            // Count the number of pipes crossed from a ray coming in from left to right. Only count pipes that when
            // traversed, actually enter or exit you from the loop. For example:
            //
            //   F-----7*     Tracing to * never actually enters the loop
            //   |  *  |      Tracing to * enters & never exits the loop
            //   |  F--J*     Tracing to * enters the loop at | and exits at J
            //   |  |*        Tracing to * enters and exists the loop
            //   L--J*        Tracing to * never actually enters the loop, but we count as enter + exit so it's fine
            let mut pipe_walls_crossed = 0;
            let ray_row = row;
            for ray_col in -1..col {
                let ray_node = Node {
                    row: ray_row,
                    col: ray_col,
                };

                // While tracing, disregard nodes NOT part of the loop
                if !pipes_in_loop.contains(&ray_node) {
                    continue;
                }

                // Only count vertical, "L", and "J" pipes
                pipe_walls_crossed += match grid.pipes[&ray_node] {
                    Vertical | UpRightBend | UpLeftBend => 1,
                    _ => 0,
                };
            }

            let on_inside = pipe_walls_crossed % 2 == 1;
            if on_inside {
                nodes_on_inside.insert(node);
            } else {
                nodes_on_outside.insert(node);
            }
        }
    }

    nodes_on_inside.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_input(Input::Test1)), 8);
    }

    #[test]
    fn test_part_2_test_2() {
        assert_eq!(part_2(get_input(Input::Test2)), 4);
    }

    #[test]
    fn test_part_2_test_3() {
        assert_eq!(part_2(get_input(Input::Test3)), 8);
    }

    #[test]
    fn test_part_2_test_4() {
        assert_eq!(part_2(get_input(Input::Test4)), 10);
    }
}
