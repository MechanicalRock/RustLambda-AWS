
### Build and deploy to AWS

```bash build-deploy```
Once the deployment succeeds, take notice of the outputs section in your cli and note down the name of the lambda function that has just been deployed. You will need this to test in your cli. 
### Test Lambda Function 

Open the `test-event.sh` script and replace `<your_lambda_function_name>` with the name of your lambda function as noted in the Build and Deploy section above. Once that is done, you can now send a test event to the lambda using the following command:
``` bash test-event.sh```

If the event is successfully processed, output that resembles the one shown below will be stored in a filed called `output.json`

~~~json
   {
   "message":"Hi Leon Ticharwa , welcome to rust in the cloud!",
   "request_id":"47f4f611-f9fa-4bca-a88b-3c89dbf47465"
   }
~~~



