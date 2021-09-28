pub mod student_upsert;
pub mod student_view;

pub trait ToOpenApi<T> {
    fn to_openapi(self) -> T;
}
