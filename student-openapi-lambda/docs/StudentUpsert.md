# StudentUpsert

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**polity_id** | [***uuid::Uuid**](UUID.md) |  | [optional] [default to None]
**saint_id_array** | [**Vec<uuid::Uuid>**](UUID.md) |  | [optional] [default to None]
**title** | [***models::StudentTitle**](StudentTitle.md) |  | [optional] [default to None]
**progress** | [***models::VowProgress**](VowProgress.md) |  | [optional] [default to None]
**first_name** | **String** |  | [optional] [default to None]
**middle_name** | **String** |  | [optional] [default to None]
**last_name** | **String** |  | [optional] [default to None]
**date_of_birth** | [***chrono::DateTime::<chrono::Utc>**](date.md) | date of birth in format YYYY-MM-DD | [optional] [default to None]
**place_of_birth** | **String** |  | [optional] [default to None]
**email** | **String** |  | [optional] [default to None]
**phone** | **String** |  | [optional] [default to None]
**address** | **String** |  | [optional] [default to None]
**nationality** | **String** |  | [optional] [default to Some("Vietnamese".to_string())]
**educational_stage** | [**Vec<models::EducationalStage>**](EducationalStage.md) |  | [optional] [default to None]
**foreign_language** | [**Vec<models::ForeignLanguage>**](ForeignLanguage.md) |  | [optional] [default to None]
**race** | **String** |  | [optional] [default to None]
**id_number** | **String** |  | [optional] [default to None]
**id_number_provider** | [***models::IdNumberProvider**](IdNumberProvider.md) |  | [optional] [default to None]
**date_of_issue** | [***chrono::DateTime::<chrono::Utc>**](date.md) |  | [optional] [default to None]
**place_of_issue** | **String** |  | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


