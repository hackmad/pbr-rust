//! BVH Common

#![allow(dead_code)]
use crate::core::geometry::*;
use crate::core::pbrt::*;
use std::sync::Arc;

/// Splitting method to use to subdivide primitives.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SplitMethod {
    /// Surface Area Heuristic.
    SAH,

    /// Hierarchical Linear Bounding Volume Hierarchy. Morton-curve-based
    /// clustering is used to first build trees for the lower levels of the
    /// hierarchy (`treelets`) and the top levels of the tree are then created
    /// using the surface area heuristic.
    HLBVH,

    /// Linear Bounding Volume Hierarchy using splitting planes that are
    /// midpoint of each region of space.
    Middle,

    /// Partition primitives into equally sized subsets such that the first half
    /// of the primitives have smallest centroid coordinate values along the
    /// chosen axis, and second have have the largest centroid coordinate values.
    EqualCounts,
}

/// SAH bucket information.
#[derive(Copy, Clone, Debug, Default)]
pub struct BucketInfo {
    /// Count of primitives.
    pub count: usize,

    /// Bounding box for the bucket.
    pub bounds: Bounds3f,
}

/// Stores information about a primitive.
#[derive(Copy, Clone, Debug)]
pub struct BVHPrimitiveInfo {
    /// Index in the `BVHAccel::primitives`.
    pub primitive_number: usize,

    /// The bounding box of primitive.
    pub bounds: Bounds3f,

    /// The centroid of bounding box of primitive.
    pub centroid: Point3f,
}

impl BVHPrimitiveInfo {
    /// Create a `BVHPrimitiveInfo`.
    ///
    /// * `primitive_number` - Index in the `BVHAccel::primitives`.
    /// * `bounds`           - The bounding box of primitive.
    pub fn new(primitive_number: usize, bounds: Bounds3f) -> Self {
        Self {
            primitive_number,
            bounds,
            centroid: 0.5 * (bounds.p_min + bounds.p_max),
        }
    }
}

/// BVHBuildNode represents a node of the Bound Volume Hierarchy.
#[derive(Clone, Default)]
pub struct BVHBuildNode {
    /// Bounding box of all children beneath this node.
    pub bounds: Bounds3f,

    /// Children of this node.
    pub children: [Option<Arc<BVHBuildNode>>; 2],

    /// Coordinate axis along which primitives are partitioned between the
    /// two children.
    pub split_axis: Axis,

    /// Index of first primitive from `BVHAccel::primitives` stored at this node.
    pub first_prim_offset: usize,

    /// Number of primitives stored from `BVHAccel::primitives` stored at this
    /// node` starting at `first_prim_offset` but not including
    /// `first_prim_offset` + `n_primitives`.
    pub n_primitives: usize,
}

impl BVHBuildNode {
    /// Create a leaf BVH node.
    ///
    /// * `first`  - Index of first primitive from `BVHAccel::primitives` stored at
    ///              this node.
    /// * `n`      - Number of primitives stored from `BVHAccel::primitives` stored
    ///              at this node` starting at `first` but not including `first` + `n`.
    /// * `bounds` - Bounding box.
    pub fn new_leaf_node(first: usize, n: usize, bounds: Bounds3f) -> Arc<Self> {
        Arc::new(Self {
            first_prim_offset: first,
            n_primitives: n,
            bounds,
            children: [None, None],
            split_axis: Axis::default(),
        })
    }

    /// Allocates an interior BVH node.
    ///
    /// * `axis` - Axis used for partitioning children.
    /// * `c0`   - First child.
    /// * `c1`   - Second child.
    pub fn new_interior_node(
        axis: Axis,
        c0: Arc<BVHBuildNode>,
        c1: Arc<BVHBuildNode>,
    ) -> Arc<Self> {
        Arc::new(Self {
            first_prim_offset: 0,
            n_primitives: 0,
            bounds: c0.bounds.union(&c1.bounds),
            children: [Some(c0), Some(c1)],
            split_axis: axis,
        })
    }
}

/// Stores information needed to traverse the BVH.
#[derive(Copy, Clone, Default, Debug)]
pub struct LinearBVHNode {
    /// Bounding box for the node.
    pub bounds: Bounds3f,

    /// For leaf nodes, offset for the primitives in the node.
    /// For interior nodes, offset to the second child.
    pub offset: u32,

    /// For leaf nodes, the number of primitives in the node.
    /// For interior nodes, 0.
    pub n_primitives: u16,

    /// For interior nodes, which coordinate axis was used for partitioning.
    pub axis: u8,

    /// Padding used to align everything to 32 byte total size.
    pub pad: u8,
}

impl LinearBVHNode {
    /// Creates a leaf linear bvh node.
    ///
    /// * `bounds`      - Bounding box for the node.
    /// * `offset`      - Offset for primitives in the node.
    /// * `n_primitives - Number of primitives in the node.
    pub fn new_leaf_node(bounds: Bounds3f, offset: u32, n_primitives: u16) -> Self {
        Self {
            bounds,
            offset,
            n_primitives,
            axis: 0,
            pad: 0,
        }
    }

    /// Creates an interior linear bvh node.
    ///
    /// * `bounds` - Bounding box for the node.
    /// * `offset` - Offset to the second child.
    /// * `axis`   - Axis used for partitioning.
    pub fn new_interior_node(bounds: Bounds3f, offset: u32, axis: u8) -> Self {
        Self {
            bounds,
            offset,
            axis,
            n_primitives: 0,
            pad: 0,
        }
    }
}
