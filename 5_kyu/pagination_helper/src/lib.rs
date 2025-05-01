// SPDX-FileCopyrightText: 2025 Vladimir Rusinov
// SPDX-License-Identifier: Apache-2.0

// PaginationHelper
// https://www.codewars.com/kata/515bb423de843ea99400000a/train/rust


// add fields for your implementation to use
struct PaginationHelper {
    // allows the program to compile, but remove after adding your fields
    items_per_page: usize,
    size: usize,
}

impl PaginationHelper {
    fn new<T>(collection: Vec<T>, items_per_page: usize) -> Self {
        return Self {
            items_per_page,
            size: collection.len(),
        };
    }

    fn item_count(&self) -> usize {
        return self.size;
    }

    fn page_count(&self) -> usize {
        // Round up integer division
        return (self.size + self.items_per_page - 1) / self.items_per_page;
    }

    fn page_item_count(&self, page_index: usize) -> Option<usize> {
        let page_count = self.page_count();
        if page_index >= page_count {
            return None;
        }
        if page_index == page_count - 1 {
            let last_page = self.size % self.items_per_page;
            if last_page == 0 {
                return Some(self.items_per_page);
            }
            return Some(last_page);
        }
        return Some(self.items_per_page);
    }

    fn page_index(&self, item_index: usize) -> Option<usize> {
        if item_index >= self.size {
            return None;
        }
        return Some(item_index / self.items_per_page);
    }
}
