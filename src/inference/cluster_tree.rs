use crate::inference::FactorGraph;
use std::sync::Arc;

/// A cluster is just a collection of factors
#[derive(Debug)]
pub struct Cluster<Graph: FactorGraph> {
    frontals: Vec<u64>,
    children: Vec<Box<Self>>,
    factors: Graph,
}

/// A Cluster Tree is associated with a Factor graph and is defined as in Koller-Friedman:
/// Each node k represents a subset $`C_k \sub X`$, and the tree is family preserving, in that each factor
/// $`f_i`$ is associated with a single cluster and $`\text{scope}(f_i) \sub C_k`$.
#[derive(Debug)]
pub struct ClusterTree<Graph: FactorGraph> {
    roots: Vec<Box<Cluster<Graph>>>,
}

impl<Graph> Cluster<Graph>
where
    Graph: FactorGraph,
{
    fn new() -> Self {
        Cluster {
            frontals: vec![],
            children: vec![],
            factors: Graph::new(),
        }
    }

    fn from_single_key(key: u64, factors: &Vec<Arc<Graph::FactorType>>) -> Self {
        let mut fg = Graph::new();
        for f in factors {
            fg.insert_shared(f.clone());
        }
        Cluster {
            frontals: vec![key],
            children: vec![],
            factors: fg,
        }
    }

    fn merge(&mut self, other: &mut Self) {
        self.frontals.append(&mut other.frontals);
        self.factors.merge(&mut other.factors);
        self.children.append(&mut other.children);
    }

    /// Add a child cluster
    /// Apparently this operation taints the Cluster
    /// as the cluster frontals will be wrong
    fn add_child(&mut self, child: Box<Self>) {
        self.children.push(child);
    }

    /// Do the merge
    /// This fixes a tainted cluster
    fn merge_children(&mut self, merge: Vec<bool>) {
        let mut old_children = std::mem::replace(&mut self.children, Vec::new());

        let mut j = 0;
        for mut child in old_children {
            if merge[j] {
                self.merge(&mut *child);
            } else {
                self.add_child(child);
            }
            j += 1;
        }

        self.frontals.reverse();
    }
}

impl<Graph> ClusterTree<Graph>
where
    Graph: FactorGraph,
{
    fn new() -> Self {
        ClusterTree { roots: Vec::new() }
    }
}

#[cfg(test)]
mod tests {
    use crate::inference::cluster_tree::{Cluster, ClusterTree};
    use crate::inference::factor_graph::*;
    use std::default::Default;

    use crate::inference::factor_graph::tests::TestFactor;

    #[test]
    fn test_cluster_tree() {
        let mut cluster = Cluster::<SimpleFactorGraph<TestFactor>>::new();
        println!("{:?}", cluster);

        let mut cluster_tree = ClusterTree::<SimpleFactorGraph<TestFactor>>::new();
        println!("{:?}", cluster_tree);

        let mut factors = SimpleFactorGraph::new();
        factors.insert(TestFactor {
            inner: "cluster0".into(),
            _keys: [0].into(),
        });
        let mut cluster_1 =
            Cluster::<SimpleFactorGraph<TestFactor>>::from_single_key(0, &factors.factors);
        println!("{:#?}", cluster_1)
    }

    #[test]
    fn test_merge_add() {
        let mut factors_0 = SimpleFactorGraph::new();
        factors_0.insert(TestFactor {
            inner: "cluster0".into(),
            _keys: [0].into(),
        });
        let mut cluster_0 =
            Cluster::<SimpleFactorGraph<TestFactor>>::from_single_key(0, &mut factors_0.factors);

        let mut factors_1 = SimpleFactorGraph::new();
        factors_1.insert(TestFactor {
            inner: "cluster1".into(),
            _keys: [1].into(),
        });
        let mut cluster_1 =
            Cluster::<SimpleFactorGraph<TestFactor>>::from_single_key(1, &mut factors_1.factors);

        cluster_0.merge(&mut cluster_1);

        println!("Merged = {:#?}", cluster_0);

        let mut factors_2 = SimpleFactorGraph::new();
        factors_2.insert(TestFactor {
            inner: "cluster2".into(),
            _keys: [2].into(),
        });

        let mut cluster_2 =
            Cluster::<SimpleFactorGraph<TestFactor>>::from_single_key(2, &mut factors_2.factors);

        cluster_0.add_child(Box::new(cluster_2));

        println!("Inserted = {:#?}", cluster_0);

        let mut factors_3 = SimpleFactorGraph::new();
        factors_3.insert(TestFactor {
            inner: "cluster3".into(),
            _keys: [3].into(),
        });

        let mut cluster_3 =
            Cluster::<SimpleFactorGraph<TestFactor>>::from_single_key(3, &mut factors_3.factors);

        cluster_0.add_child(Box::new(cluster_3));

        cluster_0.merge_children([true, true].into());

        println!("merge_children = {:?}", cluster_0);
        assert_eq!(cluster_0.frontals, vec![3, 2, 1, 0]);
        assert_eq!(cluster_0.factors.size(), 4);
    }
}
