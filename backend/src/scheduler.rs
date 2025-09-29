use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::{Dasharr, services::user_stats::scrape_indexers::scrape_indexers};

pub async fn run_periodic_tasks(arc: Arc<Dasharr>) -> Result<(), Box<dyn std::error::Error>> {
    let sched = JobScheduler::new().await?;

    let job1 = match Job::new_async("0 0 */6 * * *", move |_uuid, _l| {
        let pool_1 = Arc::clone(&arc.pool);
        Box::pin(async move {
            if let Err(e) = scrape_indexers(&pool_1).await {
                eprintln!("Error in periodic task scrape_indexers: {e}");
            }
        })
    }) {
        Ok(job) => job,
        Err(e) => {
            return Err(format!("Error creating job for task scrape_indexers: {e}").into());
        }
    };
    sched.add(job1).await?;

    sched.start().await?;

    Ok(())
}
