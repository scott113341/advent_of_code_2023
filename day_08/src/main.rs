mod data;
mod input;

use data::*;
use input::*;
use std::collections::HashMap;

fn main() {
    println!("day: 08");
    println!("  part 1: {}", part_1(get_input(Input::Real)));
    println!("  part 2: {}", part_2(get_input(Input::Real)));
}

// Start at AAA, and follow the left/right instructions. How many steps are required to reach ZZZ?
fn part_1(lines: Vec<String>) -> usize {
    let (instructions, nodes) = parse_data(&lines);

    let mut instruction_count = 0;
    let mut current_node = &nodes["AAA"];

    loop {
        let instruction = instructions[instruction_count % instructions.len()];
        current_node = match instruction {
            'L' => &nodes[&current_node.left],
            'R' => &nodes[&current_node.right],
            _ => unreachable!(),
        };

        instruction_count += 1;

        if current_node.name == "ZZZ" {
            return instruction_count;
        }
    }
}

// Start at every node that ends with A and follow all paths at the same time until they all
// simultaneously end up at nodes that end with Z. How many steps?
fn part_2(lines: Vec<String>) -> usize {
    let (instructions, nodes) = parse_data(&lines);

    let mut instruction_count = 0;
    let mut current_nodes = nodes
        .iter()
        .filter_map(|(node_ref, node)| match node_ref.ends_with('A') {
            true => Some(node),
            false => None,
        })
        .collect::<Vec<&Node>>();

    // HashMap<node name, (has cycled, instruction count)
    let mut cycle_counts = current_nodes
        .iter()
        .map(|node| ((**node).clone(), (false, 0)))
        .collect::<HashMap<Node, _>>();

    loop {
        let instruction = instructions[instruction_count % instructions.len()];
        for node in current_nodes.iter_mut() {
            *node = match instruction {
                'L' => &nodes[&node.left],
                'R' => &nodes[&node.right],
                _ => unreachable!(),
            };
        }

        instruction_count += 1;

        for (idx, (_node, (has_cycled, cycle_instruction_count))) in
            cycle_counts.iter_mut().enumerate()
        {
            let current_node = current_nodes[idx];

            if !*has_cycled {
                *cycle_instruction_count += 1;

                if current_node.name.ends_with('Z') {
                    *has_cycled = true;
                }
            }
        }

        let all_have_cycled = cycle_counts.iter().all(|(_, (has_cycled, _))| *has_cycled);
        if all_have_cycled {
            break;
        }
    }

    cycle_counts
        .values()
        .map(|(_has_cycled, ins_count)| *ins_count)
        .fold(1, num::integer::lcm)
}

fn parse_data(lines: &[String]) -> (Vec<char>, HashMap<NodeRef, Node>) {
    let instructions = lines[0].chars().collect();

    let nodes = lines[2..]
        .iter()
        .map(|line| parse_node(line))
        .map(|node| (node.name, node))
        .collect();

    (instructions, nodes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_input(Input::Test1)), 6);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(get_input(Input::Test2)), 6);
    }
}
