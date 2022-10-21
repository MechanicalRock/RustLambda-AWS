
### Build and deploy to AWS

To build and deploy the Rust program, simply run the following command in your terminal
```bash build-deploy.sh```

Once the deployment succeeds, take notice of the outputs section in your cli and note down the name of the lambda function that has just been created in your account. You will need this to send test events to the newly created lambda from your cli. 
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



