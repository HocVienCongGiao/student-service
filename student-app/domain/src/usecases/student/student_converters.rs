// use crate::boundaries::db_gateway_boundary::{
//     DbError, PolityDbResponse, StudentDbResponse, StudentMutationDbRequest,
// };
// use crate::boundaries::usecase_boundary::{
//     PolityUsecaseOutput, StudentMutationUsecaseInput, StudentUsecaseOutput, UsecaseError,
// };
// use crate::entities::student::Student as StudentEntity;
// use crate::usecases::ToEntity;
//
// impl PolityDbResponse {
//     pub(crate) fn to_usecase_output(&self) -> PolityUsecaseOutput {
//         PolityUsecaseOutput {
//             id: self.id,
//             name: self.name.clone(),
//             location_name: self.location_name.clone(),
//             location_address: self.location_address.clone(),
//             location_email: self.location_email.clone(),
//         }
//     }
// }
