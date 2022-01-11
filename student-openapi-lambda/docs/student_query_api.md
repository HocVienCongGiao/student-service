# student_query_api

All URIs are relative to *https://dev-sg.portal.hocvienconggiao.com*

Method | HTTP request | Description
------------- | ------------- | -------------
**getStudentById**](student_query_api.md#getStudentById) | **GET** /query-api/student-service/students/{id} | Find student by ID
**getStudents**](student_query_api.md#getStudents) | **GET** /query-api/student-service/students | Get all students


# **getStudentById**
> models::StudentUpsert getStudentById(ctx, id)
Find student by ID

Returns a single student

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | [****](.md)| ID of student to return | 

### Return type

[**models::StudentUpsert**](StudentUpsert.md)

### Authorization

[student_auth](../README.md#student_auth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getStudents**
> models::StudentViewCollection getStudents(ctx, optional)
Get all students

Returns all students

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**|  | 
 **email** | **String**|  | 
 **phone** | **String**|  | 
 **undergraduate_school** | **String**|  | 
 **date_of_birth** | **chrono::DateTime::<chrono::Utc>**|  | 
 **place_of_birth** | **String**|  | 
 **polity_name** | **String**|  | 
 **specialism** | **String**|  | 
 **sorts** | [**models::StudentSortCriteria**](models::StudentSortCriteria.md)| to sort +displayName for ASC and -displayName for DESC | 
 **offset** | **i32**| The number of students to skip before starting to collect the result set. | 
 **count** | **i32**| The number of students to return. | [default to 20]

### Return type

[**models::StudentViewCollection**](StudentViewCollection.md)

### Authorization

[student_auth](../README.md#student_auth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

