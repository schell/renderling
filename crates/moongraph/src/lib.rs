//! Render graph
use std::{
    any::Any,
    collections::HashMap,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use broomdog::{Loan, LoanMut};
use dagga::Dag;
use snafu::prelude::*;

pub use broomdog::{TypeKey, TypeMap};
pub use dagga::{DaggaError, Node};
pub use moongraph_macros::Edges;

#[derive(Debug, Snafu)]
pub enum GraphError {
    #[snafu(display("Error while running node: {}", error))]
    RunningNode {
        error: Box<dyn std::error::Error + Send + Sync + 'static>,
    },

    #[snafu(display("Error scheduling the graph: {source}"))]
    Scheduling { source: dagga::DaggaError },

    #[snafu(display("Resource error: {source}"))]
    Resource { source: broomdog::BroomdogErr },

    #[snafu(display("Resource is loaned"))]
    Loaned,

    #[snafu(display("Missing resource '{name}'"))]
    Missing { name: &'static str },
}

/// A function wrapper.
///
/// Wraps a function by moving it into a closure. Before running the parameters
/// of the function are constructed from a TypeMap of resources, and the results
/// of the function are packed back into the same TypeMap.
pub struct Function {
    inner: Box<dyn Fn(&mut TypeMap) -> Result<(), GraphError>>,
}

impl Function {
    pub fn run(&mut self, resources: &mut TypeMap) -> Result<(), GraphError> {
        (self.inner)(resources)
    }
}

pub trait Edges: Sized {
    fn reads() -> Vec<TypeKey> {
        vec![]
    }

    fn writes() -> Vec<TypeKey> {
        vec![]
    }

    fn moves() -> Vec<TypeKey> {
        vec![]
    }

    fn construct(resources: &mut TypeMap) -> Result<Self, GraphError>;
}

impl Edges for () {
    fn construct(_: &mut TypeMap) -> Result<Self, GraphError> {
        Ok(())
    }
}

macro_rules! impl_edges {
    ($($t:ident),+) => {
        impl<$($t: Edges),+> Edges for ($($t,)+) {
            fn construct(resources: &mut TypeMap) -> Result<Self, GraphError> {
                Ok((
                    $( $t::construct(resources)?, )+
                ))
            }

            fn reads() -> Vec<TypeKey> {
                vec![
                    $( $t::reads(), )+
                ].concat()
            }

            fn writes() -> Vec<TypeKey> {
                vec![
                    $( $t::writes(), )+
                ].concat()
            }

            fn moves() -> Vec<TypeKey> {
                vec![
                    $( $t::moves(), )+
                ].concat()
            }
        }
    }
}

impl_edges!(A);
impl_edges!(A, B);
impl_edges!(A, B, C);
impl_edges!(A, B, C, D);
impl_edges!(A, B, C, D, E);
impl_edges!(A, B, C, D, E, F);
impl_edges!(A, B, C, D, E, F, G);
impl_edges!(A, B, C, D, E, F, G, H);
impl_edges!(A, B, C, D, E, F, G, H, I);
impl_edges!(A, B, C, D, E, F, G, H, I, J);
impl_edges!(A, B, C, D, E, F, G, H, I, J, K);
impl_edges!(A, B, C, D, E, F, G, H, I, J, K, L);

pub trait NodeResults {
    fn creates() -> Vec<TypeKey>;
    fn save(self, resources: &mut TypeMap) -> Result<(), GraphError>;
}

impl NodeResults for () {
    fn creates() -> Vec<TypeKey> {
        vec![]
    }

    fn save(self, _: &mut TypeMap) -> Result<(), GraphError> {
        Ok(())
    }
}

macro_rules! impl_node_results {
    ($(($t:ident, $n:tt)),+) => {
        impl<$( $t : Any + Send + Sync ),+> NodeResults for ($($t,)+) {
            fn creates() -> Vec<TypeKey> {
                vec![$( TypeKey::new::<$t>() ),+]
            }

            fn save(self, resources: &mut TypeMap) -> Result<(), GraphError> {
                $( let _ = resources.insert_value( self.$n ); )+
                Ok(())
            }
        }
    }
}

impl_node_results!((A, 0));
impl_node_results!((A, 0), (B, 1));
impl_node_results!((A, 0), (B, 1), (C, 2));
impl_node_results!((A, 0), (B, 1), (C, 2), (D, 3));
impl_node_results!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4));
impl_node_results!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5));
impl_node_results!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6));
impl_node_results!(
    (A, 0),
    (B, 1),
    (C, 2),
    (D, 3),
    (E, 4),
    (F, 5),
    (G, 6),
    (H, 7)
);
impl_node_results!(
    (A, 0),
    (B, 1),
    (C, 2),
    (D, 3),
    (E, 4),
    (F, 5),
    (G, 6),
    (H, 7),
    (I, 8)
);
impl_node_results!(
    (A, 0),
    (B, 1),
    (C, 2),
    (D, 3),
    (E, 4),
    (F, 5),
    (G, 6),
    (H, 7),
    (I, 8),
    (J, 9)
);
impl_node_results!(
    (A, 0),
    (B, 1),
    (C, 2),
    (D, 3),
    (E, 4),
    (F, 5),
    (G, 6),
    (H, 7),
    (I, 8),
    (J, 9),
    (K, 10)
);
impl_node_results!(
    (A, 0),
    (B, 1),
    (C, 2),
    (D, 3),
    (E, 4),
    (F, 5),
    (G, 6),
    (H, 7),
    (I, 8),
    (J, 9),
    (K, 10),
    (L, 11)
);

