use crate::app::screens::main::state::TrackSort;
use crate::track::track::Track;
use std::cmp::Ordering;

/// Compute sorted indices for a list of tracks based on sort criteria
pub fn compute_sorted_indices(
    tracks: &[&Track],
    sorted_by: &Option<TrackSort>,
    sort_ascending: bool,
) -> Vec<usize> {
    let mut sorted_indices: Vec<usize> = (0..tracks.len()).collect();

    sorted_indices.sort_by(|&a, &b| {
        let track_a = tracks.get(a);
        let track_b = tracks.get(b);

        let ordering = match (track_a, track_b) {
            (Some(ta), Some(tb)) => match sorted_by {
                Some(TrackSort::Title) => ta.title.to_lowercase().cmp(&tb.title.to_lowercase()),
                Some(TrackSort::Artist) => ta.artist.to_lowercase().cmp(&tb.artist.to_lowercase()),
                Some(TrackSort::Album) => ta.album.to_lowercase().cmp(&tb.album.to_lowercase()),
                Some(TrackSort::Duration) => ta.duration.cmp(&tb.duration),
                _ => a.cmp(&b), // Keep original order if no sort
            },
            _ => Ordering::Equal,
        };

        if sort_ascending {
            ordering
        } else {
            ordering.reverse()
        }
    });

    sorted_indices
}
