use lambda_http::lambda;
use {{cookiecutter.project_slug}}::hello;


fn main() {
    lambda!(hello)
}