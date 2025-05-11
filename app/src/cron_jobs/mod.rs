use actix_jobs::{Job, Scheduler, run_forever};
use delete_expired_auth_tokens_job::DeleteExpiredAuthTokensJob;
mod delete_expired_auth_tokens_job;

pub struct CronJob;

impl CronJob {
    fn jobs() -> Vec<Box<dyn Job>> {
        vec![Box::new(DeleteExpiredAuthTokensJob)]
    }

    pub fn start() {
        let mut scheduler = Scheduler::new();

        for job in Self::jobs() {
            scheduler.add(job);
        }

        run_forever(scheduler);
    }
}
