# Terraform

1. To deploy a Rust Tonic gRPC server on AWS using Terraform and integrating it with services like ECS, ECR, IAM, Route 53, API Gateway, and Application Load Balancer (ALB), you can follow the step-by-step process outlined below:

Set up the necessary prerequisites:
- Install Rust and Cargo on your local development machine.
- Install Terraform on your local development machine.
- Set up an AWS account and obtain your access and secret keys.

2. Create a new directory for your project and navigate into it:

```sh
git clone https://github.com/codeitlikemiley/rust-grpc-server
cd rust-grpc-server
```

3. Initialize Terraform in the project directory:

```sh
terraform init
```

4. Create a new file named main.tf and define your Terraform configuration with the necessary AWS resources.
   a. Set up the provider and AWS credentials:

```hcl
provider "aws" {
  access_key = "<your-access-key>"
  secret_key = "<your-secret-key>"
  region     = "us-east-1"  # Replace with your desired region
}
```

b. Create an IAM role and policy to grant necessary permissions to your ECS tasks:

```hcl
resource "aws_iam_role" "ecs_task_role" {
  name = "my-grpc-server-ecs-task-role"

  assume_role_policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Principal": {
        "Service": "ecs-tasks.amazonaws.com"
      },
      "Action": "sts:AssumeRole"
    }
  ]
}
EOF
}

resource "aws_iam_policy" "ecs_task_policy" {
  name        = "my-grpc-server-ecs-task-policy"
  description = "Policy for ECS tasks to access necessary resources"

  policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Action": [
        "logs:CreateLogGroup",
        "logs:CreateLogStream",
        "logs:PutLogEvents"
      ],
      "Resource": "*"
    }
    // Add more necessary permissions here, such as accessing your database or other services
  ]
}
EOF
}

resource "aws_iam_role_policy_attachment" "ecs_task_policy_attachment" {
  role       = aws_iam_role.ecs_task_role.name
  policy_arn = aws_iam_policy.ecs_task_policy.arn
}
```

c. Set up an ECS cluster and task definition:

```hcl
resource "aws_ecs_cluster" "grpc_server_cluster" {
  name = "my-grpc-server-cluster"
}

resource "aws_ecs_task_definition" "grpc_server_task" {
  family                   = "my-grpc-server-task"
  requires_compatibilities = ["FARGATE"]
  cpu                      = "256"
  memory                   = "512"

  network_mode = "awsvpc"

  execution_role_arn = aws_iam_role.ecs_task_role.arn

  container_definitions = <<EOF
[
  {
    "name": "my-grpc-server-container",
    "image": "YOUR_ECR_REPOSITORY_URL",
    "essential": true,
    "portMappings": [
      {
        "containerPort": 50051,
        "protocol": "tcp"
      }
    ],
    "logConfiguration": {
      "logDriver": "awslogs",
      "options": {
        "awslogs-group": "/ecs/my-grpc-server-container",
        "awslogs-region": "us-east-1",
        "awslogs-stream-prefix": "ecs"
      }
    }
  }
]
EOF
}
```

d. Create a new ECS service to manage the deployment of your task:

```hcl
resource "aws_ecs_service" "grpc_server_service" {
  name            = "my-grpc-server-service"
  cluster         = aws_ecs_cluster.grpc_server_cluster.id
  task_definition = aws_ecs_task_definition.grpc_server_task.arn
  desired_count   = 1
}
```

e. Set up an ALB to expose your gRPC service:

```hcl
resource "aws_lb" "grpc_server_lb" {
  name               = "my-grpc-server-lb"
  load_balancer_type = "application"
  subnets            = ["<your-subnet-ids>"]  # Replace with your subnet IDs
}

resource "aws_lb_target_group" "grpc_server_target_group" {
  name     = "my-grpc-server-target-group"
  port     = 50051
  protocol = "TCP"
  vpc_id   = "<your-vpc-id>"  # Replace with your VPC ID
}

resource "aws_lb_listener" "grpc_server_listener" {
  load_balancer_arn = aws_lb.grpc_server_lb.arn
  port              = 50051
  protocol          = "TCP"

  default_action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.grpc_server_target_group.arn
  }
}
```

f. Set up a Route 53 record to point to your ALB:

```hcl
resource "aws_route53_zone" "grpc_server_zone" {
  name = "example.com"  # Replace with your domain name
}

resource "aws_lb_target_group_attachment" "grpc_server_target_group_attachment" {
  target_group_arn = aws_lb_target_group.grpc_server_target_group.arn
  target_id        = aws_ecs_service.grpc_server_service.id
  port             = 50051
}

resource "aws_route53_record" "grpc_server_record" {
  zone_id = aws_route53_zone.grpc_server_zone.zone_id
  name    = "grpc.example.com"  # Replace with your desired hostname
  type    = "A"

  alias {
    name                   = aws_lb.grpc_server_lb.dns_name
    zone_id                = aws_lb.grpc_server_lb.zone_id
    evaluate_target_health = false
  }
}
```

g. Set up an API Gateway to proxy requests to your ALB:

```hcl
resource "aws_apigatewayv2_api" "grpc_server_api" {
  name          = "my-grpc-server-api"
  protocol_type = "HTTP"
  target        = "REGIONAL"
}

resource "aws_apigatewayv2_stage" "grpc_server_stage" {
  api_id      = aws_apigatewayv2_api.grpc_server_api.id
  name        = "dev"
  auto_deploy = true
}

resource "aws_apigatewayv2_integration" "grpc_server_integration" {
  api_id             = aws_apigatewayv2_api.grpc_server_api.id
  integration_type   = "HTTP_PROXY"
  integration_uri    = aws_lb.grpc_server_lb.dns_name
  integration_method = "ANY"
  connection_type    = "INTERNET"
  timeout_milliseconds = 30000
}

resource "aws_apigatewayv2_route" "grpc_server_route" {
  api_id    = aws_apigatewayv2_api.grpc_server_api.id
  route_key = "ANY /{proxy+}"
  target    = "integrations/${aws_apigatewayv2_integration.grpc_server_integration.id}"
}
```

5. Build your Rust Tonic gRPC server using Cargo and Docker:
   a. Build the Docker image for your server:

```sh
docker build -t codeitlikemiley/rust-grpc .
```

b. Push the Docker image to your ECR repository (replace <your-ecr-repository-uri> with your actual ECR repository URI):

```sh
docker tag my-grpc-server:latest <your-ecr-repository-uri>:latest
docker push <your-ecr-repository-uri>:latest
```

Make sure you have set up the ECR repository and have the necessary permissions to push images to it.

6. Apply the Terraform configuration to deploy your gRPC server:

```sh
terraform apply
```

Review the changes that Terraform plans to make and confirm the deployment.