/// Defines render graph nodes.
///
/// A node in the render graph is a function that may create, consume, read or
/// write resources.
pub trait IsGraphNode<Input, Output> {
    fn into_node(self) -> Node<Function, TypeKey>;
}

impl<
        Input: Edges,
        Output: NodeResults,
        F: Fn(Input) -> Result<Output, E> + 'static,
        E: std::error::Error + Send + Sync + 'static,
    > IsGraphNode<Input, Output> for F
{
    fn into_node(self) -> Node<Function, TypeKey> {
        let inner = Box::new(move |resources: &mut TypeMap| {
            let input = Input::construct(resources)?;
            match (self)(input) {
                Ok(creates) => {
                    resources.unify().context(ResourceSnafu)?;
                    creates.save(resources)?;
                    Ok(())
                }
                Err(e) => Err(GraphError::RunningNode { error: Box::new(e) }),
            }
        });
        Node::new(Function { inner })
            .with_reads(Input::reads())
            .with_writes(Input::writes())
            .with_moves(Input::moves())
            .with_results(Output::creates())
    }
}

pub struct Move<T> {
    inner: T,
}

impl<T: Any + Send + Sync> Edges for Move<T> {
    fn moves() -> Vec<TypeKey> {
        vec![TypeKey::new::<T>()]
    }

    fn construct(resources: &mut TypeMap) -> Result<Self, GraphError> {
        let key = TypeKey::new::<T>();
        let inner_loan = resources
            .remove(&key)
            .context(MissingSnafu { name: key.name() })?;
        let value = inner_loan.into_owned(key.name()).context(ResourceSnafu)?;
        // UNWRAP: safe because we got this out as `T`
        let box_t = value.downcast::<T>().unwrap();
        Ok(Move { inner: *box_t })
    }
}

impl<T> Move<T> {
    pub fn into(self) -> T {
        self.inner
    }
}

impl<T> Deref for Move<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for Move<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

pub struct Read<T> {
    inner: Loan,
    _phantom: PhantomData<T>,
}

impl<T: Any + Send + Sync> Deref for Read<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // UNWRAP: safe because it was constructed with `T`
        self.inner.downcast_ref().unwrap()
    }
}

impl<T: Any + Send + Sync> Edges for Read<T> {
    fn reads() -> Vec<TypeKey> {
        vec![TypeKey::new::<T>()]
    }

