echo "Start generating open api"
java -jar openapi-generator-cli.jar generate -i contracts/academics/openapi/student.yaml \
-g rust-server -o ../student-openapi-lambda -t custom_templates/reqwest --model-package model