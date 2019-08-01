mod recipes;

use recipes::algo_2_sort_vec::*;
use recipes::algorithms_1_rnd::*;

mod book;
use book::chapter5_references::*;

fn main() {
    cook_1_random_numbers();
    cook_2_random_numbers_within_range();
    cook_2_random_bool();
    cook_2_roll_the_dice();
    cook_3_random_with_normal_distribution();
    cook_4_random_values_of_custom_tuple();
    cook_5_random_custom_type();
    cook_6_random_password_from_alfanum_characters();
    cook_7_random_password_from_userdefined_characters();

    cook_1_2_sort_a_vector_of_integers();
    cook_1_2_sort_a_vector_of_floats();
    cook_1_2_sort_a_vector_of_structs();

    hashmap_with_mutable_references();
    dereferencing();
    mutable_references();
    assigning_references(false);
    assigning_references(true);
    references_to_references();
    assigning_to_reference_makes_it_point_to_another_value();
    references_to_arbitrary_expressions();
    borrow_a_local_variable_is_forbidden();
    reference_lifetmes_good_example();
    store_globally(&122);
    store_globally2();
    store_globally3();
    structs_containing_references();
}
