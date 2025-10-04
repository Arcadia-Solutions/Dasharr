use crate::{Dasharr, error::Result};
use actix_web::{
    HttpResponse,
    web::{Data, Path},
};

#[utoipa::path(
    get,
    operation_id = "Get indexer's metrics in prometheus format",
    tag = "Indexers",
    path = "/api/indexers/{name}/prometheus",
    params(
        ("name" = String, Path, description = "Indexer name, as displayed in the webui settings")
    ),
    responses(
        (status = 200, description = "Successfully got the indexer prometheus metrics", body=String),
    )
)]
pub async fn exec(arc: Data<Dasharr>, path: Path<String>) -> Result<HttpResponse> {
    let indexer = arc.pool.find_indexer_by_name(&path.into_inner()).await?;
    let indexer_id = indexer.id;
    let indexer_name = indexer.name.clone();
    let profile = indexer.scrape().await?;

    let metrics = format!(
        "# HELP dasharr_indexer_uploaded_bytes Total uploaded bytes for indexer
# TYPE dasharr_indexer_uploaded_bytes counter
dasharr_indexer_uploaded_bytes{{indexer_id=\"{}\",indexer_name=\"{}\"}} {}

# HELP dasharr_indexer_uploaded_real_bytes Real uploaded bytes for indexer
# TYPE dasharr_indexer_uploaded_real_bytes counter
dasharr_indexer_uploaded_real_bytes{{indexer_id=\"{}\",indexer_name=\"{}\"}} {}

# HELP dasharr_indexer_downloaded_bytes Total downloaded bytes for indexer
# TYPE dasharr_indexer_downloaded_bytes counter
dasharr_indexer_downloaded_bytes{{indexer_id=\"{}\",indexer_name=\"{}\"}} {}

# HELP dasharr_indexer_downloaded_real_bytes Real downloaded bytes for indexer
# TYPE dasharr_indexer_downloaded_real_bytes counter
dasharr_indexer_downloaded_real_bytes{{indexer_id=\"{}\",indexer_name=\"{}\"}} {}

# HELP dasharr_indexer_ratio Current ratio for indexer
# TYPE dasharr_indexer_ratio gauge
dasharr_indexer_ratio{{indexer_id=\"{}\",indexer_name=\"{}\"}} {}

# HELP dasharr_indexer_required_ratio Required ratio for indexer
# TYPE dasharr_indexer_required_ratio gauge
dasharr_indexer_required_ratio{{indexer_id=\"{}\",indexer_name=\"{}\"}} {}

# HELP dasharr_indexer_rank_uploaded Rank by uploaded bytes
# TYPE dasharr_indexer_rank_uploaded gauge
dasharr_indexer_rank_uploaded{{indexer_id=\"{}\",indexer_name=\"{}\"}} {}

# HELP dasharr_indexer_rank_downloaded Rank by downloaded bytes
# TYPE dasharr_indexer_rank_downloaded gauge
dasharr_indexer_rank_downloaded{{indexer_id=\"{}\",indexer_name=\"{}\"}} {}

# HELP dasharr_indexer_rank_uploads Rank by uploads
# TYPE dasharr_indexer_rank_uploads gauge
dasharr_indexer_rank_uploads{{indexer_id=\"{}\",indexer_name=\"{}\"}} {}

# HELP dasharr_indexer_rank_requests Rank by requests
# TYPE dasharr_indexer_rank_requests gauge
dasharr_indexer_rank_requests{{indexer_id=\"{}\",indexer_name=\"{}\"}} {}

# HELP dasharr_indexer_rank_bounty Rank by bounty
# TYPE dasharr_indexer_rank_bounty gauge
dasharr_indexer_rank_bounty{{indexer_id=\"{}\",indexer_name=\"{}\"}} {}

# HELP dasharr_indexer_rank_posts Rank by posts
# TYPE dasharr_indexer_rank_posts gauge
dasharr_indexer_rank_posts{{indexer_id=\"{}\",indexer_name=\"{}\"}} {}

# HELP dasharr_indexer_rank_artists Rank by artists
# TYPE dasharr_indexer_rank_artists gauge
dasharr_indexer_rank_artists{{indexer_id=\"{}\",indexer_name=\"{}\"}} {}

# HELP dasharr_indexer_rank_overall Overall rank
# TYPE dasharr_indexer_rank_overall gauge
dasharr_indexer_rank_overall{{indexer_id=\"{}\",indexer_name=\"{}\"}} {}

# HELP dasharr_indexer_class User class for indexer
# TYPE dasharr_indexer_class gauge
dasharr_indexer_class{{indexer_id=\"{}\",indexer_name=\"{}\",class=\"{}\"}} 1

# HELP dasharr_indexer_donor Donor status
# TYPE dasharr_indexer_donor gauge
dasharr_indexer_donor{{indexer_id=\"{}\",indexer_name=\"{}\"}} {}

# HELP dasharr_indexer_warned Warning status
# TYPE dasharr_indexer_warned gauge
dasharr_indexer_warned{{indexer_id=\"{}\",indexer_name=\"{}\"}} {}

# HELP dasharr_indexer_posts Number of posts
# TYPE dasharr_indexer_posts gauge
dasharr_indexer_posts{{indexer_id=\"{}\",indexer_name=\"{}\"}} {}

# HELP dasharr_indexer_torrent_comments Number of torrent comments
# TYPE dasharr_indexer_torrent_comments gauge
dasharr_indexer_torrent_comments{{indexer_id=\"{}\",indexer_name=\"{}\"}} {}

# HELP dasharr_indexer_collages_started Number of collages started
# TYPE dasharr_indexer_collages_started gauge
dasharr_indexer_collages_started{{indexer_id=\"{}\",indexer_name=\"{}\"}} {}

# HELP dasharr_indexer_collages_contrib Number of collages contributed to
# TYPE dasharr_indexer_collages_contrib gauge
dasharr_indexer_collages_contrib{{indexer_id=\"{}\",indexer_name=\"{}\"}} {}

# HELP dasharr_indexer_requests_filled Number of requests filled
# TYPE dasharr_indexer_requests_filled gauge
dasharr_indexer_requests_filled{{indexer_id=\"{}\",indexer_name=\"{}\"}} {}

# HELP dasharr_indexer_requests_voted Number of requests voted on
# TYPE dasharr_indexer_requests_voted gauge
dasharr_indexer_requests_voted{{indexer_id=\"{}\",indexer_name=\"{}\"}} {}

# HELP dasharr_indexer_uploaded_torrents Number of uploaded torrents
# TYPE dasharr_indexer_uploaded_torrents gauge
dasharr_indexer_uploaded_torrents{{indexer_id=\"{}\",indexer_name=\"{}\"}} {}

# HELP dasharr_indexer_groups Number of groups
# TYPE dasharr_indexer_groups gauge
dasharr_indexer_groups{{indexer_id=\"{}\",indexer_name=\"{}\"}} {}

# HELP dasharr_indexer_seeding_torrents Number of seeding torrents
# TYPE dasharr_indexer_seeding_torrents gauge
dasharr_indexer_seeding_torrents{{indexer_id=\"{}\",indexer_name=\"{}\"}} {}

# HELP dasharr_indexer_leeching_torrents Number of leeching torrents
# TYPE dasharr_indexer_leeching_torrents gauge
dasharr_indexer_leeching_torrents{{indexer_id=\"{}\",indexer_name=\"{}\"}} {}

# HELP dasharr_indexer_snatched Number of snatched torrents
# TYPE dasharr_indexer_snatched gauge
dasharr_indexer_snatched{{indexer_id=\"{}\",indexer_name=\"{}\"}} {}

# HELP dasharr_indexer_invited Number of users invited
# TYPE dasharr_indexer_invited gauge
dasharr_indexer_invited{{indexer_id=\"{}\",indexer_name=\"{}\"}} {}

# HELP dasharr_indexer_bonus_points Current bonus points
# TYPE dasharr_indexer_bonus_points gauge
dasharr_indexer_bonus_points{{indexer_id=\"{}\",indexer_name=\"{}\"}} {}

# HELP dasharr_indexer_bonus_points_per_hour Bonus points per hour rate
# TYPE dasharr_indexer_bonus_points_per_hour gauge
dasharr_indexer_bonus_points_per_hour{{indexer_id=\"{}\",indexer_name=\"{}\"}} {}

# HELP dasharr_indexer_seed_size Total seed size in bytes
# TYPE dasharr_indexer_seed_size gauge
dasharr_indexer_seed_size{{indexer_id=\"{}\",indexer_name=\"{}\"}} {}

# HELP dasharr_indexer_average_seed_time Average seed time in seconds
# TYPE dasharr_indexer_average_seed_time gauge
dasharr_indexer_average_seed_time{{indexer_id=\"{}\",indexer_name=\"{}\"}} {}
",
        indexer_id,
        indexer_name,
        profile.uploaded,
        indexer_id,
        indexer_name,
        profile.uploaded_real.unwrap_or(0),
        indexer_id,
        indexer_name,
        profile.downloaded,
        indexer_id,
        indexer_name,
        profile.downloaded_real.unwrap_or(0),
        indexer_id,
        indexer_name,
        profile.ratio,
        indexer_id,
        indexer_name,
        profile.required_ratio.unwrap_or(0.0),
        indexer_id,
        indexer_name,
        profile.rank_uploaded.unwrap_or(0),
        indexer_id,
        indexer_name,
        profile.rank_downloaded.unwrap_or(0),
        indexer_id,
        indexer_name,
        profile.rank_uploads.unwrap_or(0),
        indexer_id,
        indexer_name,
        profile.rank_requests.unwrap_or(0),
        indexer_id,
        indexer_name,
        profile.rank_bounty.unwrap_or(0),
        indexer_id,
        indexer_name,
        profile.rank_posts.unwrap_or(0),
        indexer_id,
        indexer_name,
        profile.rank_artists.unwrap_or(0),
        indexer_id,
        indexer_name,
        profile.rank_overall.unwrap_or(0.0),
        indexer_id,
        indexer_name,
        profile.class,
        indexer_id,
        indexer_name,
        if profile.donor.unwrap_or(false) { 1 } else { 0 },
        indexer_id,
        indexer_name,
        if profile.warned.unwrap_or(false) {
            1
        } else {
            0
        },
        indexer_id,
        indexer_name,
        profile.posts.unwrap_or(0),
        indexer_id,
        indexer_name,
        profile.torrent_comments.unwrap_or(0),
        indexer_id,
        indexer_name,
        profile.collages_started.unwrap_or(0),
        indexer_id,
        indexer_name,
        profile.collages_contrib.unwrap_or(0),
        indexer_id,
        indexer_name,
        profile.requests_filled.unwrap_or(0),
        indexer_id,
        indexer_name,
        profile.requests_voted.unwrap_or(0),
        indexer_id,
        indexer_name,
        profile.uploaded_torrents.unwrap_or(0),
        indexer_id,
        indexer_name,
        profile.groups.unwrap_or(0),
        indexer_id,
        indexer_name,
        profile.seeding.unwrap_or(0),
        indexer_id,
        indexer_name,
        profile.leeching.unwrap_or(0),
        indexer_id,
        indexer_name,
        profile.snatched.unwrap_or(0),
        indexer_id,
        indexer_name,
        profile.invited.unwrap_or(0),
        indexer_id,
        indexer_name,
        profile.bonus_points.unwrap_or(0),
        indexer_id,
        indexer_name,
        profile.bonus_points_per_hour.unwrap_or(0.0),
        indexer_id,
        indexer_name,
        profile.seed_size.unwrap_or(0),
        indexer_id,
        indexer_name,
        profile.average_seed_time.unwrap_or(0)
    );

    Ok(HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .body(metrics))
}
