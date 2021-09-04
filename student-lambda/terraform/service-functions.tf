module "students" {
  source = "git::ssh://git@github.com/HocVienCongGiao/terraform-infra.git//skeleton/services/service-function"
  service_name = var.service_name
  query_api_parent_id = module.student-service.query_api_gateway_resource_id
  mutation_api_parent_id = module.student-service.mutation_api_gateway_resource_id
    
  function_name = "students"
  file_name     = var.service_name

  depends_on = [
    module.student-service
   ]
    
  environment = var.environment
  db_host              = var.db_host
  db_user              = var.db_user
  db_password          = var.db_password
  db_name              = var.db_name
}

module "student_id" {
  source = "git::ssh://git@github.com/HocVienCongGiao/terraform-infra.git//skeleton/services/service-function"
  service_name = var.service_name
  query_api_parent_id = module.students.query_api_gateway_resource_id
  mutation_api_parent_id = module.students.mutation_api_gateway_resource_id
    
  function_name = "student_id"
  file_name     = var.service_name
  path_part     = "{id}"
  depends_on = [
    module.students
   ]
    
  environment = var.environment
  db_host              = var.db_host
  db_user              = var.db_user
  db_password          = var.db_password
  db_name              = var.db_name
}

