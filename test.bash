aws lambda invoke \
--cli-binary-format raw-in-base64-out \
--region ap-southeast-2 \
--function-name <your_lambda_function_name> \
--payload '{"first_name": "Leon", "last_name":"Ticharwa"}' \
output.json