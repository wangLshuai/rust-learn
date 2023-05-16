// use super::AveragedCollection;
// use super::*;
// use crate::chapter17_object_oriented::AveragedCollection;
use chapter17_object_oriented::AveragedCollection;
fn main() {
    let mut averagecollection = AveragedCollection::new(vec![], 0.0);

    averagecollection.add(5);

    averagecollection.add(6);

    println!("averagecollection.average: {}", averagecollection.average());
}
