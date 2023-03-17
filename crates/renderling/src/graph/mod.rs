//! Render graph
use std::{
    any::Any,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use broomdog::{Loan, LoanMut, TypeKey, TypeMap};
use dagga::{Dag, Node};
use snafu::prelude::*;

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
    unscheduled: Vec<Node<Function, TypeKey>>,
    schedule: Vec<Vec<Node<Function, TypeKey>>>,
}

impl Graph {
    pub fn with_function<Input, Output>(
        mut self,
        name: impl Into<String>,
        f: impl IsGraphNode<Input, Output>,
    ) -> Self {
        self.unscheduled.push(f.into_node().with_name(name));
        self
    }

    pub fn run(&mut self) -> Result<(), GraphError> {
        if !self.unscheduled.is_empty() {
            log::trace!("rescheduling the render graph:");
            let all_nodes = std::mem::take(&mut self.unscheduled).into_iter().chain(
                std::mem::take(&mut self.schedule)
                    .into_iter()
                    .flat_map(|batches| batches),
            );
            let dag = all_nodes.fold(Dag::default(), |dag, node| dag.with_node(node));
            let schedule = dag.build_schedule().context(SchedulingSnafu)?;
            log::trace!("{:#?}", schedule.batched_names());
            self.schedule = schedule.batches;
        }

        // TODO: run batches concurrently
        for batch in self.schedule.iter_mut() {
            for node in batch.iter_mut() {
                node.inner_mut().run(&mut self.resources)?;
            }
        }

        Ok(())
    }

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

    pub fn get_resource<T: Any + Send + Sync>(&self) -> Result<Option<&T>, GraphError> {
        Ok(self.resources.get_value().context(ResourceSnafu)?)
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
}
