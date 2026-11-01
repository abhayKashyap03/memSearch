//! Run with: cargo test -p memory -- --ignored

use memory::fusion::{reciprocal_rank_fusion, weighted_rrf};

#[test]
#[ignore = "implement reciprocal_rank_fusion"]
fn item_ranked_well_in_both_lists_wins() {
    let semantic = vec![10, 20, 30];
    let keyword = vec![20, 40, 10];
    let fused = reciprocal_rank_fusion(&[semantic, keyword], 60.0);
    // 20 is #2 and #1, 10 is #1 and #3: 20 should edge out 10.
    assert_eq!(fused[0].id, 20);
    assert_eq!(fused[1].id, 10);
    // Union of both lists, no duplicates.
    assert_eq!(fused.len(), 4);
}

#[test]
#[ignore = "implement reciprocal_rank_fusion"]
fn empty_input_gives_empty_output() {
    assert!(reciprocal_rank_fusion(&[], 60.0).is_empty());
    assert!(reciprocal_rank_fusion(&[vec![], vec![]], 60.0).is_empty());
}

#[test]
#[ignore = "implement weighted_rrf"]
fn zero_weight_ignores_a_list() {
    let a = vec![1, 2];
    let b = vec![3, 4];
    let fused = weighted_rrf(&[a, b], &[1.0, 0.0], 60.0);
    assert_eq!(fused[0].id, 1);
}