    fn construct(resources: &mut TypeMap) -> Result<Self, GraphError> {
        let key = TypeKey::new::<T>();
        let inner = resources
            .loan(key)
            .context(ResourceSnafu)?
            .context(MissingSnafu {
                name: std::any::type_name::<T>(),
            })?;
        Ok(Read {
            inner,
            _phantom: PhantomData,
        })
    }
}

pub struct Write<T> {
    inner: LoanMut,
    _phantom: PhantomData<T>,
}

impl<T: Any + Send + Sync> Deref for Write<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // UNWRAP: safe because it was constructed with `T`
        self.inner.downcast_ref().unwrap()
    }
}

impl<T: Any + Send + Sync> DerefMut for Write<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // UNWRAP: safe because it was constructed with `T`
        self.inner.downcast_mut().unwrap()
    }
}

impl<'a, T: Any + Send + Sync> Edges for Write<T> {
    fn writes() -> Vec<TypeKey> {
        vec![TypeKey::new::<T>()]
    }

    fn construct(resources: &mut TypeMap) -> Result<Self, GraphError> {
        let key = TypeKey::new::<T>();
        let inner = resources
            .loan_mut(key)
            .context(ResourceSnafu)?
            .context(MissingSnafu {
                name: std::any::type_name::<T>(),
            })?;
        Ok(Write {
            inner,
            _phantom: PhantomData,
        })
    }
}

#[derive(Default)]
pub struct Graph {
    resources: TypeMap,
    barrier: usize,
    unscheduled: Vec<Node<Function, TypeKey>>,
    schedule: Vec<Vec<Node<Function, TypeKey>>>,
}

impl Graph {
    /// Merge two graphs, preferring the right in cases of key collisions.
    ///
    /// The values of `rhs` will override those of `lhs`.
    pub fn merge(mut lhs: Graph, mut rhs: Graph) -> Graph {
        lhs.unschedule();
        rhs.unschedule();
        let Graph {
            resources: mut rhs_resources,
            unscheduled: rhs_nodes,
            barrier: _,
            schedule: _,
        } = rhs;
        lhs.resources
            .extend(std::mem::take(rhs_resources.deref_mut()).into_iter());
        let mut unscheduled: HashMap<String, Node<Function, TypeKey>> = HashMap::default();
        unscheduled.extend(
            lhs.unscheduled
                .into_iter()
                .map(|node| (node.name().to_string(), node)),
        );
        unscheduled.extend(
            rhs_nodes
                .into_iter()
                .map(|node| (node.name().to_string(), node)),
        );
        lhs.unscheduled = unscheduled.into_iter().map(|(_, node)| node).collect();
        lhs.barrier = lhs.barrier.max(rhs.barrier);
        lhs
    }

    /// Unschedule all functions.
    fn unschedule(&mut self) {
        self.unscheduled
            .extend(std::mem::take(&mut self.schedule).into_iter().flatten());
    }

    // Schedule all functions.
    fn reschedule(&mut self) -> Result<(), GraphError> {
        log::trace!("rescheduling the render graph:");
        self.unschedule();
        let all_nodes = std::mem::take(&mut self.unscheduled);
        let dag = all_nodes
            .into_iter()
            .fold(Dag::default(), |dag, node| dag.with_node(node));
        let schedule = dag.build_schedule().map_err(|dagga::BuildScheduleError{ source, mut dag }| {
            // we have to put the nodes back so the library user can do debugging
            for node in dag.take_nodes() {
                self.add_node(node);
            }
            GraphError::Scheduling { source }
        })?;
        log::trace!("{:#?}", schedule.batched_names());
        self.schedule = schedule.batches;
        Ok(())
    }

    /// An iterator over all nodes.
    pub fn nodes(&self) -> impl Iterator<Item = &Node<Function, TypeKey>> {
        self.schedule
            .iter()
            .flatten()
            .chain(self.unscheduled.iter())
    }

    /// Add a node to the graph.
    pub fn add_node(&mut self, node: Node<Function, TypeKey>) {
        self.unscheduled.push(node.runs_after_barrier(self.barrier));
    }

