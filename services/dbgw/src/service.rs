use autometrics::autometrics;
use job::job_runner_server::JobRunner;
use job::{Empty, JobList, JobReply, JobRequest};
use tonic::{Request, Response, Status};

pub mod job {
    tonic::include_proto!("job");
}

#[derive(Debug, Default)]
pub struct MyJobRunner {}

#[tonic::async_trait]
#[autometrics]
impl JobRunner for MyJobRunner {
    async fn send_job(&self, request: Request<JobRequest>) -> Result<Response<JobReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = job::JobReply {
            message: format!("Hello {}!", request.into_inner().name).into(),
        };

        Ok(Response::new(reply))
    }

    async fn list_jobs(&self, request: Request<Empty>) -> Result<Response<JobList>, Status> {
        println!("Got a request: {:?}", request);

        let reply = job::JobList {
            job: vec![job::Job {
                id: 1,
                name: "test".into(),
            }],
        };

        Ok(Response::new(reply))
    }
}
