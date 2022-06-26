//! Contains task from leetcode.com

#![allow(non_snake_case)]
#![allow(unused)]

/// 704. Binary Search
mod bin_search;
/// 278. First Bad Version
mod first_bad_version;
/// 35. Search Insert Position
mod search_insert_position;
/// 977. Squares of a Sorted Array
mod squares_of_sorted_array;
/// 189. Rotate Array
mod rotate_array;
/// 283. Move Zeroes
mod move_zeroes;
/// 167. Two Sum II - Input Array Is Sorted
mod two_sum2_input_array_sorted;
/// 344. Reverse String
mod reverse_string;
/// 557. Reverse Words in a String III
mod reverse_words_in_str3;
/// 680. Valid Palindrome II
mod valid_palindrome2;
/// 125. Valid Palindrome
mod valid_palindrome;
/// 876. Middle of the Linked List
mod middle_of_linked_list;
/// 19. Remove Nth Node From End of List
mod remove_nth_node_from_end;
/// 31. Next Permutation
mod next_permutation;
/// 3. Longest Substring Without Repeating Characters
mod longest_substring_witout_repeating;
/// 567. Permutation in String
mod permutation_in_string;
/// 1721. Swapping Nodes in a Linked List
mod swapping_nodes_in_linked_list;
/// 11. Container With Most Water
mod container_with_most_water;
/// 733. Flood Fill
mod flood_fill;
/// 695. Max Area of Island
mod max_area_of_island;
/// 923. 3Sum With Multiplicity
mod sum_with_multiplicity;
/// 617. Merge Two Binary Trees
mod merge_two_binary_trees;
/// 542. 01 Matrix
mod matrix1;
/// 994. Rotting Oranges
mod rotting_oranges;
/// 703. Kth Largest Element in a Stream
mod kth_largest_element_in_stream;
/// 21. Merge Two Sorted Lists
mod merge_two_sorted_lists;
/// 206. Reverse Linked List
mod reverse_linked_list;
/// 77. Combinations
mod combinations;
/// Combinations pairs
mod combinations_pairs;
/// 347. Top K Frequent Elements
mod top_k_frequent_elements;
/// 46. Permutations
mod permutations;
/// 784. Letter Case Permutation
mod letter_case_permutation;
/// 682. Baseball Game
mod baseball_game;
/// 70. Climbing Stairs
mod climbing_stairs;
/// 198. House Robber
mod house_robber;
/// 120. Triangle
mod triangle;
/// 231. Power of Two
mod power_of_two;
/// 191. Number of 1 Bits
mod number_of_one_bit;
/// 1260. Shift 2D Grid
mod shift_2d_grid;
/// 190. Reverse Bits
mod reverse_bits;
/// 136. Single Number
mod single_number;
/// 289. Game of Life
mod game_of_life;
/// 59. Spiral Matrix II
mod spiral_matrix2;
/// 217. Contains Duplicate
mod contains_duplicate;
/// 53. Maximum Subarray
mod maximum_subarray;
/// 700. Search in a Binary Search Tree
mod search_in_binary_search_tree;
/// 1. Two Sum
mod two_sum;
/// 88. Merge Sorted Array
mod merge_two_sorted_array;
/// 669. Trim a Binary Search Tree
mod trim_binary_search_tree;
/// 350. Intersection of Two Arrays II
mod intersection_of_two_arrays2;
/// 121. Best Time to Buy and Sell Stock
mod best_time_to_buy_and_sell_stock;
/// 538. Convert BST to Greater Tree
mod convert_bst_to_greater_tree;
/// 566. Reshape the Matrix
mod reshape_the_matrix;
/// 118. Pascal's Triangle
mod pascals_triangle;
/// 897. Increasing Order Search Tree
mod increase_order_search_tree;
/// 36. Valid Sudoku
mod valid_sudoku;
/// 74. Search a 2D Matrix
mod search_2d_matrix;
/// 230. Kth Smallest Element in a BST
mod kth_smallets_element_in_bst;
/// 387. First Unique Character in a String
mod first_unique_character_in_string;
/// 383. Ransom Note
mod ransome_note;
/// 242. Valid Anagram
mod valid_anagram;
/// 99. Recover Binary Search Tree
mod recover_binary_search_tree;
/// 141. Linked List Cycle
mod linked_list_cycle;
/// 203. Remove Linked List Elements
mod remove_linked_list_elements;
/// 173. Binary Search Tree Iterator
mod binary_search_tree_iterator;
/// 83. Remove Duplicates from Sorted List
mod remove_duplicates_from_sorted_list;
/// 705. Design HashSet
mod design_hashset;
/// 20. Valid Parentheses
mod valid_parentheses;
/// 232. Implement Queue using Stacks
mod implement_queue_using_stack;
/// 706. Design HashMap
mod design_hashmap;
/// 144. Binary Tree Preorder Traversal
mod binary_tree_preorder_traversal;
/// 94. Binary Tree Inorder Traversal
mod binary_tree_inorder_traversal;
/// 145. Binary Tree Postorder Traversal
mod binary_tree_postorder_traversal;
/// 102. Binary Tree Level Order Traversal
mod binary_tree_level_order_traversal;
/// 104. Maximum Depth of Binary Tree
mod maximum_depth_of_binary_tree;
/// 101. Symmetric Tree
mod symmetric_tree;
/// 535. Encode and Decode TinyURL
mod encode_and_decode_tiny_url;
/// 1396. Design Underground System
mod design_underground_system;
/// 226. Invert Binary Tree
mod invert_binary_tree;
/// 112. Path Sum
mod path_sum;
/// 284. Peeking Iterator
mod peeking_iterator;
/// 701. Insert into a Binary Search Tree
mod insert_into_binary_tree;
/// 1584. Min Cost to Connect All Points
mod min_cost_to_connect_all_points;
/// 98. Validate Binary Search Tree
mod validate_binary_search_tree;
/// 653. Two Sum IV - Input is a BST
mod two_sumIV_input_is_bst;
/// 235. Lowest Common Ancestor of a Binary Search Tree
mod lower_common_ancestor_of_bst;
/// 1202. Smallest String With Swaps
mod smallest_string_with_swaps;
/// 1631. Path With Minimum Effort
mod path_with_minimum_effort;
/// 785. Is Graph Bipartite?
mod is_graph_bipartite;
/// 399. Evaluate Division
mod evaluate_division;
/// 844. Backspace String Compare
mod backspace_string_compare;
/// 204. Count Primes
mod count_primes;
/// 34. Find First and Last Position of Element in Sorted Array
mod find_first_and_last_position_of_element_in_sorted_array;
/// 33. Search in Rotated Sorted Array
mod search_in_rotated_sorted_array;
/// 905. Sort Array By Parity
mod sort_array_by_parity;
/// 153. Find Minimum in Rotated Sorted Array
mod find_minimum_in_rotated_sorted_array;
/// 581. Shortest Unsorted Continuous Subarray
mod shortest_unsorted_continuous_subarray;
/// Max Number of K-Sum Pairs
mod max_number_of_ksum_pairs;
/// 225. Implement Stack using Queues
mod implement_stack_using_queues;
/// 82. Remove Duplicates from Sorted List II
mod remove_duplicates_from_sorted_list_2;
/// 1209. Remove All Adjacent Duplicates in String II
mod remove_all_adjacent_duplicates_in_string_2;
/// 15. 3Sum
mod three_sum;
/// 456. 132 Pattern
mod one_three_two_pattern;
/// 986. Interval List Intersections
mod interval_list_intersections;
/// 438. Find All Anagrams in a String
mod find_all_anagrams_in_string;
/// 341. Flatten Nested List Iterator
mod flatten_nested_list_iterator;
/// 713. Subarray Product Less Than K
mod subarray_product_less_than_k;
/// 209. Minimum Size Subarray Sum
mod minimum_size_subarray_sum;
/// 17. Letter Combinations of a Phone Number
mod letter_combinations_of_phone_number;
/// 216. Combination Sum III
mod combination_sum_3;
/// 200. Number of Islands
mod number_of_islands;
/// 547. Number of Provinces
mod number_of_provinces;
/// 117. Populating Next Right Pointers in Each Node II
mod populating_next_right_pointers_in_each_node_2;
/// 572. Subtree of Another Tree
mod subtree_of_another_tree;
/// 1091. Shortest Path in Binary Matrix
mod shortest_path_in_binary_matrix;
/// 130. Surrounded Regions
mod surrounded_regions;
/// 1641. Count Sorted Vowel Strings
mod count_sorted_vowel_string;
/// 797. All Paths From Source to Target
mod all_paths_from_source_to_target;
/// 47. Permutations II
mod permutations2;
/// 52. N-Queens II (HARD)
mod n_queens_2;
/// 743. Network Delay Time
mod network_delay_time;
/// 1302. Deepest Leaves Sum
mod deepest_leaves_sum;
/// 78. Subsets
mod subsets;
/// 90. Subsets II
mod subset2;
/// 39. Combination Sum
mod combination_sum;
/// 40. Combination Sum II
mod combination_sum_2;
/// 79. Word Search
mod word_search;
/// 22. Generate Parentheses
mod generate_parantheses;
/// 213. House Robber II
mod house_robber_2;
/// 55. Jump Game
mod jump_game;
/// 45. Jump Game II
mod jump_game_2;
/// 1192. Critical Connections in a Network (HARD)
mod critical_connections_in_network;
/// 62. Unique Paths
mod unique_paths;
/// 329. Longest Increasing Path in a Matrix
mod longest_increasing_path_in_matrix;
/// 63. Unique Paths II
mod unique_paths_2;
/// 413. Arithmetic Slices
mod arithmetic_slices;
/// 91. Decode Ways
mod decode_ways;
/// 322. Coin Change
mod coin_change;
/// 139. Word Break
mod word_break;
/// 300. Longest Increasing Subsequence
mod longest_increasing_subsequence;
/// 673. Number of Longest Increasing Subsequence
mod number_increasing_subsequence;
/// 647. Palindromic Substrings
mod palindromic_substrings;
/// 622. Design Circular Queue
mod design_circular_queue;
/// 474. Ones and Zeroes (HARD)
mod ones_and_zeroes;
/// 354. Russian Doll Envelopes (HARD)
mod russian_doll_envelopes;
/// 32. Longest Valid Parentheses
mod longest_valid_parentheses;
/// 201. Bitwise AND of Numbers Range
mod bitwise_and_of_numver_range;
/// 318. Maximum Product of Word Lengths
mod maximum_product_of_word_lengths;
/// 752. Open the Lock
mod open_the_lock;
/// 279. Perfect Squares
mod perfect_squares;
/// 384. Shuffle an Array
mod shuffle_an_array;
/// 29. Divide Two Integers
mod divide_two_integers;
/// 1342. Number of Steps to Reduce a Number to Zero
mod number_of_steps_to_reduce_number_to_zero;
/// 268. Missing Number
mod missing_number;
/// 1461. Check If a String Contains All Binary Codes of Size K
mod check_if_string_contains_all_binary_codes_of_size_k;
/// 304. Range Sum Query 2D - Immutable
mod range_sum_query_2d_immutable;
/// 51. N-Queens (HARD)
mod n_queens;
/// 1143. Longest Common Subsequence
mod longest_common_subsequence;
/// 202. Happy Number
mod happy_number;
/// 160. Intersection of Two Linked Lists
mod intersection_of_two_linked_list;
/// 1480. Running Sum of 1d Array
mod running_sum_of_1d_array;
/// 155. Min Stack
mod min_stack;
/// 583. Delete Operation for Two Strings
mod delete_operation_for_two_strings;
/// 1332. Remove Palindromic Subsequences
mod remove_palindromic_subsequence;
/// 72. Edit Distance (HARD)
mod edit_distance;
/// 343. Integer Break
mod integer_break;
/// 149. Max Points on a Line (HARD)
mod max_pointer_line;
/// 1658. Minimum Operations to Reduce X to Zero
mod minimum_operations_to_reduce_x_to_zero;
/// 1695. Maximum Erasure Value
mod maximum_erasure_value;
/// 739. Daily Temperatures
mod daily_temperatures;
/// 1048. Longest String Chain
mod longest_string_chain;
/// 150. Evaluate Reverse Polish Notation
mod evaluate_reverse_polish_notation;
/// 968. Binary Tree Cameras (HARD)
mod binary_tree_cameras;
/// 745. Prefix and Suffix Search (HARD)
mod prefix_and_suffix_search;
/// 820. Short Encoding of Words
mod short_encoding_of_words;
/// 1642. Furthest Building You Can Reach
mod furthest_building_you_can_reach;
/// 215. Kth Largest Element in an Array
mod kth_largest_element_in_array;
/// 9. Palindrome Number
mod palindrome_number;
/// 630. Course Schedule III (HARD)
mod course_schedule_3;
/// 13. Roman to Integer
mod roman_to_integer;
/// 14. Longest Common Prefix
mod longest_common_prefix;
/// 28. Implement strStr()
mod implement_strStr;
/// 27. Remove Element
mod remove_element;
/// 26. Remove Duplicates from Sorted Array
mod remove_duplicates_from_sorted_array;
/// 58. Length of Last Word
mod length_of_last_word;
/// 1354. Construct Target Array With Multiple Sums (HARD)
mod construct_target_array_with_multiple_sums;
/// 66. Plus One
mod plus_one;
/// 67. Add Binary
mod add_binary;
/// 169. Majority Element
mod majority_element;
/// 665. Non-decreasing Array
mod non_decreasing_array;
/// 2299. Strong Password Checker II
mod strong_password_checker;
/// 2185. Counting Words With a Given Prefix
mod counting_word_with_a_given_prefix;
/// 2169. Count Operations to Obtain Zero
mod count_operations_to_obtain_zero;
/// 1678. Goal Parser Interpretation
mod goal_parser_interpretation;
/// 263. Ugly Number
mod ugly_number;
/// 494. Target Sum
mod target_sum;
/// 394. Decode String
mod decode_string;
/// 1423. Maximum Points You Can Obtain from Cards
mod maximum_points_you_can_obtain_from_cards;
/// 1603. Design Parking System
mod design_parking_system;
/// 841. Keys and Rooms
mod keys_and_rooms;
/// 10. Regular Expression Matching (HARD)
mod regular_expression_matching;