    /// Return a reference to the node with the given name, if possible.
    pub fn get_node(&self, name: impl AsRef<str>) -> Option<&Node<Function, TypeKey>> {
        for node in self.nodes() {
            if node.name() == name.as_ref() {
                return Some(node);
            }
        }
        None
    }

    /// Remove a node from the graph by name.
    ///
    /// This leaves the graph in an unscheduled state.
    pub fn remove_node(&mut self, name: impl AsRef<str>) -> Option<Node<Function, TypeKey>> {
        self.unschedule();
        let mut may_index = None;
        for (i, node) in self.unscheduled.iter().enumerate() {
            if node.name() == name.as_ref() {
                may_index = Some(i);
            }
        }
        if let Some(i) = may_index.take() {
            Some(self.unscheduled.swap_remove(i))
        } else {
            None
        }
    }

    /// Add a node to the graph.
    pub fn with_node(mut self, node: Node<Function, TypeKey>) -> Self {
        self.add_node(node);
        self
    }

    /// Add a named function to the graph.
    pub fn with_function<Input, Output>(
        mut self,
        name: impl Into<String>,
        f: impl IsGraphNode<Input, Output>,
    ) -> Self {
        self.add_function(name, f);
        self
    }

    /// Add a named function to the graph.
    pub fn add_function<Input, Output>(
        &mut self,
        name: impl Into<String>,
        f: impl IsGraphNode<Input, Output>,
    ) {
        self.add_node(f.into_node().with_name(name));
    }

    /// Return whether the graph contains a function with the given name.
    pub fn contains_function(&self, name: impl AsRef<str>) -> bool {
        let name = name.as_ref();
        let search = |node: &Node<Function, TypeKey>| node.name() == name;
        if self.unscheduled.iter().any(search) {
            return true;
        }
        self.schedule.iter().flatten().any(search)
    }

    /// Return whether the graph contains a resource with the parameterized
    /// type.
    pub fn contains_resource<T: Any + Send + Sync>(&self) -> bool {
        let key = TypeKey::new::<T>();
        self.resources.contains_key(&key)
    }

    /// Explicitly insert a resource (an edge) into the graph.
    ///
    /// This will overwrite an existing resource of the same type in the graph.
    pub fn with_resource<T: Any + Send + Sync>(mut self, t: T) -> Self {
        self.add_resource(t);
        self
    }

    /// Explicitly insert a resource (an edge) into the graph.
    ///
    /// This will overwrite an existing resource of the same type in the graph.
    pub fn add_resource<T: Any + Send + Sync>(&mut self, t: T) {
        // UNWRAP: safe because of the guarantees around `insert_value`
        self.resources.insert_value(t).unwrap();
    }

    /// Add a barrier to the graph.
    ///
    /// All nodes added after the barrier will run after nodes added before the
    /// barrier.
    pub fn add_barrier(&mut self) {
        self.barrier += 1;
    }

    /// Add a barrier to the graph.
    ///
    /// All nodes added after the barrier will run after nodes added before the
    /// barrier.
    pub fn with_barrier(mut self) -> Self {
        self.add_barrier();
        self
    }

    /// Run the graph.
    pub fn run(&mut self) -> Result<(), GraphError> {
        if !self.unscheduled.is_empty() {
            self.reschedule()?;
        }

        // TODO: run batches concurrently
        for batch in self.schedule.iter_mut() {
            for node in batch.iter_mut() {
                node.inner_mut().run(&mut self.resources)?;
            }
        }

        Ok(())
    }

    /// Remove a resource from the graph.
    pub fn remove_resource<T: Any + Send + Sync>(&mut self) -> Result<Option<T>, GraphError> {
        let key = TypeKey::new::<T>();
        if let Some(inner_loan) = self.resources.remove(&key) {
            let value = inner_loan
                .into_owned(key.name())
                .with_context(|_| ResourceSnafu)?;
            let box_t = value.downcast::<T>().ok().with_context(|| LoanedSnafu)?;
            Ok(Some(*box_t))
        } else {
            Ok(None)
        }
    }

