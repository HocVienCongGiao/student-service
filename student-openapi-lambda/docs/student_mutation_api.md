# student_mutation_api

All URIs are relative to *https://dev-sg.portal.hocvienconggiao.com*

Method | HTTP request | Description
------------- | ------------- | -------------
**addStudent**](student_mutation_api.md#addStudent) | **POST** /mutation-api/student-service/students | Add new student
**deleteStudent**](student_mutation_api.md#deleteStudent) | **DELETE** /mutation-api/student-service/students/{id} | Deletes a student
**updateStudent**](student_mutation_api.md#updateStudent) | **PUT** /mutation-api/student-service/students/{id} | Update an existing student


# **addStudent**
> models::StudentUpsert addStudent(ctx, student_upsert)
Add new student

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **student_upsert** | [**StudentUpsert**](StudentUpsert.md)| student object that needs to be added to the database | 

### Return type

[**models::StudentUpsert**](StudentUpsert.md)

### Authorization

[student_auth](../README.md#student_auth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteStudent**
> deleteStudent(ctx, id)
Deletes a student

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | [****](.md)| ID of student to delete | 

### Return type

 (empty response body)

### Authorization

[student_auth](../README.md#student_auth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **updateStudent**
> models::StudentUpsert updateStudent(ctx, id, student_upsert)
Update an existing student

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | [****](.md)| ID of student to update | 
  **student_upsert** | [**StudentUpsert**](StudentUpsert.md)| student object that needs to be added to the database | 

### Return type

[**models::StudentUpsert**](StudentUpsert.md)

### Authorization

[student_auth](../README.md#student_auth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

