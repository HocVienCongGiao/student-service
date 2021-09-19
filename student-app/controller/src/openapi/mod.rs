pub mod student;
pub mod student_view;

pub trait ToOpenApi<T> {
    fn to_openapi(self) -> T;
}