    /// Get a reference to a resource in the graph.
    pub fn get_resource<T: Any + Send + Sync>(&self) -> Result<Option<&T>, GraphError> {
        Ok(self.resources.get_value().context(ResourceSnafu)?)
    }

    /// Get a mutable reference to a resource in the graph.
    pub fn get_resource_mut<T: Any + Send + Sync>(&mut self) -> Result<Option<&mut T>, GraphError> {
        Ok(self.resources.get_value_mut().context(ResourceSnafu)?)
    }

    /// Fetch a loanable type and visit it with a closure.
    ///
    /// This is like running a one-off graph node, but `S` does not get packed
    /// into the graph as a result resource, instead it is given back to the
    /// callsite.
    ///
    /// ## Note
    /// By design, visiting the graph with a type that uses `Move` in one of its
    /// fields will result in the type of that field being `move`d **out**
    /// of the graph. The resource will no longer be available within the
    /// graph.
    ///
    /// ```rust
    /// use moongraph::*;
    /// use snafu::prelude::*;
    ///
    /// #[derive(Debug, Snafu)]
    /// enum TestError {}
    ///
    /// #[derive(Edges)]
    /// struct Input {
    ///     num_usize: Read<usize>,
    ///     num_f32: Write<f32>,
    ///     num_f64: Move<f64>,
    /// }
    ///
    /// // pack the graph with resources
    /// let mut graph = Graph::default()
    ///     .with_resource(0usize)
    ///     .with_resource(0.0f32)
    ///     .with_resource(0.0f64);
    ///
    /// // visit the graph, reading, modifying and _moving_!
    /// let num_usize = graph.visit(|mut input: Input| {
    ///     *input.num_f32 = 666.0;
    ///     *input.num_f64 += 10.0;
    ///     *input.num_usize
    /// }).unwrap();
    ///
    /// // observe we read usize
    /// assert_eq!(0, num_usize);
    /// assert_eq!(0, *graph.get_resource::<usize>().unwrap().unwrap());
    ///
    /// // observe we modified f32
    /// assert_eq!(666.0, *graph.get_resource::<f32>().unwrap().unwrap());
    ///
    /// // observe we moved f64 out of the graph and it is no longer present
    /// assert!(!graph.contains_resource::<f64>());
    pub fn visit<T: Edges, S>(&mut self, f: impl FnOnce(T) -> S) -> Result<S, GraphError> {
        let t = T::construct(&mut self.resources)?;
        let s = f(t);
        self.resources.unify().context(ResourceSnafu)?;
        Ok(s)
    }

