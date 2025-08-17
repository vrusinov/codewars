// SPDX-FileCopyrightText: 2025 Vladimir Rusinov
// SPDX-License-Identifier: Apache-2.0

// Sort binary tree by levels
// https://www.codewars.com/kata/52bef5e3588c56132c0003bc/rust

mod preloaded;
use preloaded::Node;

fn tree_by_levels(root: &Node) -> Vec<u32> {
    let mut result = Vec::new();
    let mut queue = vec![root];

    while !queue.is_empty() {
        let current_node = queue.remove(0);
        result.push(current_node.value);

        if let Some(left) = &current_node.left {
            queue.push(left);
        }
        if let Some(right) = &current_node.right {
            queue.push(right);
        }
    }

    result
}
