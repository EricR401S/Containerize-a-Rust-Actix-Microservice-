# Containerize a Rust Axum/Actix Microservice - Lotto Generator


## Description
In this project, we containerize an Rust Axum web application, which generates random numbers according to the amount of digits that a user wants. I also managed to deploy it to AWS Apprunner, making the microservice accessible via the cloud, versus a local level.

## Why a lottery number generator?

One of the trademark conversation pieces for the elderly community in Puerto Rico actually revolves around playing the lotto games, known as "Pega 3", "Pega 4", and "Lotería Electrónica". I would often see them strike new friendships and conversations about what number combination looks pretty, how it wouldn't hurt to have an extra 200 dollars, and how they would be happy with even with 10,000 dollars, not even a full 100K. I thought it would be nice if I had a number for them for the next time in which I see them. However, I don't want them to use the app if it means them losing the conversation piece. It should be an afterthought if and only if they needed a bit more numerical inspiration.

## Usage

Apprunner Link : https://ja3mrdz8ra.us-east-2.awsapprunner.com/

![Alt text](image-1.png)

![Alt text](image.png)

At the link, add a "/digit" like "/4", to obtain your randomly generated number. Happy generating!!!!

## Prerequisites
For those not using the AWS Cloud 9 environment, the required setup will be installing the following:

* AWS cli (the most recent version)

* Add the permissions to the IAM User/Role for the cloud instance or developer environment. 

    * For the IAM user, I opted for this combination of permissions. The codedeploy ones are likely not necessary, but I didn't test that case.

    ![Alt text](image-3.png)

    * For the AWS Cloud 9 role attached to the instance, I chose these permissions.

    ![Alt text](image-2.png)

## Setup Instructions

1. Make a directory for the project and navigate inside it.
```
$ cargo new your_project
$ cd your_project
```
2. Specify your logic in the lib.rs and the main.rs. Here is a specific look at the web framework : axum.
```
use axum::{routing::get, Router};
use lotto_randomizer::generate_random_number;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(welcome_handler)).route("/:digits", get(generate_random_number_handler));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn welcome_handler() -> &'static str {
    "Welcome to the Random Number Generator! To generate a random number, add the desired number of digits to the URL, e.g., /5 \n\n\n
    Bienvenidos al generador aleatorio de numeros, para inspirar sus opciones de Pega 3 , Pega 4, o cualquier loteria. \n 
    Para obtener el numero deseado de digitos, añada la diagonal y el digito al enlace (e.j. '/5')"
}

async fn generate_random_number_handler(params: axum::extract::Path<(usize,)>) -> String {
    let digits = params.0 .0; // Extract the usize from the tuple
    let random_number = generate_random_number(digits);

    format!("Generated Random Number with {} digits: {}", digits, random_number)
}
```
3. Build the project. Run the project in the terminal. 
```
cargo build
cargo run
```
![Alt text](image-4.png)

4. In a separate terminal session, run "curl 'localhost:3000'" and "curl 'localhost:3000/5'", to see if it works locally. Remember to adjust these commands to the unique functionality of your microservice. 
```
curl 'localhost:3000'
curl 'localhost:3000/5'
```
![Alt text](image-5.png)

5. Terminate your original terminal session running the "cargo run" command with by pressing control+c. Now, Proceed to building your docker. The axum friendly, docker file [template](https://gitlab.com/dukeaiml/duke-coursera-labs/rust-axum-greedy-coin-microservice/-/blob/main/Dockerfile?ref_type=heads) seen here is a courtesy of [Noah Gift](https://noahgift.com/). Then proceed to build the docker image. 
```
docker build -t any_lowercase_name_for_your_image .
```
![Alt text](image-6.png)

6. Run the docker image, and this will be a great opportunity for you to access the port as soon as it runs, as you will be essentially accessing the app from the image. I will also include the stopping commands as comments, as you will need to stop the container after you're done testing. At this point, if you want, you can stop here. 
```
docker run -dp 3000:3000 number_generator
# docker ps # to get the container id
# docker stop <your_container_id>
```
![Alt text](image-7.png)

7. (Optional) Deploy the app to Apprunner : Part 1. We have to configure an ECR repository in AWS. It's an incredibly straightforward process, and I recommend making a public ECR repository. As soon as it is done, look for the push commands for your repository. Then appropriately push the image to the repository. I put my sequence of commands as a "make deploy-aws".

![Alt text](image-8.png)

![Alt text](image-9.png)

![Alt text](image-10.png)

```
deploy-aws:
	cargo build
	aws ecr get-login-password --region us-east-2 | docker login --username AWS --password-stdin 667719398048.dkr.ecr.us-east-2.amazonaws.com
	docker build -t lotto_axum .
	docker tag lotto_axum:latest 667719398048.dkr.ecr.us-east-2.amazonaws.com/lotto_axum:latest
	docker push 667719398048.dkr.ecr.us-east-2.amazonaws.com/lotto_axum:latest
```

8. Final Step :  Configure to run on AWS Apprunner. I recommend the options shown in the image, allowing you to choose your public repository and having Apprunner automatically re-deploy the service as new images are pushed to AWS. As for the role, I was able to leverage the regular "AppRunnerECRAccessRole", without any special permissions for this project. If you needed to automate the process, Github Actions and different types of roles or permissions might be necessary.  

![Alt text](image-12.png)

For this image, I also changed the ports and adjusted the health checks, so that the deployment would not falsely fail quickly.

![Alt text](image-11.png)

The link from your AppRunner, once deployed, is the link to your service, where the end user must add the required information (in my case, backslashes and digits) to access the functionality of the microservice. As for how my microservice appeared, look at my result section.

![Alt text](image-13.png)

## Result

Assignment Requirement - Running the docker locally:

![Alt text](image-7.png)

Here is an example of my deployed app, the lottery number generator.

![Alt text](image-15.png)

![Alt text](image-16.png)

## Licenses
Creative Commons.

## Status
This project is complete as of February 20th, 2024.

## Extra Notes : Considerations with Actix

Running the app with cargo locally works.

![Alt text](image-17.png)

Building the docker, the image does not appear in docker image ls or anything of the sort. When running the image, the GLIBC_2.XX errors come up.  

![Alt text](image-18.png)

The fix was replacing the debian image with debian:testing. However, a new error "curl: (56) Recv failure: Connection reset by peer" crept up that I couldn't debug, and I discovered it was an issue with the port binding, so I changed it to ".bind("0.0.0.0:8080")?". That was the final fix. 

![Alt text](image-19.png)

![Alt text](image-20.png)

other pics:

![Alt text](image-21.png)
![Alt text](image-22.png)

![Alt text](image-23.png)

![Alt text](image-24.png)

https://qpmrrrsni2.us-east-2.awsapprunner.com/

![Alt text](image-25.png)

![Alt text](image-26.png)