    #[cfg(feature = "dot")]
    pub fn save_graph_dot(&self, path: &str) {
        use dagga::dot::DagLegend;

        let legend = DagLegend::new(self.nodes()).with_resources_named(|ty: &TypeKey| {
            ty.name().to_string()
        });
        legend.save_to(path).unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn create(_: ()) -> Result<(usize,), GraphError> {
        Ok((0,))
    }

    fn edit((mut num,): (Write<usize>,)) -> Result<(), GraphError> {
        *num += 1;
        Ok(())
    }

    fn finish((num,): (Move<usize>,)) -> Result<(), GraphError> {
        assert_eq!(1, num.into(), "edit did not run");
        Ok(())
    }

    #[test]
    fn function_to_node() {
        // sanity test
        let mut graph = Graph::default()
            .with_function("create", create)
            .with_function("edit", edit);
        graph.run().unwrap();
        assert_eq!(1, *graph.get_resource::<usize>().unwrap().unwrap());

        let mut graph = graph.with_function("finish", finish);
        graph.run().unwrap();
        assert!(graph.get_resource::<usize>().unwrap().is_none());
    }

    #[test]
    fn many_inputs_many_outputs() {
        // tests our Edges and NodeResults impl macros
        fn start(_: ()) -> Result<(usize, u32, f32, f64, &'static str, String), GraphError> {
            Ok((0, 0, 0.0, 0.0, "hello", "HELLO".into()))
        }

        fn modify_ints(
            (mut numusize, mut numu32): (Write<usize>, Write<u32>),
        ) -> Result<(), GraphError> {
            *numusize += 1;
            *numu32 += 1;
            Ok(())
        }

        fn modify_floats(
            (mut numf32, mut numf64): (Write<f32>, Write<f64>),
        ) -> Result<(), GraphError> {
            *numf32 += 10.0;
            *numf64 += 10.0;
            Ok(())
        }

        fn modify_strings(
            (mut strstatic, mut strowned): (Write<&'static str>, Write<String>),
        ) -> Result<(), GraphError> {
            *strstatic = "goodbye";
            *strowned = "GOODBYE".into();
            Ok(())
        }

        fn end(
            (nusize, nu32, nf32, nf64, sstatic, sowned): (
                Move<usize>,
                Move<u32>,
                Move<f32>,
                Move<f64>,
                Move<&'static str>,
                Move<String>,
            ),
        ) -> Result<(bool,), GraphError> {
            assert_eq!(1, *nusize);
            assert_eq!(1, *nu32);
            assert_eq!(10.0, *nf32);
            assert_eq!(10.0, *nf64);
            assert_eq!("goodbye", *sstatic);
            assert_eq!("GOODBYE", *sowned);
            Ok((true,))
        }

        let mut graph = Graph::default()
            .with_function("start", start)
            .with_function("modify_ints", modify_ints)
            .with_function("modify_floats", modify_floats)
            .with_function("modify_strings", modify_strings)
            .with_function("end", end);

        graph.run().unwrap();
        let run_was_all_good = graph.get_resource::<bool>().unwrap().unwrap();
        assert!(run_was_all_good, "run was not all good");

        // TODO: make (re)scheduling explicit so we can check the schedule before
        // running
        let schedule = graph
            .schedule
            .iter()
            .map(|batch| {
                batch
                    .iter()
                    .map(|node| node.name())
                    .collect::<Vec<_>>()
                    .join(", ")
            })
            .collect::<Vec<_>>();
        assert_eq!(
            vec!["start", "modify_strings, modify_floats, modify_ints", "end"],
            schedule,
            "schedule is wrong"
        );
    }

    #[test]
    fn can_derive() {
        use crate as moongraph;

        #[derive(Debug, Snafu)]
        enum TestError {}

        #[derive(Edges)]
        struct Input {
            num_usize: Read<usize>,
            num_f32: Write<f32>,
            num_f64: Move<f64>,
        }

        type Output = (String, &'static str);

        fn start(_: ()) -> Result<(usize, f32, f64), TestError> {
            Ok((1, 0.0, 10.0))
        }

        fn end(mut input: Input) -> Result<Output, TestError> {
            *input.num_f32 += *input.num_f64 as f32;
            Ok((
                format!("{},{},{}", *input.num_usize, *input.num_f32, *input.num_f64),
                "done",
            ))
        }

        let mut graph = Graph::default()
            .with_function("start", start)
            .with_function("end", end);
        graph.run().unwrap();
        assert_eq!(
            "1,10,10",
            graph.get_resource::<String>().unwrap().unwrap().as_str()
        );
    }

    #[test]
    fn can_visit_and_then_borrow() {
        use crate as moongraph;

        #[derive(Debug, Snafu)]
        enum TestError {}

        #[derive(Edges)]
        struct Input {
            num_usize: Read<usize>,
            num_f32: Write<f32>,
            num_f64: Move<f64>,
        }

        let mut graph = Graph::default()
            .with_resource(0usize)
            .with_resource(0.0f32)
            .with_resource(0.0f64);
        let num_usize = graph
            .visit(|mut input: Input| {
                *input.num_f32 = 666.0;
                *input.num_f64 += 10.0;
                *input.num_usize
            })
            .unwrap();
        assert_eq!(0, num_usize);
        assert_eq!(0, *graph.get_resource::<usize>().unwrap().unwrap());
        assert_eq!(666.0, *graph.get_resource::<f32>().unwrap().unwrap());
        assert!(!graph.contains_resource::<f64>());
    }
}
