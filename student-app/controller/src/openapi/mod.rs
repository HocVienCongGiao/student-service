pub mod student;

pub trait ToOpenApi<T> {
    fn to_openapi(self) -> T;
}
