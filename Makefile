env=dev
bounded_context=academics
entity_name=student
gh_pat=`cat ~/.peterbean/.gh_pat`
organisation=HocVienCongGiao
contract_repo=hvcg-contracts

clean:
	rm -fr ./contracts
	rm -fr *-openapi

download-contracts: clean
	mkdir -p ./contracts/$(bounded_context)/openapi
	curl -s \
    -H "Authorization: token $(gh_pat)" \
  -X GET \
  -H "Accept: application/vnd.github.v3+json" \
  https://api.github.com/repos/$(organisation)/$(contract_repo)/contents/contracts/academics/openapi/student.yaml | jq ".download_url" --raw-output | xargs curl -s -X GET > ./contracts/${bounded_context}/openapi/${entity_name}.yaml

unpack:	download-contracts
	echo "hello world $(env)"
	mkdir -p $(entity_name)-openapi
	java -jar ./bin/openapi-generator-cli.jar generate -i ./contracts/$(bounded_context)/openapi/$(entity_name).yaml -g rust-server -o $(entity_name)-openapi --package-name=hvcg_$(bounded_context)_openapi_$(entity_name) -c ./bin/config.yaml --type-mappings=date=NaiveDate

	
