use lambda_http::lambda;
use {{cookiecutter.project_slug}}::goodbye;


fn main() {
    lambda!(goodbye)
}