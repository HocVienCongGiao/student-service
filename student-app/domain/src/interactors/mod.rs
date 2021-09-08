mod student_converters;
pub mod student_mutation;
pub mod student_query;

pub(crate) trait ToEntity<T> {
    fn to_entity(self) -> T;
}
