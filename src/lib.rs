#![feature(box_patterns)]

// Definition for a binary tree node.
 #[derive(Debug, PartialEq, Eq)]
 pub struct TreeNode {
   pub val: i32,
   pub left: Option<Rc<RefCell<TreeNode>>>,
   pub right: Option<Rc<RefCell<TreeNode>>>,
 }

 impl TreeNode {
   #[inline]
   pub fn new(val: i32) -> Self {
     TreeNode {
       val,
       left: None,
       right: None
     }
   }
 }

extern crate regex;
extern crate core;

use std::rc::Rc;
use std::cell::RefCell;

mod buddy;
#[macro_use]
mod macros;
mod help_your_granny;
mod bleatrix_trotter;
mod last_digit;
mod candy;
mod add_two_numbers;
mod masking_personal_information;
mod repeated_times;
mod max_subarray_sum_circular;
mod find_max_consecutive_ones;
mod delete_node;
mod min_distance;
mod fraction_addition;
mod find_min;
mod permute;
mod recent_counter;
mod string_compress;
mod flipgame;

#[cfg(test)]
mod tests {}
